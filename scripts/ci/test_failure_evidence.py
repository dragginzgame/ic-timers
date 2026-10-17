"""Rejection checks for the downloaded qualification evidence boundary."""

import io
from contextlib import contextmanager
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import unittest

from verify_failure_evidence import verify_archive


@contextmanager
def fixture_directory():
    directory = Path(tempfile.mkdtemp(prefix="timer-downloaded-evidence."))
    try:
        yield directory
    except BaseException:
        print(f"Failed downloaded-evidence fixture retained: {directory}", file=sys.stderr)
        raise
    else:
        shutil.rmtree(directory)


class DownloadedEvidenceTests(unittest.TestCase):
    def test_collected_early_qualification(self):
        # Exercise maintained producers as well as synthetic archive rejection.
        # This suite runs in the Ubuntu download-verification job; it adds no
        # Python requirement to native macOS or the Make release gate.
        root = Path(__file__).resolve().parents[2]
        with fixture_directory() as temporary:
            workspace, runner = temporary / "checkout", temporary / "runner"
            subprocess.run(["git", "clone", "--quiet", "--shared", "--no-checkout",
                            str(root), str(workspace)], check=True)
            for name in (
                "scripts/ci/qualify-failure-evidence.sh",
                "scripts/ci/qualify-hosted-failure.sh",
                "scripts/ci/verify-file-checksum.sh",
                "scripts/dev/install-host-tools.sh",
                "scripts/dev/tool-store.sh",
                "ci/tool-versions.env", "ci/ic-tools-qualified.tsv",
            ):
                target = workspace / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(root / name, target)
            runner.mkdir()
            sha = subprocess.check_output(
                ["git", "-C", str(workspace), "rev-parse", "HEAD"], text=True).strip()
            identity = dict(checkout_sha=sha, event_sha=sha, job="checks",
                            host="Linux/X64", run="123", attempt="2")
            env = dict(os.environ, GITHUB_WORKSPACE=str(workspace), RUNNER_TEMP=str(runner),
                       TMPDIR=str(runner), SHARED_TOOLING_STORE=str(temporary / "shared-store"),
                       GITHUB_ACTIONS="true", GITHUB_EVENT_NAME="workflow_dispatch",
                       GITHUB_SHA=sha, GITHUB_JOB="checks", RUNNER_OS="Linux", RUNNER_ARCH="X64",
                       GITHUB_RUN_ID="123", GITHUB_RUN_ATTEMPT="2")
            for command, expected, log in (
                (["bash", str(workspace / "scripts/ci/qualify-failure-evidence.sh"), "early"],
                 22, "early.log"),
                (["bash", str(root / "scripts/ci/collect-failure-evidence.sh")],
                 0, "collection.log"),
            ):
                result = subprocess.run(command, env=env, capture_output=True)
                (temporary / log).write_bytes(result.stdout + result.stderr)
                self.assertEqual(result.returncode, expected, (result.stdout, result.stderr))
            tool_pins = {kind: (root / "ci" / name).read_bytes() for kind, name in
                         (("host", "tool-versions.env"), ("ic", "ic-tools-qualified.tsv"))}
            verify_archive(runner / "ic-timers-failure-evidence.tar.gz", "early", identity, tool_pins)

    def test_qualification_and_rejections(self):
        identity = dict(checkout_sha="selected", event_sha="selected", job="macos",
                        host="macOS/X64", run="123", attempt="2")
        ic_pins = "".join(
            f"{tool}\t{version}\t{host}\t{'0' * 64}\n"
            for host in ("linux-x86_64", "darwin-x86_64", "darwin-arm64")
            for tool, version in (("quill", "0.5.4"), ("icp", "1.6.0"),
                                  ("didc", "0.6.2"), ("ic-wasm", "0.11.1"),
                                  ("wasm-opt", "132"))
        ).encode()
        tool_pins = {"host": b"selected host pins\n", "ic": ic_pins}
        for stage in ("early", "late"):
            scenario = f"ic-timers-fixtures/hosted-{stage}.ABC123"
            marker = (b"error: controlled early installer download failure\n" if stage == "early"
                      else b"error: controlled late validation failure\n")
            status = 22 if stage == "early" else 2
            files = {
                "identity.txt": ("".join(f"{key}={value}\n" for key, value in identity.items()).encode(), 0o644),
                scenario + "/status.txt": (f"stage={stage}\nstatus={status}\nexpected={status}\n".encode(), 0o644),
                scenario + "/scenario.log": (marker, 0o644),
            }
            for name in ("before.txt", "after.txt", "consumer/input.txt"):
                files[scenario + "/" + name] = (f"controlled {stage} input\n".encode(), 0o640)
            if stage == "early":
                payload = scenario + "/tool-store/host-" + "0" * 64 + "/set.ABC123/bin/jq"
                files[payload] = (b"controlled rejected download bytes\n", 0o644)
            else:
                payload = "target/validation-failures/latest-combined.log"
                files[payload] = (b"batch\n" + marker, 0o644)
                files["target/validation-failures/latest.log"] = (marker, 0o644)
                for kind in ("host", "ic"):
                    prefix = f"tool-evidence/{kind}/"
                    files[prefix + "caller-pins"] = (tool_pins[kind], 0o644)
                    files[prefix + "check.log"] = (b"fresh verification output\n", 0o644)
                    files[prefix + "selection.txt"] = (f"/native/checkout/.tools/{kind}-set.ABC123\n".encode(), 0o644)
                files["tool-evidence/ic/pins.tsv"] = (tool_pins["ic"], 0o644)
                files["tool-evidence/ic/host"] = (b"darwin-x86_64\n", 0o644)
                files["tool-evidence/ic/files.sha256"] = (b"retained checksum receipt\n", 0o644)
            with fixture_directory() as temporary:
                archive = temporary / "evidence.tar.gz"

                def write(entries):
                    with tarfile.open(archive, "w:gz") as output:
                        for name, (data, mode) in entries.items():
                            member = tarfile.TarInfo(name)
                            member.size, member.mode = len(data), mode
                            output.addfile(member, io.BytesIO(data))

                write(files)
                verify_archive(archive, stage, identity, tool_pins)
                if stage == "late":
                    # Exercise the actual CLI's checkout selection: the shared
                    # default intentionally differs from our qualified exception.
                    catalogs = temporary / "ci"
                    catalogs.mkdir()
                    (catalogs / "tool-versions.env").write_bytes(tool_pins["host"])
                    qualified = catalogs / "ic-tools-qualified.tsv"
                    qualified.write_bytes(ic_pins)
                    default = ic_pins.replace(b"\t132\t", b"\t133\t")
                    (catalogs / "ic-tools.tsv").write_bytes(default)
                    env = dict(os.environ, GITHUB_WORKSPACE=str(temporary),
                               GITHUB_SHA=identity["checkout_sha"], GITHUB_RUN_ID=identity["run"],
                               GITHUB_RUN_ATTEMPT=identity["attempt"])
                    command = [sys.executable, str(Path(__file__).with_name("verify_failure_evidence.py")),
                               str(archive), stage, identity["job"], identity["host"]]
                    result = subprocess.run(command, env=env, capture_output=True)
                    self.assertEqual(result.returncode, 0, (result.stdout, result.stderr))
                    for wrong in (default, ic_pins.replace(b"0" * 64, b"1" * 64, 1)):
                        qualified.write_bytes(wrong)
                        result = subprocess.run(command, env=env, capture_output=True)
                        self.assertNotEqual(result.returncode, 0)
                        self.assertIn(b"different caller pins", result.stderr)
                    qualified.unlink()
                    result = subprocess.run(command, env=env, capture_output=True)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(b"ic-tools-qualified.tsv", result.stderr)
                # Shared owns scenario-format rejection permutations. Keep the
                # collector integration, identity, pin selection and a corrupt
                # status case here to prove the caller actually delegates.
                write(files)
                with self.assertRaises(ValueError):
                    verify_archive(archive, "late" if stage == "early" else "early", identity, tool_pins)
                for key in identity:
                    with self.assertRaises(ValueError):
                        verify_archive(archive, stage, dict(identity, **{key: "different"}), tool_pins)
                # Owner fixtures cover generic archive/receipt permutations.
                # These counterexamples prove this consumer actually delegates.
                mutations = [(scenario + "/status.txt", (b"status=0\n", 0o644)),
                             ("../outside", (b"unsafe", 0o644))]
                if stage == "late":
                    mutations.append(("tool-evidence/ic/host", (b"darwin-arm64\n", 0o644)))
                for name, replacement in mutations:
                    write(dict(files, **{name: replacement}))
                    with self.assertRaises(ValueError):
                        verify_archive(archive, stage, identity, tool_pins)
                write({key: value for key, value in files.items() if key != "identity.txt"})
                with self.assertRaises(ValueError):
                    verify_archive(archive, stage, identity, tool_pins)


if __name__ == "__main__":
    unittest.main()
