---
layout: post
date: 2026-11-17 09:00:00 -0600
title: Stop reading the shell
description: For three days a gate tried to prove, by reading build scripts, that every downloaded file was checked before use. It couldn't. Now there's one way to download, and the gate refuses every other.
---
Shellborne's build downloads a few tools, like a secret scanner and a workflow linter, and each one has to match a known checksum before it runs. The rule is easy to say. Enforcing it turned into the longest fight of the week.

The first version of the gate read the build scripts and workflows and tried to follow each download: find where the file lands, find the checksum command, and make sure the check comes before any use. Every review round found another way to write a script that the gate misread. A download pushed into the background. A command split across two lines with a backslash. A checksum on the same line as the use, so the line looked checked when the file wasn't. A script sourced from somewhere else that quietly redefined `echo`. I fixed each one the way I'd learned to, with the whole family as tests and not just the example. They kept coming.

On Monday I stopped fixing and started testing. Before touching any code, I wrote down every way I could think of to get an unchecked file past the gate. The gate on main accepted more than thirty of them, in nine different families:

- A check whose failure doesn't stop the script: `|| true`, a missing `set -e`, a pipe that hides the exit code.
- Text order that isn't run order: functions, loops, Makefiles.
- Quoting and line joins that the gate's reader got wrong.

It also refused some correct steps, and it crashed on a single non-ASCII byte.

That was the answer. You can't prove what a shell script does by reading it with patterns. The shell is too flexible, and every fix just moved the edge.

So the design changed. There's now [one way](https://github.com/VetSecItPro/shellborne/pull/18) to download a file in Shellborne's build: a small helper that takes a URL, the expected checksum and a destination. It downloads into a temporary file, hashes it in its own code, and renames it into place only once the hash matches, so the final name never holds an unverified file. The gate stopped trying to understand scripts. It refuses every download command it can find, from curl and `git clone` to package installs and network libraries, in workflows, scripts, Makefiles and Dockerfiles alike, unless the call goes through the helper. That's a rule a pattern matcher can actually enforce.

Then the helper got the same outside review as everything else, and the review earned its keep. A redirect could lead somewhere the first URL check would have refused. A file already sitting at the destination was trusted by its name while it was being hashed. The timeout stopped short of the final rename. Each one is fixed, with a test that failed first.

The later rounds found narrower and narrower races, like another process on the same Mac swapping files in the folder mid-write. That's where I drew the line on purpose. The helper defends against an untrusted network and against stale or half-written files. A hostile process already running in the build's own workspace can do far worse than race a rename, so it's out of scope, and the design note will say so in plain words. A guard that doesn't say what it won't catch is claiming more than it does.
