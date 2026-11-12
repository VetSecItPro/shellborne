---
layout: post
date: 2026-11-12 13:00:00 -0600
title: The first real code is a set of walls
description: Shellborne's first Rust and Swift code does almost nothing yet. What it does is decide, for good, which parts are allowed to talk to which.
---
Shellborne finally has real code: a Rust workspace for the parts that keep your sessions alive, and a Swift package for everything with a window. It builds cleanly with every warning treated as an error, and it is in review now; it goes into the public repository with the next merge. And it does almost nothing yet, on purpose.

What it does is draw the walls. The app is split into five layers, stacked like floors in a building:

1. **The engine adapters**, the only code allowed to touch the terminal engine.
2. **The session daemon**, which owns every running session, so the app can quit, crash or update without taking your agents with it.
3. **The core**: sessions, workspaces, settings, commands.
4. **Features**, each one plugged into the core and nothing else.
5. **The Mac app shell** itself.

Each layer may lean only on the layers below it, and two features may never reach into each other. Those rules sit in one short list, and a check compares the list with the real dependencies in both languages, plus every import line in the Swift code, on every change.

The rule I care about most is the one that keeps the core free of the Mac's windowing code. It is what keeps an iPhone version possible later without a rewrite. To prove the check works, I added one line to the core that pulled in the Mac windowing code. The check refused it immediately. I removed the line and it passed.

Writing walls before rooms feels slow. But this is the cheapest moment there will ever be to set them, and an AI agent working fast is exactly the kind of builder that would otherwise wander through one without noticing.

Next: the actual rooms, starting with how a session survives the app.
