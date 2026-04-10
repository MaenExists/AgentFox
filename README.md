<p align="center">
  <img src="docs/assets/logo.svg" width="150" alt="AgentFox Logo">
</p>

<h1 align="center">AgentFox (afox)</h1>

<p align="center">
  <strong>The High-Performance, Persistent Browser Runtime for AI Agents.</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Language-Rust-orange.svg" alt="Rust"></a>
  <img src="https://img.shields.io/badge/Status-Production--Grade-brightgreen.svg" alt="Status">
  <a href="CONTRIBUTING.md"><img src="https://img.shields.io/badge/PRs-Welcome-brightgreen.svg" alt="PRs Welcome"></a>
</p>

---

## 🦊 What is AgentFox?

**AgentFox** is not another browser automation framework. It's a **browser runtime** designed specifically for the low-latency, high-frequency interaction loops required by modern AI agents.

Traditional tools like Puppeteer and Playwright were built for human-authored test scripts. They are heavy, slow to start, and leak automation internals. **AgentFox** flips the script by providing a persistent, daemon-backed surface that stays "hot," allowing agents to navigate, inspect, and act with near-zero overhead.

---

## 🔥 Key Advantages

### ⚡ Zero Latency Interaction
Existing tools often require re-initializing a browser session or reconnecting a CDP bridge for every action. AgentFox keeps the browser warm in a background daemon (`afoxd`). Commands execute instantly over a local Unix socket.

### 🧠 Semantic-First Inspection
Agents shouldn't have to parse megabytes of raw HTML. The `afox snap` command returns a **Semantic Snapshot**: a compressed, structured tree of interactive elements (links, buttons, inputs) with stable IDs.

### 🔄 Persistent State
Navigate to a site, perform a search, and click a result across independent CLI calls. The daemon maintains the full JS and DOM state, so your agent can "think" between actions without losing context.

### 🍃 Lightweight & Secure
Built in **Rust** and powered by **WebKitGTK**, AgentFox is significantly lighter than Chromium-based stacks. It provides a clean security boundary between your agent logic and the browser engine.

---

## 🚀 One-Line Installation

Get AgentFox up and running in seconds:

```bash
curl -sSL https://raw.githubusercontent.com/user/AgentFox/main/install.sh | bash
```

> **Requirements:** Rust toolchain and WebKitGTK headers (e.g., `libwebkit2gtk-4.1-dev` on Ubuntu/Debian).

---

## 📖 The Agentic Workflow

AgentFox is optimized for the **Inspect -> Reason -> Act** cycle.

### 1. Start the Engine
The daemon keeps the browser instance alive in memory.
```bash
afoxd &
```

### 2. Intelligent Search
Smart navigation resolves URLs or queries instantly.
```bash
afox search "latest breakthroughs in nuclear fusion"
```

### 3. Semantic Snapshots
Get exactly what the agent needs to see.
```json
// afox snap
{
  "url": "https://science.org/fusion",
  "title": "Fusion News",
  "elements": [
    {"id": "e1", "role": "heading", "text": "New Record in Fusion Energy"},
    {"id": "e2", "role": "link", "text": "Read Full Report", "href": "/news/fusion-record"}
  ]
}
```

### 4. Direct Action
Interact using stable, predictable IDs.
```bash
afox click e2
afox fill e5 "agent@agentfox.dev"
```

---

## 🛠 Command Reference

| Command | Purpose |
|---|---|
| `afox search <query>` | Smart navigation: automatically handles URLs or search queries. |
| `afox open <url>`     | Direct navigation to a specific URL. |
| `afox snap`           | Generates a semantic JSON snapshot of the current page. |
| `afox click <id>`     | Triggers a realistic, multi-event click on an element. |
| `afox fill <id> <val>`| Inputs text into form fields and triggers input events. |
| `afox text <id>`      | Extracts the clean text content or value of an element. |
| `afox eval <code>`    | Executes arbitrary JavaScript as an escape hatch. |
| `afox quit`           | Shuts down the background daemon gracefully. |

---

## 🏗 Architecture

AgentFox uses a client-daemon architecture to ensure performance:

```text
  [ Agent Logic ] 
        |
  [ afox CLI ] <--- (JSON over Unix Socket) ---> [ afoxd Daemon ]
                                                      |
                                               [ WebKit Engine ]
```

---

## 📚 Resources

- **[Getting Started](docs/getting-started.md):** A deep dive into integrating AgentFox with your agent.
- **[Contributing](CONTRIBUTING.md):** How to help build the future of agentic browsing.
- **[License](LICENSE):** MIT Licensed.

---

<p align="center">
  Built for the agentic era. Built for speed. Built by 🦊 <strong>AgentFox Contributors</strong>.
</p>
