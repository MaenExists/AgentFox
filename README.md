# 🦊 AgentFox

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status: Production Grade](https://img.shields.io/badge/Status-Production--Grade-brightgreen.svg)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-brightgreen.svg)](CONTRIBUTING.md)

**AgentFox** is a high-performance, persistent browser runtime engineered for the AI agentic era.

Stop treating your agents like web testers. Traditional frameworks like Puppeteer and Playwright were built for human-authored test scripts. **AgentFox is built for the agentic loop.**

---

## 🔥 Key Advantages

*   **Zero Cold-Start:** The daemon (`afoxd`) maintains a hot browser instance. Action latency is measured in milliseconds, not seconds.
*   **Persistent Interaction:** Navigate, click, and fill across independent CLI calls. The session state lives as long as the daemon does.
*   **Agent-Native Snapshots:** `afox snap` provides a clean, semantic JSON representation of the page, optimized for LLM reasoning. No more raw HTML noise.
*   **Lightweight Footprint:** Built with Rust and WebKitGTK. Meaningfully lower memory and CPU overhead compared to Chromium-based stacks.
*   **Simple CLI Interface:** Perfect for shell-based agent loops or any language capable of executing a process.

---

## 🚀 One-Line Installation

Install the AgentFox suite to your local system (`~/.local/bin`):

```bash
curl -sSL https://raw.githubusercontent.com/user/AgentFox/main/install.sh | bash
```

*Prerequisites: Rust toolchain and WebKitGTK development libraries (e.g., `libwebkit2gtk-4.1-dev` on Linux).*

---

## 📖 The Agentic Loop

AgentFox is optimized for the **Inspect -> Reason -> Act** cycle.

### 1. Start the Engine
```bash
afoxd &
```

### 2. Intelligent Navigation
```bash
afox search "latest advancements in AGI"
```

### 3. Semantic Inspection
```bash
afox snap
```
Returns a structured tree:
```json
{
  "url": "https://example.com",
  "elements": [
    {"id": "e1", "role": "heading", "text": "The Future of Agents"},
    {"id": "e2", "role": "link", "text": "Read the Paper", "href": "/research/agi-2026"}
  ]
}
```

### 4. Direct Action
```bash
afox click e2
afox fill e5 "agent@fox.com"
```

---

## 🛠 Commands

| Command | Purpose |
|---|---|
| `afox search <query>` | Smart navigation: URL or search query. |
| `afox snap` | Get a semantic snapshot of the current page. |
| `afox click <id>` | Perform a realistic browser-level click. |
| `afox fill <id> <text>` | Input text into fields. |
| `afox text <id>` | Extract text or value from an element. |
| `afox eval <code>` | Run arbitrary JS as an escape hatch. |
| `afox quit` | Shutdown the runtime gracefully. |

---

## 📚 Documentation
- [Getting Started Guide](docs/getting-started.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Internal Architecture](docs/internal/project.md)

---

## 📜 License
AgentFox is open-source software licensed under the [MIT License](LICENSE).

---

*“AgentFox: A browser that thinks, inspects, and acts at the speed of your agent.”*
