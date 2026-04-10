# AgentFox

AgentFox is a browser runtime for AI agents.

It is not intended to be another browser automation framework.
It is intended to be a fast, lightweight, daemon-backed browser surface that agents can use directly through CLI commands.

## Product Direction

AgentFox should feel like:

- a shell-native browser tool
- a persistent runtime
- a semantic interface for agent loops

It should not feel like:

- Puppeteer with shell commands
- Playwright with thinner syntax
- a test framework

The core loop is:

1. open or search
2. inspect
3. act
4. inspect again

The daemon must keep browser state alive so this loop stays fast.

## Workspace

- `cli/`: `afox`, the thin CLI interface
- `daemon/`: `afoxd`, the persistent browser daemon
- `protocol/`: shared JSON command/response types

## Current State

The current prototype already has a working daemon-backed command loop with:

- `open`
- `snap`
- `text`
- `click`
- `fill`
- `eval`
- `quit`

The next alignment work is focused on making the product more clearly agent-native:

- first-class `search`
- better command ergonomics
- stronger performance measurement
- less backend leakage into the product surface

## Source Of Truth

If rebuilding from scratch, use these files as the primary context:

- `AGENTS.md`
- `project.md`
- `BUILD.md`
