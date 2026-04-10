# 🦊 AgentFox

**AgentFox** is a high-velocity browser built specifically for AI agents.

While traditional browsers are built for humans, and automation frameworks (Puppeteer/Playwright) are built for testers, **AgentFox is built for the agentic loop.** It is a daemon-backed, persistent browser that allows agents to search the web and interact with sites with near-zero latency.

---

## ⚡ Why AgentFox?

*   **Faster than existing tools:** No cold starts. The daemon (`afoxd`) keeps the browser "hot" in the background. Commands execute instantly over a local Unix socket.
*   **Persistent Session:** Navigate, click, and browse across independent CLI calls. The browser maintains full session, cookie, and JS state.
*   **Agent-Native Snapshots:** `afox snap` provides a clean, semantic JSON tree of interactive elements. Your agent gets exactly what it needs to reason, not a 1MB HTML dump.
*   **Minimal Overhead:** Built with Rust and WebKitGTK. It's significantly lighter and faster than any Chromium-based solution.
*   **Search First:** A first-class `search` command that intelligently handles both direct URLs and search queries.

---

## 🚀 One-Line Install

Install the AgentFox suite to `~/.local/bin`:

```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```

*Note: Requires Rust and WebKitGTK headers (`libwebkit2gtk-4.1-dev` on Linux).*

---

## 📖 The Agentic Loop

AgentFox is designed for the **Inspect -> Reason -> Act** cycle.

### 1. Start the Browser
```bash
afoxd &
```

### 2. Search & Navigate
```bash
afox search "latest news on autonomous agents"
```

### 3. Inspect the Surface
```bash
afox snap
```
Returns a clean, interactive element tree:
```json
{
  "url": "https://example.com/news",
  "title": "Agent News",
  "elements": [
    {"id": "e1", "role": "heading", "text": "Agents are the future"},
    {"id": "e2", "role": "link", "text": "Read More", "href": "/article/1"}
  ]
}
```

### 4. Act Instantly
```bash
afox click e2
afox fill e5 "agent@agentfox.dev"
```

---

## 🛠 Commands

| Command | Usage | Description |
|---|---|---|
| `search` | `afox search <query>` | High-speed navigation (URL or query). |
| `snap`   | `afox snap`         | Get a semantic JSON snapshot of the page. |
| `click`  | `afox click <id>`    | Realistic browser-level interaction. |
| `fill`   | `afox fill <id> <val>` | Instant form input. |
| `text`   | `afox text <id>`     | Extract clean text content. |
| `eval`   | `afox eval <code>`    | Escape hatch for raw JS execution. |
| `quit`   | `afox quit`         | Shutdown the runtime. |

---

## 🏗 Architecture

AgentFox uses a client-daemon architecture to eliminate the overhead of traditional browser control:

- `cli/`: The `afox` interface.
- `daemon/`: The `afoxd` browser engine (WebKitGTK).
- `protocol/`: High-speed JSON protocol for CLI-daemon communication.

---

## 📜 License
AgentFox is open-source software licensed under the [MIT License](LICENSE).
