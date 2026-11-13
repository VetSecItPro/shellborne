---
layout: post
date: 2026-11-12 18:05:00 -0600
title: The checks caught me, which is the point
description: I rebuilt the tool that decides what counts as done. The first thing it did was find three mistakes in my own history.
---
The review found holes in the tool that keeps Shellborne's plan honest, so this afternoon I rebuilt the worst part of it: the part that decides when a piece of work counts as done.

Until today, the tool trusted any note in my private records that said a step was finished. It never asked whether the steps before it were finished first, and it never checked whether a step that only I am allowed to close actually carried my signature. Those were the two biggest holes the review found.

The new rules are strict. A record counts only if it was written in the right order, its evidence still exists, and, for anything that is mine to approve, it is signed with a key that only I can use.

Then I ran the new rules over my own history, and they failed it.

Three records from the past two days had been written out of order: work closed before the work in front of it. Nothing was broken in the code. The old tool simply never looked. Because each record leans on the ones before it, those three pulled down nine more.

I did not rewrite history to make the check pass. The old records stay exactly where they are. I wrote fresh ones, in the right order, and signed the one that was mine. The check passed after that, and it passed honestly.

The checks caught me twice more before dinner. A test run reported success when it had failed, because of how I had piped its output, and the commit guard refused to let the code in until the tests truly passed. Then the leak guard blocked a harmless fix to this website, because my private review notes quote the public README. That turned out to be a real flaw in the guard, so it got its own fix and its own tests.

A check that never catches you is decoration. Today mine caught me three times. I'll take that.
