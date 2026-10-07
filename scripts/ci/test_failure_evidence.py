"""Rejection checks for the downloaded qualification evidence boundary."""

import io
from contextlib import contextmanager
from pathlib import Path
import shutil
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
    def test_qualification_and_rejections(self):
        identity = dict(checkout_sha="selected", event_sha="selected", job="macos",
                        host="macOS/X64", run="123", attempt="2")
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
                payload = scenario + "/consumer/.tools/host-set.ABC123/bin/jq"
                files[payload] = (b"controlled rejected download bytes\n", 0o644)
            else:
                payload = "target/validation-failures/latest-combined.log"
                files[payload] = (b"batch\n" + marker, 0o644)
                files["target/validation-failures/latest.log"] = (marker, 0o644)
            with fixture_directory() as temporary:
                archive = temporary / "evidence.tar.gz"

                def write(entries, extra=None):
                    with tarfile.open(archive, "w:gz") as output:
                        for name, (data, mode) in entries.items():
                            member = tarfile.TarInfo(name)
                            member.size, member.mode = len(data), mode
                            output.addfile(member, io.BytesIO(data))
                        if extra is not None:
                            output.addfile(extra, io.BytesIO(b""))

                write(files)
                verify_archive(archive, stage, identity)
                with self.assertRaises(ValueError):
                    verify_archive(archive, "late" if stage == "early" else "early", identity)
                for key in identity:
                    with self.assertRaises(ValueError):
                        verify_archive(archive, stage, dict(identity, **{key: "different"}))
                mutations = [
                    (scenario + "/after.txt", (b"changed\n", 0o640)),
                    (scenario + "/before.txt", (f"controlled {stage} input\n".encode(), 0o644)),
                    (scenario + "/status.txt", (b"status=0\n", 0o644)),
                    (scenario + "/scenario.log", (b"unrelated setup outage\n", 0o644)),
                    (payload, (b"damaged retained bytes\n", 0o644)),
                    ("../outside", (b"unsafe", 0o644)),
                ]
                for name, replacement in mutations:
                    write(dict(files, **{name: replacement}))
                    with self.assertRaises(ValueError):
                        verify_archive(archive, stage, identity)
                for name in files:
                    write({key: value for key, value in files.items() if key != name})
                    with self.assertRaises(ValueError):
                        verify_archive(archive, stage, identity)
                duplicate = tarfile.TarInfo("identity.txt")
                write(files, duplicate)
                with self.assertRaises(ValueError):
                    verify_archive(archive, stage, identity)
                link = tarfile.TarInfo(scenario + "/after.txt")
                link.type, link.linkname = tarfile.SYMTYPE, "before.txt"
                write({key: value for key, value in files.items() if key != link.name}, link)
                with self.assertRaises(ValueError):
                    verify_archive(archive, stage, identity)


if __name__ == "__main__":
    unittest.main()
