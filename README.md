# 🦊 AgentFox

**AgentFox** is a high-velocity, persistent browser runtime built specifically for AI agents.

While traditional browsers are for humans, and frameworks like Playwright are for testers, **AgentFox is for the agentic loop.** It provides a daemon-backed, persistent browser surface that eliminates the latency of traditional automation tools.

---

## 🔥 Why AgentFox?

*   **Fast as Fuck:** **~10ms** command latency. No cold starts. The daemon (`afoxd`) keeps the browser hot.
*   **Agent-Native Snapshots:** `afox snap` returns a **Turbo-Snap**—a semantic, compressed JSON tree of interactive elements with stable IDs.
*   **Direct-ID Selection:** High-speed interaction using `[data-afox-id]` selectors, bypassing the overhead of traditional CSS/XPath calculation.
*   **Persistent Session:** Maintain cookies, logins, and JS state across multiple independent CLI calls.
*   **Lightweight:** Powered by Rust and WebKitGTK. Meaningfully lower resource footprint than Chromium-based stacks.

---

## 🚀 One-Line Installation

Install the AgentFox suite to `~/.local/bin`:

```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```

*Note: Requires Rust and WebKitGTK development headers (`libwebkit2gtk-4.1-dev` on Linux).*

---

## 📖 The Agentic Loop

AgentFox is optimized for the **Inspect -> Reason -> Act** cycle.

### 1. Start the Engine
```bash
afoxd &
```

### 2. Search & Navigate
```bash
afox search "latest news on autonomous agents"
```

### 3. Inspect (Turbo-Snap)
```bash
afox snap
```
Returns a clean, machine-readable element tree:
```json
{
  "url": "https://example.com/news",
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
| `click`  | `afox click <id>`    | Direct-ID browser interaction. |
| `fill`   | `afox fill <id> <val>` | Instant form input with event triggering. |
| `text`   | `afox text <id>`     | Extract clean text content from an element. |
| `eval`   | `afox eval <code>`    | Escape hatch for raw JS execution. |
| `ping`   | `afox ping`         | Check if the daemon is alive. |
| `quit`   | `afox quit`         | Shutdown the runtime gracefully. |

---

## 🏗 Architecture

AgentFox uses a client-daemon architecture to eliminate the overhead of traditional browser control:

- `cli/`: The `afox` interface (Rust).
- `daemon/`: The `afoxd` browser engine (Rust + WebKitGTK).
- `protocol/`: Shared high-speed JSON protocol for CLI-daemon IPC.

---

## 📜 License
AgentFox is open-source software licensed under the [MIT License](LICENSE).
