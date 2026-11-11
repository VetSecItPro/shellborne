---
layout: post
date: 2026-11-11 09:00:00 -0600
title: Why I'm building a terminal for AI agents
description: My day now runs on AI coding agents. The window they live in should never lose their work.
---
Most of my coding day now happens through AI agents: Claude Code in one pane, Codex in another, often several sessions going at once. They read the code, run the tests, and ask me before they do anything risky.

The weak spot is not the agents. It is the window they run in.

Close the terminal by accident and a session dies mid-task. An update restarts the app and takes three agents down with it. One runaway pane hogs the machine and the others stall. And when I glance across six tabs, I often cannot tell which agent is working, which one is waiting on me, and which one quietly stopped an hour ago.

So I'm building Shellborne: a native Mac terminal made for this way of working.

The idea is simple to say and hard to get right:

- **Sessions outlive the app.** If Shellborne quits, crashes or updates, your agents keep running and you pick up where you were.
- **One bad pane stays one bad pane.** A misbehaving session cannot drag the rest down.
- **Status you can trust.** When it does not know what a session is doing, it says so instead of guessing.
- **Nothing leaves your Mac unless you choose.** No account, no sign-in, no analytics.

It will be free and open source. The code is public on GitHub from the very first commit, and a lot of it is written by the same kind of AI agents it is built for. That part is not hidden: every change is tested first and approved by me before it goes in.

There is nothing to download yet. Right now I'm laying the foundation, and this devlog is where I'll share how it goes: the decisions, the mistakes, and the numbers. Follow along with the RSS feed, or watch for posts on LinkedIn.
