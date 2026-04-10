# AgentFox — Build Instructions

## First Read

Before changing code, read:

1. `AGENTS.md`
2. `project.md`

Those files define the product.
This file defines the build approach.

## Alignment Rule

Do not build AgentFox as:

- a Playwright wrapper
- a Puppeteer wrapper
- a generic automation SDK
- a test framework with shell commands

Build it as:

- a persistent daemon
- with a thin CLI
- with semantic outputs
- with real browser interaction
- with speed and low memory as top-level requirements

## Recommended Build Sequence

### Step 1: Initialize the workspace

Use a Rust workspace with separate crates for:

- CLI
- daemon
- shared protocol/types

Suggested shape:

```text
AgentFox/
├── cli/
├── daemon/
├── protocol/
├── AGENTS.md
├── project.md
└── README.md
```

### Step 2: Build command transport first

Before deep browser work, get this right:

- persistent daemon process
- Unix socket or similarly cheap local IPC
- structured JSON request/response protocol

This is mandatory because command overhead is part of the product.

### Step 3: Prove persistent browser state

Do not spawn a new browser per command.

The first real technical milestone is:

- daemon starts browser backend once
- `afox open <url>` reuses that live browser
- subsequent commands operate on the same page/session

### Step 4: Build the minimum useful command loop

In order:

1. `open`
2. `snap`
3. stable element ids
4. `text`
5. `click`
6. `fill`
7. `eval`
8. `search`

`search` should be in the MVP, not treated as optional polish.

### Step 5: Measure before expanding scope

As soon as the command loop works, measure:

- cold start time
- warm command latency
- daemon memory usage
- behavior on real modern sites

If the backend undermines the speed thesis, replace it early.

## Backend Guidance

Start with the lightest backend that can:

- render real modern websites
- maintain session state
- execute JavaScript
- support form and click interaction

Candidate backends:

- **WebKitGTK**
  - good bootstrap choice
  - lighter than Chrome-based stacks
  - acceptable if it keeps the command loop fast

- **Chromium-based runtime**
  - fallback if site compatibility requires it
  - risk: too heavy for the product thesis

The backend is not the product.
Do not become attached to an implementation that prevents AgentFox from being meaningfully lighter/faster than existing tools.

## Practical Rule For Every Feature

For each addition, ask:

1. Does this reduce or increase command overhead?
2. Does this improve the agent loop directly?
3. Does this push AgentFox toward being a true browser for agents, or toward being another automation wrapper?

If the answer to 3 is the second one, stop and correct course.

## First Real Milestone

A real early milestone is not just “page title prints.”

A real milestone is this:

```bash
afox open https://example.com
afox snap
afox text e1
afox click e2
```

with:

- one persistent daemon
- one persistent browser session
- semantic output
- working interaction

## Definition Of “On Track”

The build is on track only if:

- the command interface feels shell-native
- the browser session persists between commands
- interaction is real, not simulated loosely
- the data returned is semantic and agent-usable
- latency and memory are treated as product requirements from the start

## Final Reminder

The point is not to prove that browser automation from a CLI is possible.

The point is to build a browser runtime that agents can use directly, faster and lighter than the current generation of browser automation tools.
