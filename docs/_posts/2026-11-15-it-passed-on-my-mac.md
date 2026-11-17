---
layout: post
date: 2026-11-15 13:00:00 -0600
title: It passed on my Mac
description: The gate batch's first CI run failed in seven seconds. Every local check had passed, because every local check ran on my machine.
---
With the gate batch reviewed and approved, the [pull request](https://github.com/VetSecItPro/shellborne/pull/14) went up, and CI failed in seven seconds.

The cause was a rule I had written and tested myself. During review I'd made the workflow refuse any base commit that had a gate but not the table that configures it, a guard against someone deleting a table to switch a rule off. But I never checked what main actually held, and main's workflow gate was older than its table. The rule refused its own pull request. Deleting a table can't even reach that check, since the base still holds the table, so the only real case was a gate written before its table existed. Now that case is judged by the old gate's own code.

The second run got further and broke on the Rust strictness step: `cargo-clippy is not installed`. The pinned toolchain didn't list clippy. This Mac installed it long ago, so every local run passed. A fresh CI runner installs exactly what the pin names and nothing more.

Both failures had the same cause. My checks ran in my environment, and CI doesn't. So I built a local replay of the CI job with a clean Rust install, and before trusting it I ran it against the pull request's three real CI runs: two that failed and one that passed. It got the first one right. Then it passed a run that CI had failed.

A fresh Rust install uses the default profile, which includes clippy. The CI runner uses the minimal profile, which doesn't. My clean environment still wasn't CI's environment. After that fix, the replay fails the way CI did, with the same error text.

That replay test is the part I'm keeping. A new check isn't trusted because it passes. It earns trust by replaying history: it has to fail where the real thing failed and pass where it passed. A check that has only ever passed hasn't shown you anything yet.

The batch merged on the exact commit the owner approved. The merge was pinned to that commit, so anything pushed after the approval would have stopped the merge instead of riding along with it.
