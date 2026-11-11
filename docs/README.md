# The shellborne.dev site and devlog

GitHub Pages builds this folder with its built-in Jekyll and serves it at shellborne.dev. This file is
not published.

## Where things live

```
docs/
  _config.yml          site settings
  _layouts/            page frames: default (every page), post (an entry)
  _includes/           pieces: side windows, entry cards, picture frames
  _posts/              the entries, one Markdown file each
  index.html           the front page
  devlog/index.html    the list of every entry
  about/index.html     the About page
  assets/
    site.css           the whole look, in one file
    brand/             icon, favicon, home-screen icon, the share card
    devlog/            pictures and screenshots used in entries
    fonts/             the pixel font and its license
```

Nothing else goes at the top of `assets/`. A new kind of file gets its own folder and a line here.

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
2. The approved entry goes in `docs/_posts/` as `YYYY-MM-DD-short-title.md` with a `title`, a
   `description` and a `date` with the time in its front matter, and rides the next pull request.
3. `tools/gates/devlog-check .` must pass: no internal ids, and no number without a link to its
   evidence. The leak guards check every post on commit, push and in CI.
4. When the "where it stands" window changes, update `_includes/side.html` in the same pull request.

## Pictures and screenshots

- Put the file in `docs/assets/devlog/`, named for what it shows. Screenshots as PNG, photos and
  drawings as JPEG, and keep each one under about 300 KB.
- Show it in a window frame, with alt text that says what the picture shows:

  ```
  {% include figure.html src="/assets/devlog/name.png" alt="What it shows" caption="Window title" %}
  ```

  Add `note="..."` for a line under the picture and `size="small"` for a narrow one. Wrap two figures
  in `<div class="pair">` and `</div>` to set them side by side.
- Add `image: /assets/devlog/name.png` to the front matter to give the entry a picture in the entry
  lists. Every page shares the same turtle card on LinkedIn, so no per-entry card is needed.
- Check every screenshot before it goes in: no paths, account names, tokens or private notes on it.

## Sharing on LinkedIn

- A short native post in the owner's own words: the one thing that happened and why it matters, with
  one image or a short clip, and a link to the full entry.
- Clips run 30 to 60 seconds, have captions, and are recorded with the Mac's built-in screen
  recording. Small GIFs can live in the repository; full videos go on LinkedIn or YouTube.
