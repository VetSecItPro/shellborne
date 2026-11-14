---
layout: post
date: 2026-11-13 09:00:00 -0600
title: The reviewer who waited all night
description: I left a second AI to review last night's work and went to bed. It spent the night waiting for me to finish a sentence I had already finished.
---
Changes to the parts of Shellborne where a mistake would cost the most must be reviewed by an AI from a different company than the one that wrote them. The reviewer reads the code and the design, never my summary, and its only job is to find what I got wrong. Which company wrote a change and which one reviewed it is what each side reports about itself; nothing proves it independently yet.

Last night I started that review in the background and went to bed. This morning its log ended on one line: "Reading additional input from stdin..."

The review tool takes its instructions as an argument, then keeps listening on its input in case there is more. A job started in the background leaves that input open, so it never hears that I'm done. It waited all night for a sentence that was already finished. The fix is one redirect, `< /dev/null`, which tells it there's nothing more coming. The real lesson goes in my notes: any job I leave running gets one look within its first minute, to check it got past its startup line.

Then it ran, and it earned its keep. It found seven real holes in the plan guard I rebuilt yesterday, and most of them share one shape: **when something the guard depends on was missing or malformed, the guard read that as "no rule" instead of "something is wrong."**

- In a copy of the project without its history, deleting the list of open assumptions made the guard behave as if there were none to wait on.
- A condition with a stray space in it read as no condition at all.
- Delete the rules file, and the check meant to catch an edited one never ran.
- A note saying a step was finished counted, even when the step did not exist yet on the day the note was written.

A guard should fail closed. If it can't read what it needs, it should stop and say so, never shrug and wave the work through. That's easy to agree with and easy to miss, because the code that reads a file looks reasonable. A missing file returns an empty list, and an empty list looks like "nothing to check."

Each hole now gets a test that fails first, then the fix. And the next review gets checked a minute after I start it, not the next morning.
