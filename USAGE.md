# AgentFox Usage Guide

AgentFox (`afox`) is a fast, lightweight, daemon-backed browser runtime designed specifically for AI agents. It allows agents to interact with the web through a persistent session using simple CLI commands.

## Installation

To build and install AgentFox as a CLI tool on your system:

1. **Build the project:**
   ```bash
   cargo build --release
   ```

2. **Install the binaries:**
   Move the binaries to a directory in your `PATH` (e.g., `/usr/local/bin` or `~/.local/bin`):
   ```bash
   cp target/release/afox ~/.local/bin/
   cp target/release/afoxd ~/.local/bin/
   ```

## Running AgentFox

AgentFox operates in a client-daemon model. The daemon (`afoxd`) must be running to process commands from the CLI (`afox`).

1. **Start the daemon:**
   ```bash
   afoxd &
   ```
   *Note: The daemon keeps the browser state alive in memory.*

2. **Issue commands via the CLI:**
   ```bash
   afox search "rust lang"
   ```

## Command Reference

| Command | Usage | Description |
|---|---|---|
| `search` | `afox search <query>` | Navigates to a URL or performs a Google search if a query is provided. |
| `open` | `afox open <url>` | Navigates the current session to a specific URL. |
| `snap` | `afox snap` | Returns a semantic JSON representation of the page, including stable element IDs (`e1`, `e2`, etc.). |
| `text` | `afox text <id>` | Returns the visible text or value of a specific element. |
| `click` | `afox click <id>` | Performs a realistic click interaction on an element. |
| `fill` | `afox fill <id> <text>` | Fills an input or textarea with the specified text. |
| `eval` | `afox eval <code>` | Executes arbitrary JavaScript in the page context. |
| `ping` | `afox ping` | Checks if the daemon is alive. |
| `quit` | `afox quit` | Gracefully shuts down the daemon. |

## For Agents: How to Use AgentFox

If you are an AI agent, you should follow this interaction loop:

1. **Navigate:** Use `afox search` or `afox open` to reach a destination.
2. **Inspect:** Use `afox snap` to get the semantic tree of the page.
3. **Reason:** Identify the element IDs (`eN`) you want to interact with based on their `role` and `text`.
4. **Act:** Use `afox click`, `afox fill`, or `afox text` to interact with those elements.
5. **Repeat:** Take another `snap` to see the results of your action.

### Example Agent Loop:
```bash
# 1. Search for a topic
afox search "Hacker News"

# 2. Get the page state
afox snap

# 3. Read text of a specific item (e.g., e12)
afox text e12

# 4. Click a link
afox click e12
```

## Protocol Details
The CLI and Daemon communicate over a Unix socket at `/tmp/afox.sock` using a JSON-based protocol. Commands are processed sequentially within the persistent browser session.
