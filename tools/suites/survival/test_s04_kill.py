"""S04KillSuite: a GUI crash or quit never closes a PTY (Plan-Step: P2.1.2).

Run it from tools/, only through the account guard: harness/run-suite --kind kill -- python3 -m unittest suites.survival.test_s04_kill

With 10 panes each running a `survival tick` logger, it kills each of the GUI, the attach clients, the vt workers and
the supervisor 20 times. After every kill the session hosts and shells must be the same processes (same PID, same
start time, checked by tools/survival), every pane must be attached again within 3 s, and no pane's log may show a
missing tick or a pause over 2 s. The session host is not killed here: the kill matrix defines that outcome and
KillMatrixSuite tests it. The numbers are proposed until baselined.

Until the kernel exists (Plan-Step: P2.4) the suite fails, naming what is missing; it never skips.
"""
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import time
import unittest

from suites.survival.kernel import KILLABLE, Kernel

ROOT = pathlib.Path(__file__).resolve().parents[3]
SURVIVAL = ROOT / "tools" / "survival" / "survival"
PANES = 10
CYCLES = 20
REATTACH_SECONDS = 3.0
MAX_GAP_SECONDS = 2.0
TICK_EVERY = 0.1


def survival(*args):
    return subprocess.run([sys.executable, "-I", str(SURVIVAL), *map(str, args)], capture_output=True, text=True,
                          timeout=120, check=False)


class S04KillSuite(unittest.TestCase):
    def setUp(self):
        self.tmp = pathlib.Path(tempfile.mkdtemp(prefix="s04-"))
        self.kernel = Kernel(self.tmp)
        missing = self.kernel.available()
        if missing:
            self.fail(missing)
        self.addCleanup(self.kernel.stop)
        self.kernel.start(PANES, [sys.executable, "-I", str(SURVIVAL), "tick", "--every", str(TICK_EVERY)])

    def survivors(self, path):
        table = self.kernel.pids()
        pairs = [f"{role}{i}={pid}" for role in ("host", "shell") for i, pid in enumerate(table[role])]
        self.assertEqual(len(pairs), 2 * PANES, f"expected {PANES} hosts and shells, saw {table}")
        done = survival("snapshot", "--out", path, *pairs)
        self.assertEqual(done.returncode, 0, done.stdout + done.stderr)

    def kill_cycles(self, role):
        before = self.tmp / f"{role}-before.json"
        self.survivors(before)
        for cycle in range(CYCLES):
            self.kernel.kill(role)
            took = self.kernel.wait_attached(REATTACH_SECONDS)
            self.assertIsNotNone(took, f"{role} kill {cycle + 1}: panes not attached again within {REATTACH_SECONDS} s")
            after = self.tmp / f"{role}-after-{cycle}.json"
            self.survivors(after)
            held = survival("compare", before, after)
            self.assertEqual(held.returncode, 0, f"{role} kill {cycle + 1}: {held.stdout}{held.stderr}")
        for log in self.kernel.pane_logs():
            gaps = survival("gaps", log, "--max-gap", MAX_GAP_SECONDS)
            self.assertEqual(gaps.returncode, 0, f"{role}: {log}: {gaps.stdout}{gaps.stderr}")

    def test_gui(self):
        self.kill_cycles("gui")

    def test_attach_client(self):
        self.kill_cycles("attach")

    def test_vt_worker(self):
        self.kill_cycles("vt")

    def test_supervisor(self):
        self.kill_cycles("supervisor")


class S04Coverage(unittest.TestCase):
    """Runs without the kernel: every killable role has its kill case, so a deleted case fails here."""

    def test_every_killable_role_has_a_case(self):
        cases = {name[len("test_"):] for name in dir(S04KillSuite) if name.startswith("test_")}
        named = {"gui": "gui", "attach": "attach_client", "vt": "vt_worker", "supervisor": "supervisor"}
        self.assertEqual({named[role] for role in KILLABLE} - cases, set())


if __name__ == "__main__":
    unittest.main()
