# AgentFox

**AgentFox** is a fast, lightweight, persistent browser runtime for AI agents, exposed as a CLI, designed to make agent browsing dramatically lighter and faster than existing browser automation tools.

## Core Vision

AgentFox is built for the **Inspect -> Reason -> Act** cycle of AI agents. Unlike traditional browser automation tools (Playwright, Puppeteer) which are designed for testing and human-authored scripts, AgentFox is designed to be used directly by agents through a low-latency CLI.

## Key Principles

1.  **Speed is the Product**: Minimal latency and low memory footprint.
2.  **Persistent Session**: Keep browser state (cookies, JS state) alive between independent CLI calls.
3.  **Agent-Native Output**: Clean Markdown representation of pages with stable element IDs.
4.  **CLI-First**: Thin CLI interface over a long-running daemon.

## Architecture

- **`afox` (CLI)**: A thin wrapper that sends commands to the daemon.
- **`afoxd` (Daemon)**: The browser engine (WebKitGTK) that maintains state and executes commands.
- **`protocol`**: A high-speed JSON protocol for communication between CLI and Daemon.

## Usage Example

```bash
afox search "latest news on autonomous agents"
afox view
afox click e15
afox fill e21 "agentfox vs playwright"
```
