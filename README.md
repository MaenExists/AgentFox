# 🦊 AgentFox

**AgentFox** is a persistent, lightweight browser runtime designed for AI agents.

Unlike traditional automation frameworks (Puppeteer, Playwright) built for human-authored tests, AgentFox is a daemon-backed CLI tool optimized for the **Inspect -> Reason -> Act** cycle. It provides a "hot" browser surface that agents can use with near-zero latency.

---

## Core Features

*   **Persistent Runtime:** The daemon (`afoxd`) keeps the browser alive. Navigate, click, and interact across independent CLI calls without losing state.
*   **Zero Cold-Start:** Commands execute instantly over a local Unix socket. No re-initializing the browser for every action.
*   **Semantic Snapshots:** `afox snap` returns a structured JSON tree of interactive elements (links, buttons, inputs) with stable IDs, not raw HTML noise.
*   **Built with Rust & WebKit:** Low memory footprint and high performance, using WebKitGTK for real-world site compatibility.
*   **CLI-Native:** Simple command interface for easy integration with any agentic stack (Python, Shell, LangChain, etc.).

---

## 🚀 Installation

Install AgentFox to your local system (`~/.local/bin`):

```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```

*Note: Requires Rust and WebKitGTK development libraries (e.g., `libwebkit2gtk-4.1-dev` on Linux).*

---

## 📖 Quick Start

### 1. Start the Runtime
```bash
afoxd &
```

### 2. Navigate
```bash
afox search "Hacker News"
```

### 3. Inspect the Page
```bash
afox snap
```
Returns a clean, interactive element tree:
```json
{
  "url": "https://news.ycombinator.com/",
  "elements": [
    {"id": "e1", "role": "link", "text": "Hacker News", "href": "news"},
    {"id": "e12", "role": "link", "text": "Show HN: AgentFox", "href": "item?id=..."}
  ]
}
```

### 4. Interact
```bash
afox click e12
afox fill e5 "Hello World"
```

---

## 🛠 Commands

| Command | Usage | Description |
|---|---|---|
| `search` | `afox search <query>` | Smart navigation (URLs or search queries). |
| `snap`   | `afox snap`         | Get a semantic JSON snapshot of the current page. |
| `click`  | `afox click <id>`    | Perform a realistic browser-level click. |
| `fill`   | `afox fill <id> <val>` | Input text into fields/textareas. |
| `text`   | `afox text <id>`     | Extract clean text content from an element. |
| `eval`   | `afox eval <code>`    | Run arbitrary JS in the page context. |
| `quit`   | `afox quit`         | Shutdown the runtime gracefully. |

---

## 🏗 Project Structure

- `cli/`: The `afox` CLI tool.
- `daemon/`: The `afoxd` browser engine (WebKitGTK).
- `protocol/`: Shared JSON protocol for IPC.

---

## 📜 License
AgentFox is open-source software licensed under the [MIT License](LICENSE).
