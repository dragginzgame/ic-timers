#!/usr/bin/env python3
"""Check downloaded manual-qualification archives without extracting them."""

import os
import re
import subprocess
import sys
import tarfile


def verify_archive(path, stage, identity, tool_pins):
    if stage not in ("early", "late"):
        raise ValueError("expected early or late qualification")
    with tarfile.open(path, "r:gz") as archive:
        members = {}
        for member in archive:
            name = member.name.rstrip("/")
            # The shared tar caller emits one conventional relative prefix.
            # Normalize it before duplicate admission; aliases must still fail.
            if name.startswith("./"):
                name = name[2:]
            if (not name or name.startswith("/")
                    or any(part in ("", ".", "..") for part in name.split("/"))
                    or name in members):
                raise ValueError("ambiguous or unsafe archive member")
            members[name] = member

        def read(name, mode=None):
            member = members.get(name)
            if member is None or not member.isfile():
                raise ValueError(f"missing regular evidence file: {name}")
            if mode is not None and member.mode != mode:
                raise ValueError(f"incorrect retained mode: {name}")
            with archive.extractfile(member) as source:
                return source.read()

        expected_identity = "".join(f"{key}={value}\n" for key, value in identity.items())
        if read("identity.txt") != expected_identity.encode():
            raise ValueError("archive does not identify the selected source/job/host/run/attempt")
        scenarios = [name.rsplit("/", 1)[0] for name in members
                     if re.fullmatch(r"ic-timers-fixtures/hosted-" + stage
                                     + r"\.[A-Za-z0-9]+/status\.txt", name)]
        if len(scenarios) != 1:
            raise ValueError("expected exactly one selected qualification scenario")
        scenario = scenarios[0]
        status = 22 if stage == "early" else 2
        expected_status = f"stage={stage}\nstatus={status}\nexpected={status}\n".encode()
        if read(scenario + "/status.txt") != expected_status:
            raise ValueError("controlled failure status was not retained")
        expected_input = f"controlled {stage} input\n".encode()
        for name in ("before.txt", "after.txt", "consumer/input.txt"):
            if read(scenario + "/" + name, 0o640) != expected_input:
                raise ValueError("scenario input bytes were not preserved")
        marker = (b"error: controlled early installer download failure" if stage == "early"
                  else b"error: controlled late validation failure")
        # The logger annotates its live diagnostics; retained raw logs below
        # keep the original complete lines.
        if marker not in read(scenario + "/scenario.log"):
            raise ValueError("scenario did not reach the controlled failure")
        if stage == "early":
            candidates = [name for name in members
                          if re.fullmatch(re.escape(scenario)
                                          + r"/consumer/\.tools/host-set\.[^/]+/bin/jq", name)]
            if len(candidates) != 1 or read(candidates[0]) != b"controlled rejected download bytes\n":
                raise ValueError("rejected installer candidate bytes were not retained")
        else:
            prefix = "target/validation-failures/"
            raw = read(prefix + "latest.log")
            if marker not in raw.splitlines() or raw not in read(prefix + "latest-combined.log"):
                raise ValueError("raw/combined validation logs were not retained")
            # Late qualification follows successful complete native setup.
            # Require compact selection and caller pins in the downloaded bytes;
            # ordinary failures can still retain actively failed sets in full.
            for kind in ("host", "ic"):
                prefix = f"tool-evidence/{kind}/"
                if read(prefix + "caller-pins") != tool_pins[kind]:
                    raise ValueError("compact evidence uses different caller pins")
                if not read(prefix + "check.log"):
                    raise ValueError("compact evidence lost its fresh check log")
                selected = re.fullmatch(rb".+/\.tools/(" + kind.encode()
                                        + rb"-set\.[A-Za-z0-9]+)\n",
                                        read(prefix + "selection.txt"), re.DOTALL)
                if selected is None:
                    raise ValueError("compact evidence lost its managed selection identity")
                active = ".tools/" + selected[1].decode()
                if any(name == active or name.startswith(active + "/") for name in members):
                    raise ValueError("verified active tool payload was not compacted")
            # The shared installer admits validated records, preserving original
            # receipt bytes across comment/order-only caller changes. Use that
            # same parser rather than inventing a second pin admission schema.
            records = []
            for pins in (read("tool-evidence/ic/pins.tsv"), tool_pins["ic"]):
                parsed = subprocess.run(
                    ["awk", "-v", "records=1", "-f",
                     os.path.join(os.path.dirname(__file__), "ic-tool-pins.awk")],
                    input=pins, capture_output=True, check=False,
                )
                if parsed.returncode != 0:
                    raise ValueError("compact IC receipt has an invalid pin selection")
                records.append(sorted(parsed.stdout.splitlines()))
            if records[0] != records[1]:
                raise ValueError("compact IC receipt uses different caller pins")
            host = {"Linux/X64": b"linux-x86_64\n", "macOS/X64": b"darwin-x86_64\n",
                    "macOS/ARM64": b"darwin-arm64\n"}.get(identity["host"])
            if host is None or read("tool-evidence/ic/host") != host:
                raise ValueError("compact IC receipt identifies a different native host")
            if not read("tool-evidence/ic/files.sha256"):
                raise ValueError("compact IC checksum receipt is empty")


if __name__ == "__main__":
    if len(sys.argv) != 5:
        sys.exit("usage: verify_failure_evidence.py ARCHIVE early|late JOB OS/ARCH")
    identity = {
        "checkout_sha": os.environ["GITHUB_SHA"],
        "event_sha": os.environ["GITHUB_SHA"],
        "job": sys.argv[3],
        "host": sys.argv[4],
        "run": os.environ["GITHUB_RUN_ID"],
        "attempt": os.environ["GITHUB_RUN_ATTEMPT"],
    }
    try:
        workspace = os.environ["GITHUB_WORKSPACE"]
        tool_pins = {}
        for kind, name in (("host", "tool-versions.env"), ("ic", "ic-tools.tsv")):
            with open(os.path.join(workspace, "ci", name), "rb") as pins:
                tool_pins[kind] = pins.read()
        verify_archive(sys.argv[1], sys.argv[2], identity, tool_pins)
    except (OSError, ValueError, tarfile.TarError) as error:
        sys.exit(f"failure artifact qualification refused: {error}")
    print(f"Downloaded {sys.argv[2]} failure evidence qualified for {sys.argv[3]} {sys.argv[4]}")
