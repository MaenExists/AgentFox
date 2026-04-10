# AgentFox — Build Context

You are building **AgentFox**, a lightweight browser CLI for AI agents.

## The Vision

Afox is not another Puppeteer/Playwright. Those are browser automation tools for humans. Afox is a browser **for agents** — built from the ground up to be fast, lightweight, and agent-native.

- **Speed**: Everything should feel instant. No process spawn per command.
- **Lightweight**: Orders of magnitude less memory than Chrome-based tools.
- **Agent-native**: Output semantic trees, not raw HTML. Commands feel like CLI tools.

## Core Philosophy

1. **Speed over features** — Get the core working first, then add polish.
2. **Semantic over raw** — Agents reason better with structured data, not HTML dumps.
3. **CLI-native** — Think `curl`, `wget`, not "headless browser SDK".
4. **Daemon-first** — The daemon stays alive. Connections are persistent.

## The Architecture

```
Agent → Afox CLI → Afox Daemon → Browser Engine
                    (long-running)
```

- **CLI**: Thin wrapper, sends commands to daemon via Unix socket/JSON-RPC
- **Daemon**: Manages browser contexts, caches state, handles IPC
- **Browser Engine**: Headless browser (we'll start with what's available, likely a minimal WebKit or Chromium in headless mode)

## How to Approach This Build

### Phase 1: Get Something Running

The goal is a working MVP that actually works, not a spec document.

1. Choose a browser backend that can be embedded or invoked — options:
   - **WebKitGTK** with GTK and libsoup — lightweight, open source
   - **wkhtmltopdf** — headless WebKit, simple CLI
   - **Chromium headless** — heavier but reliable, works out of box
   - **Selenium-style with any headless browser**

2. Start simple: just get a page to load and return its text content.

3. Build from there: clickable elements, forms, navigation.

### Phase 2: Make It Agent-Friendly

- Define the semantic tree output format
- Add caching for repeated requests
- Implement context isolation

### Phase 3: Polish

- Error handling, timeouts
- Session save/load
- Memory optimization
- Packaging

## What Success Looks Like (MVP)

```bash
# These commands work:
afox open https://example.com
afox snap          # Returns semantic tree
afox click e2     # Click element by ID
afox fill e3 "query"  # Fill input
afox text e1      # Get text content
```

That's it. That's the MVP. Everything else comes after.

## Key Constraints

- **Low-spec machine**: Optimize for low memory usage. No bloat.
- **Speed is critical**: Every command should be < 100ms when possible.
- **Keep it simple**: Don't over-engineer. Ship working code.

## The Person You're Building For

Maen is a self-taught tinkerer on a tight budget. He broke his potato computer but keeps building. He wants tools that work on low-end hardware. He's impatient with fluff — show him working code.

---

**Build with intent. Ship fast. Iterate.**