---
layout: post
date: 2026-11-13 09:05:00 -0600
title: Writing down the guesses
description: Every plan rests on things nobody has measured yet. Last night I made Shellborne's plan say which ones, and refuse to build on them until they are tested.
---
Every plan rests on guesses. Shellborne's rests on a handful of big ones, about how macOS treats long-running background work and how the tools it builds on behave. All of them sound reasonable. None of them has been measured on a real Mac yet.

The danger isn't in guessing. It's in forgetting you guessed. Weeks later, a dozen features are built on top, and the guess has quietly turned into a fact nobody remembers checking.

So last night I gave the plan a list of its own guesses. Each entry says three things:

- **The guess**, in one sentence.
- **The experiment** that settles it: a small, throwaway test on real hardware.
- **What rests on it:** the pieces of work that only make sense if the guess holds.

The tool that hands out work now reads that list. While a serious guess is untested, anything resting on it is held: no agent is offered it, and no change citing it gets in. When the experiment runs and its result is recorded, the hold lifts.

Two rules came out of writing this, and both surprised me a little.

**A "no" counts as settled.** The point of an experiment is to find out, not to pass. If the test shows macOS won't allow something, that's a result, and a valuable one. The work that rested on the guess gets rethought instead of built on sand. So "verified" means the experiment ran and left evidence, whichever way it came out.

**Only I can wave a guess through.** An agent can propose dropping an entry, but the drop only counts in a commit signed with my key. Otherwise the fastest way past a hold would be to delete it, and the agents doing the work would be the ones holding the eraser.

Then the check did what good checks do and looked somewhere new. It went through every security test the threat model names and asked whether each one has a place in the plan. A good share didn't. Each now has a home, so none of them can be forgotten, which is the same lesson again.
