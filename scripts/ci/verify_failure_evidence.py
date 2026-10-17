#!/usr/bin/env python3
"""Check downloaded manual-qualification archives without extracting them."""

import os
import re
import sys
import tarfile

from failure_evidence import (regular_evidence_archive, verify_hosted_scenario)


def verify_archive(path, stage, identity, tool_pins):
    with regular_evidence_archive(path) as (members, read):
        expected_identity = "".join(f"{key}={value}\n" for key, value in identity.items())
        if read("identity.txt") != expected_identity.encode():
            raise ValueError("archive does not identify the selected source/job/host/run/attempt")
        verify_hosted_scenario(members, read, stage,
                               scenario_parent="ic-timers-fixtures",
                               failure_log_root="target/validation-failures")
        if stage == "late":
            # Late qualification follows successful complete native setup.
            # Require compact selection and caller pins in the downloaded bytes;
            # ordinary failures can still retain actively failed sets in full.
            # Timers selects only the host bundle. IC installer/receipt schemas
            # remain upstream; old unselected IC evidence may still be retained.
            prefix = "tool-evidence/host/"
            if read(prefix + "caller-pins") != tool_pins["host"]:
                raise ValueError("compact evidence uses different caller pins")
            if not read(prefix + "check.log"):
                raise ValueError("compact host evidence lost its fresh check log")
            selection = re.fullmatch(rb".+/\.tools/(host-set\.[A-Za-z0-9]+)\n",
                                     read(prefix + "selection.txt"), re.DOTALL)
            if selection is None:
                raise ValueError("compact host evidence lost its selection identity")
            active = ".tools/" + selection[1].decode()
            if any(name == active or name.startswith(active + "/") for name in members):
                raise ValueError("verified host payload was not compacted")


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
        for kind, name in (("host", "tool-versions.env"),):
            with open(os.path.join(workspace, "ci", name), "rb") as pins:
                tool_pins[kind] = pins.read()
        verify_archive(sys.argv[1], sys.argv[2], identity, tool_pins)
    except (OSError, ValueError, tarfile.TarError) as error:
        sys.exit(f"failure artifact qualification refused: {error}")
    print(f"Downloaded {sys.argv[2]} failure evidence qualified for {sys.argv[3]} {sys.argv[4]}")
