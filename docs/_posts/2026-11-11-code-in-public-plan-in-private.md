---
layout: post
date: 2026-11-11 11:00:00 -0600
title: Code in public, plan in private
description: The code is open from day one. The plan stays on my Mac. A guard checks every commit to keep it that way.
---
Shellborne's code is public from the first commit. Its plan is not.

That split is deliberate. The code should be open so anyone can check what the app really does with their sessions. The plan, the research, the threat notes and the roadmap stay on my Mac, in a folder that has no remote and refuses every push. Plans change daily, and half-finished thinking does not belong on GitHub.

The hard part is keeping the two apart when AI agents do much of the typing. An agent that has read the plan can easily paste a sentence of it into a code comment, a commit message or a web page without anyone noticing.

So before anything else was built, I built a leak guard, and it checks at three points:

1. **When I commit.** Planning material is stopped before it ever lands in the history.
2. **When I push.** If the first check was skipped for any reason, this one still looks.
3. **On GitHub itself.** Public CI looks again, as a last net behind the two on my Mac.

It has a public half and a private half. The public half knows only shapes: planning file names, internal ids, the format of a hidden marker planted in every private document. The private half runs only on my Mac and compares what is about to go public against the private folder itself, down to any run of a dozen words copied from it.

It earned its keep today. While building this site, an agent reused a few lines of styling from a private design mockup. The private guard refused the push, the lines were rewritten, and nothing private went out.

Every guard was tested the same way before I trusted it: plant a known-bad example and watch it get refused. A check that cannot fail is not a check.
