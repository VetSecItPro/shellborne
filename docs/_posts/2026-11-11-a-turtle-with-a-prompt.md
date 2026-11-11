---
layout: post
date: 2026-11-11 14:00:00 -0600
title: A turtle with a prompt on its back
image: /assets/devlog/shellborne-app-icon.png
description: How Shellborne got its icon in four rounds, and the rules the mascot has to live by.
---
Before there was any code, Shellborne needed a face. An app icon sits in the Dock all day, so it has to be clear at a glance and still look like something when macOS shrinks it to the size of a fingernail.

The name comes first. A shellback is a sailor who has crossed the equator. Airborne means you trusted your gear and jumped. Both are veteran traditions, and both describe what I want from a terminal: every session goes out, and every session comes back.

It took four rounds.

**Round one** tried a shell under a parachute, a shell seen from above, and a friendly face. On paper the parachute was the clever one, since it says "shell" and "airborne" in one picture. None of the three survived.

**Round two** tried a glossy dome, a baby turtle, and a logo mark. The dome kept reading as a honeycomb, and the baby turtle as something made for kids.

**Round three** handed the brief to AI image models: six options, then rounds of refinement. I picked and pushed back. A rule came out of it: show a real `>_` prompt, the way a terminal draws it, on a turtle you can recognize when it is tiny.

**Round four** went flat. The prompt is printed straight on a gold shell, the turtle walks forward, and the background is deep teal. Every candidate was judged on a contact sheet at every size from sixteen pixels up, on light and dark backgrounds, because that is where an icon spends its life: Finder lists, tabs, the app switcher. The winner was traced into clean vector layers, and the smallest sizes got their own simpler drawing, since the full one turns to mush there.

{% include figure.html src="/assets/devlog/shellborne-app-icon.png" alt="The Shellborne app icon: a teal turtle walking right, with a dark terminal prompt printed on its gold shell, on a deep teal square" caption="The final icon" size="small" %}

{% include figure.html src="/assets/devlog/shellborne-icon-sizes.png" alt="The icon drawn at six sizes on a dark background, from sixteen pixels up to five hundred and twelve; the smallest ones use a simpler drawing" caption="The size test" note="The smallest sizes use their own simpler drawing." %}

The mascot is a separate character, a small turtle called Hatch for now. The name works four ways: a hatchling, a ship's hatch, an aircraft hatch, and the escape hatch every engineer wants.

{% include figure.html src="/assets/devlog/hatch-mascot-concept.jpg" alt="A concept drawing of Hatch: a smiling green turtle peeking out of its shell, holding a gold key with a terminal prompt on it" caption="Hatch, a concept" note="Drawn with an AI image model. A professional illustrator redraws it before it goes anywhere official." size="small" %}

Hatch comes with rules, and I care about them more than the drawing:

- **Never next to your terminal.** The work area stays clean.
- **Never on a risky decision.** Deleting files or approving an agent is not the moment for cute.
- **Never nags or guilts you.** No sad faces, no streaks.
- **One switch turns it off**, and every screen still makes sense without it.

The drawings are concepts for now. A professional illustrator and a proper name check come before Hatch goes anywhere official.
