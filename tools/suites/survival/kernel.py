"""The one place the survival suites reach the session kernel (Plan-Step: P2.1.2).

The suites never start, find or kill a kernel process any other way, so when the kernel's interface is designed
(Plan-Step: P2.4) only this file changes. Until then available() names what is missing and every suite fails on it:
a suite that skips when the kernel is absent would read as a pass.

Roles, as the PRD's terms define them:
  gui         the app; it owns no PTY
  attach      the disposable filter between one session and a view
  vt          the per-session parser process
  supervisor  the per-user service holding the lock and the control socket
  host        the per-session process that owns the PTY and the byte ring
  shell       the program running on the PTY (in the suites, a `survival tick` logger)
"""
import pathlib

ROLES = ("gui", "attach", "vt", "supervisor", "host", "shell")
KILLABLE = ("gui", "attach", "vt", "supervisor")


class KernelAbsent(Exception):
    pass


class Kernel:
    """The kernel as the suites see it. Every method raises KernelAbsent until Plan-Step P2.4 wires it."""

    MISSING = "the session kernel is not built yet (Plan-Step: P2.4), so there is nothing to start or kill"

    def __init__(self, workdir):
        self.workdir = pathlib.Path(workdir)

    def available(self):
        """None when the kernel can run here, otherwise one sentence saying what is missing."""
        return self.MISSING

    def start(self, panes, command):
        """Start `panes` sessions, each running `command` (a list), and wait until each is attached to a view."""
        raise KernelAbsent(self.MISSING)

    def pids(self):
        """{role: [pid, ...]} for every live process of every role, from the kernel's own process table."""
        raise KernelAbsent(self.MISSING)

    def kill(self, role):
        """SIGKILL every process of one killable role, as a crash would."""
        raise KernelAbsent(self.MISSING)

    def wait_attached(self, seconds):
        """Seconds until every pane is attached to a view again, or None past the limit."""
        raise KernelAbsent(self.MISSING)

    def pane_logs(self):
        """The path each pane's output is captured to, one per pane."""
        raise KernelAbsent(self.MISSING)

    def stop(self):
        """End every session and process this kernel started."""
        return None
