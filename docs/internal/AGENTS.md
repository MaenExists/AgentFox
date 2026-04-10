# AgentFox — Build Context

You are building **AgentFox**, a browser for AI agents.

## What AgentFox Is

AgentFox is **not** another browser automation library like Puppeteer or Playwright.
It is **not** primarily an SDK for human-authored test automation.
It is **not** a Chrome wrapper with a CLI bolted on.

AgentFox is a **persistent, agent-facing browser runtime** that AI agents can use as a fast CLI tool.

The product is:

- A long-running browser daemon
- A very thin CLI command surface
- A semantic interaction model for agents
- Fast command execution with persistent state

An agent should be able to do things like:

```bash
afox search "google.com"
afox open https://news.ycombinator.com
afox snap
afox click e12
afox fill e7 "hello world"
afox text e4
```

The command surface is the interface.
The daemon is the engine.
The browser backend is an implementation detail.

## The Real Problem Being Solved

The problem is not "how do we automate a browser?"

The problem is:

- Existing browser control stacks are too heavy
- Existing browser control stacks are too slow between actions
- Existing tools are designed for human test authors, not agent loops
- Agents need repeated browse-read-act cycles with low latency
- Spawning or reconnecting browser control stacks repeatedly wastes too much time and memory

AgentFox exists to make browser use by agents:

- **lighter**
- **faster**
- **more semantic**
- **more CLI-native**

## Product Thesis

If this build is aligned, it should feel closer to:

- `curl` for read speed
- a shell tool for composability
- a browser runtime for interaction

and farther from:

- Playwright test code
- Puppeteer scripts
- "headless browser as a library"

The key difference is this:

Puppeteer and Playwright are tools to *script a browser*.
AgentFox should be a *browser that agents use directly*.

## Non-Negotiable Principles

1. **Speed is the product**
   Every design choice should be judged by latency and memory first.

2. **Daemon-first**
   The browser process stays alive. Commands should reuse live state.

3. **CLI-native**
   The primary interface is terminal commands, not a programming SDK.

4. **Semantic over raw**
   Agents should get structured, useful output instead of raw DOM dumps when possible.

5. **Persistent interaction**
   An agent must be able to browse, inspect, click, fill, and continue in the same live session.

6. **Low-spec friendly**
   This should be viable on cheap or weak hardware.

7. **Do not drift into “yet another automation wrapper”**
   If a change makes AgentFox look more like a wrapper around an existing automation framework than a true agent runtime, reconsider it.

## What Must Be True For The MVP

The MVP is not "some browser commands work."

The MVP is:

- an AI agent can use `afox` commands as a real browsing interface
- the daemon stays alive across commands
- interaction with modern websites is possible
- performance is meaningfully lighter/faster than mainstream browser automation stacks
- the output format is optimized for agent reasoning

## MVP Command Shape

At minimum, the CLI should support a command set like:

```bash
afox search "google.com"
afox open https://example.com
afox snap
afox text e1
afox click e2
afox fill e3 "query"
afox eval "document.title"
afox quit
```

Notes:

- `search` should be treated as a first-class product command, not an afterthought
- `snap` should return a semantic tree or semantic element list
- element ids must remain stable enough for subsequent commands in the same page state
- commands should feel like shell tools, not like RPC debug helpers

## Architecture Direction

The ideal architecture is:

```text
Agent
  -> afox CLI
  -> AgentFox daemon
  -> browser engine/runtime
```

Responsibilities:

- **CLI**
  - parse commands
  - connect fast
  - print machine-usable results

- **Daemon**
  - hold browser state in memory
  - maintain page/session/context state
  - process commands quickly
  - avoid repeated expensive setup

- **Browser engine**
  - render and execute real web apps
  - support DOM interaction, JS, navigation, and form state
  - stay as lightweight as possible

## Backend Guidance

The backend is a tactical choice, not the identity of the project.

A temporary backend is acceptable for bootstrapping if it helps prove the command model.
But do not confuse "working on WebKit" with "product solved."

Backend decisions should be judged by:

- startup cost
- warm command latency
- steady-state memory usage
- reliability with real sites
- ability to support repeated interactions cleanly

If a backend blocks the performance thesis, replace it.

## What To Avoid

Avoid building:

- a Playwright clone with a CLI wrapper
- a test framework
- a screenshot-first browser tool
- a raw HTML fetcher with a few click hacks
- a system where every command spins up expensive browser state
- a design that depends on large amounts of glue code around a heavyweight browser stack

Avoid success theater:

- "it works on example.com" is not enough
- "it exposes commands" is not enough
- "it can evaluate JS" is not enough

The product only becomes real when the agent loop is actually fast, persistent, and useful on real web apps.

## Build Priorities

Build in this order:

1. Persistent daemon and command transport
2. Real page open/load in a reused browser session
3. Semantic snapshot output with stable element references
4. Real click/fill/text workflows on modern pages
5. Search command and better browse ergonomics
6. Performance measurement and optimization
7. Session/context handling

## Required Mindset

When making tradeoffs, prefer:

- less overhead over more abstraction
- fewer moving parts over generic architecture
- direct command usefulness over elegant internals
- measurable speed over theoretical completeness

If starting from scratch, stay aligned to this sentence:

**AgentFox is a fast, lightweight, persistent browser runtime for AI agents, exposed as a CLI, designed to make agent browsing dramatically lighter and faster than existing browser automation tools.**
