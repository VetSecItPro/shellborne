# Shellborne

A free, open-source Mac terminal built for working with AI coding agents like Claude Code and Codex.

If your day runs on several agent sessions at once, the terminal they live in matters more than it
used to. Close a window by accident, let an update restart the app, or have one runaway pane drag the
others down, and work that was in flight is gone. Shellborne is being built so that never happens.

## What it is for

- **Sessions outlive the app.** Every session runs in a background process that Shellborne's window
  only connects to. If the app quits, crashes or updates, your agents keep running and you pick up
  where you were.
- **One bad pane stays one bad pane.** A misbehaving session cannot take the others down with it.
- **Exact input.** What you type reaches the agent exactly as you sent it.
- **Status you can trust.** When Shellborne does not know what a session is doing, it says so
  instead of guessing.
- **Local by default.** No account, no sign-in and no analytics. Nothing leaves your Mac unless you
  choose to send it.
- **It runs your agents as they are.** Shellborne starts the exact command you give it and never
  changes an agent's permissions, sandbox or settings.

## Status

Early and in the open. There is nothing to download yet. Follow along at
[shellborne.dev](https://shellborne.dev), where the devlog covers each step as it happens.

## How it is built

Native to the Mac: Swift and AppKit for everything with a window, Rust for the background session
processes, and the [Ghostty](https://ghostty.org) terminal engine underneath.

Much of the code is written by AI coding agents, and that is said plainly. Every change is tested
first, checked by automated gates, reviewed by a second AI model when the change is risky, and
approved by a person before it merges.

## License and policies

Free and open source under the [Apache License 2.0](LICENSE). The name and logo are covered by the
[trademark policy](TRADEMARKS.md). Report security problems through the
[security policy](SECURITY.md). Contributions follow [CONTRIBUTING.md](CONTRIBUTING.md).
