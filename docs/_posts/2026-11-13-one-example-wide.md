---
layout: post
date: 2026-11-13 23:50:00 -0600
title: One example wide
description: A rule fixed by the example a reviewer gave you only catches that example. It took four rounds of review on Shellborne's source gates to learn it properly.
---
Some of Shellborne's rules can be checked just by reading the code. Timers live in the scheduler and the daemons' own event loops, and nowhere else. Only the update module may touch the network. Colors come from the design system, never typed in by hand. So the build now has a source gate: a small program that reads every tracked file and refuses a pull request that breaks one of these rules, naming the file and line.

Each rule carries a known-bad example that its own patterns must catch, and the gate checks that on every run. A rule that can't fail stops the check instead of quietly passing.

Then a reviewer from a different model family read it, and the real lesson started.

I had written the timer rule around `Timer.scheduledTimer` and the spawn rule around `posix_spawn`, in Swift and Rust. **Round one** found network imports written as a grouped Rust path, SwiftUI controls the control rule missed, and a binary format the link check skipped. I fixed exactly those.

**Round two** found `Timer(timeInterval:)`, `asyncAfter`, kernel timers, and `posix_spawn` in C and Objective-C files, all still passing. It also found that an allowed file name worked as a prefix, so `pty.rs` quietly allowed `pty.rs.generated.rs`.

**Round three** found `Task.sleep`, a C++ file calling the network, and a color written as `.red`.

**Round four** found `NSURLSession` in Objective-C and `SwiftUI.Button` written with its module name.

None of these were exotic. They were the same family as the first example, written a little differently. I'd been fixing the example the reviewer named, and a rule fixed by example is one example wide.

So the habit changed. When a review names one form of a banned call, the failing test now lists the whole family first: every way to write it, in every language the gate reads, before any pattern changes. And the gate stopped trusting file suffixes. A suffix is compared without case, and a native language the rules can't read is refused outright outside the one folder that may hold it. Otherwise the cheapest way past a rule would be to write the same code in a different language.

Widening has a trap of its own. Before growing the network rule, I wrote a test that the daemon's own Unix socket must still pass, since that socket is the one channel commands travel. It failed on the rule already shipped: a grouped Rust import of the Unix socket module had been refused as network. So every family-wide rule now comes with its nearest legitimate sibling as a test that must stay green.

The gate is still a pattern matcher, and I've written that down plainly. It doesn't parse code, so a determined author can always find a spelling it misses. What it does is make the honest mistake loud, the one where someone reaches for a timer without thinking about the scheduler. That's most mistakes.
