---
layout: post
date: 2026-11-12 18:00:00 -0600
title: A fresh pair of eyes on the whole plan
description: A new model came out today, so I handed it the entire build plan and asked it to find what I had missed. It found the one thing that would have stopped everything.
---
Shellborne is built from a plan. Every piece of work is written down before anyone starts it: what to build, in what order, and what proves it is finished. Agents follow it step by step, and a small tool refuses any work that is out of turn. After a few days of building that way, the plan felt solid. Of course it did. I had been staring at it the whole time.

A new Claude model came out this morning, so I gave it the whole thing and one instruction: assume it has holes, and find them.

It did not read it as one reviewer. I split the job six ways. One reader asked whether each step could actually prove what it claimed. Another read only for security. Another hunted for the ways a terminal fails in real life: the laptop going to sleep with agents running, a full disk, an update landing in the middle of someone's work. Then six more readers checked the first six, line by line, against the documents. A finding only stayed if it held up twice.

Most of what they found was the slow, useful kind: a test that could never fail, a promise with no step behind it, two documents quietly disagreeing about the same thing. Dozens of those.

One finding was different. Some steps only happen if an early experiment goes a certain way. If it goes the other way, the step is skipped. But the tool treated a skipped step as unfinished work, forever, and the plan's most likely path ran straight through one. A few weeks from now the build would have reached that point and stopped cold, with every remaining step waiting on something that was never going to happen.

That is fixed now, along with the rest. Nothing went in on the reviewers' say-so. I answered every decision myself, and every change to the plan carries my signature.

The lesson I keep relearning: the author is the worst reviewer. Even when the author is mostly an AI.
