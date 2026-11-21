# Contributing

Shellborne does not accept pull requests from outside the project. Only the maintainer and the project's own
build account can open one, and the repository is set that way. Bug reports are welcome as issues; security reports
go through SECURITY.md.

## Checks on your machine

Run `tools/install-hooks` once after cloning. From then on, every push runs `tools/ci-local` once per pushed
ref, on the commit at its tip; gates that read history check every commit the push adds. It starts by feeding every
gate an input it must accept and at least one it must reject, each with the message the gate should give, and only
when all of them answer correctly does it run the gates on that commit. Three
gates read the pull request's own description, so a push lists them as NOT RUN; they run once the pull request is
open. The last line of every run counts what passed, failed, broke and did not run.

You can run it at any time with `tools/ci-local`. If it stops before judging your commit, one of the gates no
longer tells good input from bad; please open an issue rather than working around it.

## Sign-off (Developer Certificate of Origin)

Every commit must be signed off under the Developer Certificate of Origin
(https://developercertificate.org). Add the line with `git commit -s`:

    Signed-off-by: Your Name <your.email@example.com>

Signing off states that you wrote the change or otherwise have the right to submit it under the
project's license. Pull requests with unsigned commits fail the checks.

There is no CLA. Contributions are accepted under the Apache License 2.0, the same license as the code.

## AI-assisted work

AI tools are welcome. Say in the pull request which tool you used and what you verified by running it.
You are responsible for every line you submit, whoever or whatever wrote it.

## Security problems

Do not file them as issues. See SECURITY.md.
