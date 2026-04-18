# AgentFox — Project Specification

> Status: Product Definition
> Goal: Build a browser for agents, not another browser automation wrapper

## 1. Product Definition

**AgentFox (`afox`)** is a CLI-accessible browser runtime for AI agents.

It should let an agent browse and interact with the web through fast terminal commands while keeping browser state alive between commands.

This is the target experience:

```bash
afox search "google.com"
afox open https://news.ycombinator.com
afox snap
afox click e2
afox fill e7 "rust webkit"
afox text e4
```

This is not meant to feel like Playwright code translated into shell commands.
It is meant to feel like a real browser surface designed for agents.

## 2. Product Goal

The goal is to make web use by AI agents:

- faster than mainstream browser automation stacks
- lighter on memory and CPU
- easier to use from a shell loop
- semantically readable by an LLM or agent runtime

The main thing being optimized is the repeated cycle:

1. open or search
2. inspect
3. act
4. inspect again
5. continue

If each loop is slow or heavy, the product fails even if the commands technically work.

## 3. What AgentFox Is Not

AgentFox is not:

- a browser test framework
- a Puppeteer clone
- a Playwright clone
- a generic automation SDK with a CLI layer
- just `curl` plus some DOM helpers

If the implementation starts drifting toward “browser automation tool with a command wrapper,” it is moving away from the product.

## 4. Required User Experience

An agent should be able to:

- open a site
- inspect a semantic page representation
- address meaningful page elements by stable ids
- click links and buttons
- fill inputs and textareas
- read targeted text from the page
- execute JS when needed as an escape hatch
- continue interaction in the same live session

The interface should be:

- command-oriented
- low-latency
- machine-readable
- predictable

## 5. MVP Scope

### Required Commands

| Command | Purpose |
|---|---|
| `afox search <query> [-s]` | Open or resolve a destination using a browser-aware flow |
| `afox open <url> [-s]` | Navigate current session to a URL |
| `afox snap [-s]` | Return semantic page state or summary |
| `afox view [-s]` | Return a clean Markdown representation |
| `afox text <element-id>` | Return readable text/value for one element |
| `afox click <element-id>` | Trigger realistic click interaction |
| `afox fill <element-id> <text>` | Fill an input-like field |
| `afox eval <js>` | Escape hatch for page-context JS |
| `afox auth <key> <url> <model>` | Set LLM credentials for summarization |
| `afox quit` | Stop daemon |

### Required `snap` Format

The output must be semantic and compact, not raw HTML.

Example:

```json
{
  "url": "https://example.com",
  "title": "Example Domain",
  "elements": [
    {"id": "e1", "role": "heading", "text": "Example Domain"},
    {"id": "e2", "role": "paragraph", "text": "This domain is for use in documentation examples..."},
    {"id": "e3", "role": "link", "text": "Learn more", "href": "https://iana.org/domains/example"}
  ]
}
```

The semantic layer exists because agents reason better over:

- role
- text
- href/value/state
- stable element ids

than over raw DOM or HTML dumps.

### Required Click Behavior

Click must behave like a real browser interaction path, not a toy event dispatch.

At minimum it should cover the normal event sequence expected by modern apps:

1. `pointerdown`
2. `mousedown`
3. `focus`
4. `mouseup`
5. `click`

If a framework-backed app breaks under click, AgentFox is not yet good enough.

## 6. MVP Success Criteria

The MVP is done only when all of these are true:

- `afox` commands operate against a persistent daemon-backed browser session
- command output is useful to an LLM without postprocessing raw HTML
- `open`, `snap`, `text`, `click`, and `fill` work on real sites, not just toy pages
- command-to-command interaction is fast because browser state is reused
- memory use is clearly below mainstream browser automation workflows on the same machine
- the system feels like a browser tool for agents, not a repackaged test framework

## 7. Performance Requirements

Performance is part of the product, not a later optimization pass.

Targets:

- no cold browser spawn on every command
- warm command execution should feel near-instant where possible
- keep memory low enough for low-spec machines
- benchmark against Puppeteer/Playwright-style workflows, not against static fetch tools

You are not finished when the commands work.
You are finished when they work with the right performance profile.

## 8. Architecture

```text
Agent
  -> afox CLI
  -> AgentFox daemon
  -> browser engine
```

### CLI Responsibilities

- parse command arguments
- connect to daemon quickly
- print structured results
- remain thin

### Daemon Responsibilities

- keep browser runtime alive
- maintain current session/page state
- map stable command ids to live page elements
- execute commands with minimal overhead
- support future contexts/sessions

### Browser Engine Responsibilities

- load and execute real web apps
- maintain DOM/JS state across commands
- support navigation and interaction
- stay as lightweight as possible

## 9. Backend Strategy

The browser backend is a means to an end.

Possible options:

- WebKitGTK
- embedded WebKit-style runtime
- Chromium-derived runtime if unavoidable
- a custom lighter engine path later

Selection criteria:

- warm interaction latency
- memory footprint
- reliability on modern sites
- ease of keeping persistent browser state

Important:

- a temporary backend is acceptable for bootstrapping
- a backend that undermines the performance thesis should not become permanent by inertia

## 10. Build Order

If rebuilding from scratch, do it in this order:

1. Initialize repo and Rust workspace
2. Define shared command/response protocol
3. Implement daemon and persistent command transport
4. Prove a live browser session can stay alive across commands
5. Implement `open`
6. Implement semantic `snap`
7. Implement stable element addressing
8. Implement `text`
9. Implement `click`
10. Implement `fill`
11. Add `search`
12. Benchmark latency and memory
13. Optimize bottlenecks before adding non-essential features

## 11. Out of Scope Until Core Speed Is Proven

Do not prioritize these before the core interaction loop is fast and solid:

- screenshots
- visual diffing
- network interception
- full test framework APIs
- large SDK surfaces
- CI polish
- packaging polish
- browser-specific bells and whistles

## 12. Key Build Rule

When uncertain, choose the option that better supports this statement:

**AgentFox is a fast, lightweight, persistent browser runtime for AI agents, exposed as a CLI, designed to make agent browsing dramatically lighter and faster than existing browser automation tools.**
