---
layout: default
title: Intel support policy
description: How long Shellborne runs on Intel Macs, what holds until then, and what ends it.
---

# Intel support policy

## What we expect

Today Shellborne is built for both Apple silicon Macs and Intel Macs. Our working assumption is that Intel builds
stop once macOS 26 no longer gets security coverage from Apple. Treat that as a forecast, not a promise; only the
conditions listed further down can end Intel support, never a calendar.

## What holds until then

- Each release ships universal: every binary inside it has one slice for arm64 and one for x86_64.
- Both kinds of Mac get builds aimed at macOS 26.
- An Intel iMac remains one of the release gate machines, and a release it fails is held back.
- No optimization is made for Intel alone. Code that runs only on x86_64 carries a comment saying why.

## What ends Intel support

Intel support ends when evidence says so. Any single one of these is enough:

- macOS 26 has lost its security coverage from Apple.
- Our pinned toolchain cannot build an x86_64 slice anymore, or cannot test one.
- We have no Intel machine left that can serve as the release gate.

## What you are told

If that happens, you are told before the last Intel release ships, never by that release itself. The notes of the
release before it, and this page, name the condition that was met and say which coming release will be the final one
for Intel Macs. That final release keeps running unchanged on any Intel Mac where it is installed. Updates for it stop,
and none of its features are taken away.

tools/gates/term-lint checks this page and refuses wording about calendar dates or lengths of time (Plan-Step
P0.12.5).
