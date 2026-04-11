# 🦊 AgentFox

**AgentFox** is a high-velocity, persistent browser runtime built specifically for AI agents.

While traditional browsers are for humans, and frameworks like Playwright are for testers, **AgentFox is for the agentic loop.** It provides a daemon-backed, persistent browser surface that eliminates the latency of traditional automation tools.

---

## 🔥 Why AgentFox?

*   **Fast as Fuck:** **~10ms** command latency once the page is loaded. No cold starts, no overhead.
*   **Agent-Native Snapshots:** `afox view` returns a **Clean Markdown** tree of interactive elements with stable IDs, pruned of utility noise (accessibility skips, cookie banners, etc.).
*   **Direct-ID Selection:** High-speed interaction using `[e1]`, `[e2]` style IDs that map directly to internal DOM elements, bypassing brittle CSS selectors.
*   **Persistent Session:** Maintain cookies, logins, and JS state across multiple independent CLI calls.
*   **Lightweight:** Powered by Rust and WebKitGTK. Meaningfully lower resource footprint than Chromium-based stacks.
*   **Zero Configuration:** The daemon (`afoxd`) is automatically started by the CLI if it's not already running. It just works.

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

### 1. Search & Navigate
No need to manually start a daemon. Just run a command.
```bash
afox search "latest news on autonomous agents"
```

### 2. Inspect (Agent-Friendly Markdown)
```bash
afox view
```
Returns a clean, machine-readable element tree:
```markdown
# Google Search
URL: https://www.google.com/search?q=...

## [e10] Search Results
- [e15] (link) [Agents are the future](https://example.com/article)
- [e21] (input) Search: [ ... ]
- [e25] <button> Search </button>
```

### 3. Act Instantly
```bash
afox click e15
afox fill e21 "agentfox vs playwright"
```

---

## 🛠 Commands

| Command | Usage | Description |
|---|---|---|
| `search` | `afox search <q>`   | High-speed navigation (URL or search query). |
| `view`   | `afox view`         | Get a clean Markdown snapshot of the page. |
| `snap`   | `afox snap`         | Get the raw semantic JSON tree. |
| `click`  | `afox click <id>`    | Direct-ID browser interaction. |
| `fill`   | `afox fill <id> <v>` | Instant form input with event triggering. |
| `text`   | `afox text <id>`     | Extract clean text content from an element. |
| `eval`   | `afox eval <code>`    | Escape hatch for raw JS execution. |
| `ping`   | `afox ping`         | Check if the daemon is alive (starts it if not). |
| `quit`   | `afox quit`         | Shutdown the background daemon. |

---

## 🏗 Architecture

AgentFox uses a client-daemon architecture to eliminate the overhead of traditional browser control:

- `cli/`: The `afox` interface (Rust). Starts `afoxd` automatically.
- `daemon/`: The `afoxd` browser engine (Rust + WebKitGTK).
- `protocol/`: Shared high-speed JSON protocol for CLI-daemon IPC.

---

## 📜 License
AgentFox is open-source software licensed under the [MIT License](LICENSE).
