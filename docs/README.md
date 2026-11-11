# The shellborne.dev site and devlog

GitHub Pages builds this folder with its built-in Jekyll and serves it at shellborne.dev. This file is
not published.

## Writing an entry

Each entry tells one real thing that happened while building Shellborne, so a reader comes away
knowing more and wants the next one.

- **One thing per entry:** a decision, a failure that got caught, or something measured. Say what
  happened, why it matters, and what comes next.
- **Around 300 words.** Short paragraphs. A list when it helps, never a wall of text.
- **The owner's voice:** first person, plain and direct, like an experienced engineer talking to a
  peer. Calm, exact and honest. No hype, no exclamation marks, no filler.
- **Numbers are earned:** every timing, count or percentage links to where it came from, such as a
  test run or a pull request. If there is nothing public to link, leave the number out.
- **Never** promise a feature or a date, invent a number, or reuse text from private notes.
- Two or three entries a day are fine when there is real substance, and none when there is not.

## Publishing an entry

1. An agent drafts the entry from the day's work; the owner edits it and approves it.
2. The approved entry goes in `docs/_posts/` as `YYYY-MM-DD-short-title.md` with a `title` and a
   `description` in its front matter, and rides the next pull request.
3. `tools/gates/devlog-check .` must pass: no internal ids, and no number without a link to its
   evidence. The leak guards check every post on commit, push and in CI.
4. When the "where it stands" window changes, update `_includes/side.html` in the same pull request.

## Sharing on LinkedIn

- A short native post in the owner's own words: the one thing that happened and why it matters, with
  one image or a short clip, and a link to the full entry.
- Clips run 30 to 60 seconds, have captions, and are recorded with the Mac's built-in screen
  recording. Small GIFs can live in the repository; full videos go on LinkedIn or YouTube.
