---
layout: post
date: 2026-11-11 13:00:00 -0600
title: Five times the guards said no
description: A day of building the checks that keep an AI-built project honest, and the five times they stopped something.
---
Today was mostly about guardrails, the unglamorous part that lets agents write code without me reading every byte. The good news is that they work, because they kept saying no. Here are five times they did.

**1. The plan guard refused its own author's push.** The tool that checks every commit against the plan blocked my first push of the day. The change was marked high-risk and was missing the design note those changes require. The note was written and the push went through.

**2. A second AI reviewer found a list of holes.** Every high-risk change gets reviewed by a different AI model than the one that wrote it. On the plan checker, that reviewer found a long list of real problems, including a way to claim work was finished without proof. Each one got a test that failed first, then a fix.

**3. CodeQL caught a planted bug.** To prove the security scanner actually fails a pull request, we planted an unsafe file that wrote a password to the log. The check went red. We removed the file and it went green. Proof, not assumption.

**4. A personal email address slipped into a GitHub-made commit.** The machine account that opens pull requests was missing one privacy setting, so the test-merge commit GitHub builds for every pull request carried a personal email address. The leak guard refused it, so it never reached the main branch. One setting fixed it for good.

**5. My own commit, before the tests had run.** I committed before reading a test result, and the commit held five failures. Now a guard refuses any code commit unless both test suites passed on exactly that code.

None of this is visible in the app yet, because there is no app yet. But it is the reason I'll be able to move fast later without breaking the promise that matters most: your sessions survive.
