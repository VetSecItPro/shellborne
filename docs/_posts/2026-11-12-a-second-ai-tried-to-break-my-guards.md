---
layout: post
date: 2026-11-12 09:00:00 -0600
title: I asked a second AI to break my guards
description: One AI writes most of Shellborne's code. A different one gets paid, in effect, to find what the first one got wrong. This week it found plenty.
---
Most of Shellborne's code is written by one AI agent. So before anything risky merges, a different AI model reads the change cold, with no explanation from the author, and tries to find a way around it. This week I pointed it at the guards themselves: the checks that decide whether a change is allowed in at all.

It found ten real problems. A few of them were the kind that keep me up at night:

- **A pull request could rewrite the check that judges it.** The automated checks on GitHub run the pull request's own copy of the workflow. Change the workflow to "always pass" and it would. Approval by me was the only thing in the way.
- **The order rule could be gamed.** Write the code first, add the test afterwards, and the gate called it test-first.
- **A blank form passed.** My own template for the private review notes satisfied the check that was supposed to make me fill it in.

Every one of them got a test that failed first, then a fix. The biggest change is where trust lives. The checks still run on GitHub, but before a change can merge, my Mac runs its own private, trusted copy of every check against the exact files in that pull request and signs the result. GitHub then verifies that signature using a copy of the checker that the pull request cannot touch. Rewriting the public check no longer gets you anywhere.

The reviewer and I disagreed twice. The one that mattered: it wanted every test added after code to be refused. That would also block the normal rhythm of test, code, test, code inside one batch of work, so I said no and wrote down why. It read the reason and agreed.

Then I found two bugs of my own when the new system ran for real for the first time, one of which would have rejected every single review note I write. The checks are only as good as the first honest run.

None of this is visible in the app yet. It is what lets an AI write most of the code without me reading every line.
