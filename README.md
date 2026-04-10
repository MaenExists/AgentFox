# AgentFox

AgentFox is a fast, lightweight, persistent browser runtime built specifically for AI agents.

Unlike traditional browser automation frameworks (Puppeteer, Playwright), AgentFox is designed to be **agent-native**. It runs as a persistent daemon (`afoxd`) with a thin CLI surface (`afox`), allowing agents to browse, inspect, and interact with the web with minimal overhead and zero cold-start latency between actions.

## 🚀 Quick Install

To install AgentFox to your local path (`~/.local/bin`):

```bash
make install
```

*Prerequisites: [Rust](https://www.rust-lang.org/tools/install) and WebKitGTK development libraries (e.g., `libwebkit2gtk-4.1-dev` on Ubuntu/Debian).*

## 📖 How It Works

1.  **Start the Daemon:** The browser state lives in `afoxd`.
    ```bash
    afoxd &
    ```
2.  **Interact via CLI:** Commands reuse the live browser session.
    ```bash
    afox search "google.com"
    afox snap
    afox click e12
    ```

## 🛠 Command Reference

| Command | Usage | Description |
|---|---|---|
| `search` | `afox search <query>` | Smart navigation: URLs go direct, queries go to search engine. |
| `open`   | `afox open <url>`     | Navigate to a specific URL. |
| `snap`   | `afox snap`         | Get a **semantic** JSON snapshot of the page with stable IDs. |
| `text`   | `afox text <id>`     | Extract visible text or value from a specific element. |
| `click`  | `afox click <id>`    | Perform a realistic browser-level click interaction. |
| `fill`   | `afox fill <id> <text>` | Input text into fields/textareas. |
| `eval`   | `afox eval <code>`    | Run arbitrary JavaScript in the page context. |
| `quit`   | `afox quit`         | Gracefully shut down the daemon. |

## 🤖 Agent Interaction Loop

AgentFox is optimized for the **Inspect -> Reason -> Act** cycle:

1.  **Navigate:** `afox search "Hacker News"`
2.  **Inspect:** `afox snap` (Agent receives clean, semantic JSON)
3.  **Act:** `afox click e10` (Agent clicks a link by its stable ID)
4.  **Repeat:** The session persists, keeping memory usage low and speed high.

## 🏗 Workspace Structure

- `cli/`: The thin `afox` interface.
- `daemon/`: `afoxd`, the persistent browser engine (powered by WebKitGTK).
- `protocol/`: Shared JSON types for low-latency IPC over Unix sockets.

## 📝 Documentation
For detailed usage instructions and agent integration tips, see [USAGE.md](./USAGE.md).
