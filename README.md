# 🦊 AgentFox

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Status: Production Grade](https://img.shields.io/badge/Status-Production--Grade-brightgreen.svg)]()

**AgentFox** is a high-performance, persistent browser runtime built from the ground up for AI agents.

Stop treating your agents like web testers. Traditional frameworks like Puppeteer and Playwright are designed for human-authored test scripts. **AgentFox is designed for the agentic loop.**

---

## 🔥 Why AgentFox?

*   **Zero Latency Loop:** No cold starts. The daemon (`afoxd`) keeps the browser warm. Commands execute in milliseconds.
*   **Persistent State:** Navigate, click, and fill across multiple CLI calls. The session stays alive exactly where you left it.
*   **Semantic Intelligence:** Forget raw HTML dumps. `afox snap` returns a structured, agent-readable tree of interactive elements with stable IDs.
*   **Lightweight & Fast:** Powered by WebKitGTK and Rust. Meaningfully lower CPU and memory footprint than Chromium-based automation.
*   **CLI-Native:** Perfect for shell-based agent loops or any language that can spawn a process.

---

## 🚀 One-Line Installation

Install AgentFox to your local system in seconds:

```bash
curl -sSL https://raw.githubusercontent.com/user/AgentFox/main/install.sh | bash
```

*Prerequisites: Rust and WebKitGTK headers (e.g., `libwebkit2gtk-4.1-dev` on Linux).*

---

## 📖 The Agentic Workflow

AgentFox is designed for the **Inspect -> Reason -> Act** cycle.

### 1. Start the Runtime
```bash
afoxd &
```

### 2. Navigate & Search
```bash
afox search "latest news on quantum computing"
```

### 3. Inspect the Semantic Surface
```bash
afox snap
```
Returns:
```json
{
  "url": "https://example.com",
  "elements": [
    {"id": "e1", "role": "heading", "text": "Quantum Leap"},
    {"id": "e2", "role": "link", "text": "Read More", "href": "/article/123"}
  ]
}
```

### 4. Direct Interaction
```bash
afox click e2
afox fill e5 "subscribe@example.com"
```

---

## 🛠 Commands At a Glance

| Command | Description |
|---|---|
| `afox search <q>` | Smart search navigation (URL or query). |
| `afox open <url>` | Direct navigation. |
| `afox snap` | Get a semantic representation of the live page. |
| `afox click <id>` | Perform a realistic browser-level click. |
| `afox fill <id> <val>` | Input text into fields. |
| `afox text <id>` | Extract text or value from an element. |
| `afox eval <code>` | Run arbitrary JS as an escape hatch. |
| `afox quit` | Shutdown the runtime. |

---

## 🏗 Built for the Modern Agent Stack

AgentFox is the missing "Browser tool" for your agent. Whether you are building with LangChain, AutoGPT, or a custom Rust/Python loop, AgentFox provides the fastest and lightest path to the web.

---

## 📜 License
AgentFox is open-source software licensed under the **MIT License**.

---

*“AgentFox isn't just another automation tool; it's a browser runtime that thinks like an agent.”*
