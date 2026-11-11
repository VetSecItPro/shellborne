# Security policy

## Reporting a vulnerability

Please report security problems privately. Do not open a public issue for them.

- **Private report:** use GitHub private vulnerability reporting on this repository
  (the Security tab, then "Report a vulnerability").
- **Contact:** VetSecItPro (GitHub @VetSecItPro), the maintainer, reads every report.

Include what you found, how to reproduce it, and the version or commit you tested.

## What happens next

- Every report is treated as valid until it is shown not to be.
- You get an acknowledgment before any explanation or argument about severity.
- Critical problems are worked first. Their status is updated publicly until the issue is closed.
- If a report shows a real flaw, it is described as a flaw, never relabeled as intended behavior.

## Honest limits

- Shellborne has had no independent human security audit. Changes to its most sensitive parts are
  reviewed by a different AI model family than the one that wrote them, and the code is public for
  anyone to check.
- There are no releases yet. Until there is one, reports apply to the code on the `main` branch.
