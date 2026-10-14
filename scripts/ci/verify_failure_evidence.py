#!/usr/bin/env python3
"""Check downloaded manual-qualification archives without extracting them."""

import os
import re
import sys
import tarfile

from failure_evidence import regular_evidence_archive, verify_compact_tools


def verify_archive(path, stage, identity, tool_pins):
    if stage not in ("early", "late"):
        raise ValueError("expected early or late qualification")
    with regular_evidence_archive(path) as (members, read):
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
                                          + r"/tool-store/host-[0-9a-f]{64}/set\.[A-Za-z0-9]+/bin/jq", name)]
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
            verify_compact_tools(members, read, tool_pins, identity["host"])


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
        for kind, name in (("host", "tool-versions.env"), ("ic", "ic-tools-qualified.tsv")):
            with open(os.path.join(workspace, "ci", name), "rb") as pins:
                tool_pins[kind] = pins.read()
        verify_archive(sys.argv[1], sys.argv[2], identity, tool_pins)
    except (OSError, ValueError, tarfile.TarError) as error:
        sys.exit(f"failure artifact qualification refused: {error}")
    print(f"Downloaded {sys.argv[2]} failure evidence qualified for {sys.argv[3]} {sys.argv[4]}")
