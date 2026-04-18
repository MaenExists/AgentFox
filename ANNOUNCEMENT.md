# 🚀 AgentFox is Now Open Source! 

After building it for my own AI agent workflows, I'm excited to share **AgentFox** - a high-velocity browser runtime built specifically for AI agents.

## 🦊 What is AgentFox?

AgentFox solves the pain of web interaction for AI agents:
- **⚡ Blazing fast:** ~10ms command latency (no cold starts!)
- **👁️ Agent-native output:** `afox view` returns clean Markdown with stable IDs
- **🎯 Direct-ID interaction:** No brittle CSS selectors - use `[e1]`, `[e2]` style IDs
- **💾 Persistent sessions:** Maintain cookies, logins, and JS state
- **🪶 Lightweight:** Rust + WebKitGTK (lower footprint than Chromium)
- **🔧 Zero config:** Daemon auto-starts, just works

## 🔥 Why AI Agents Love It

Traditional browsers and testing frameworks are slow and over-engineered for agent needs. AgentFox is built for the **Inspect → Reason → Act** cycle:

```bash
# Search
afox search "latest AI agent frameworks"

# Inspect (get clean, parsable view)
afox view
# Returns: ## [e10] Results - [e15] (link) [Article](url) - [e21] (input) Search: [...]

# Act instantly
afox click e15
afox fill e21 "agentfox comparison"
afox eval "document.title"
```

## 🚀 Getting Started

One-line installation:
```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```
*(Requires Rust and WebKitGTK dev headers)*

Try the demo:
```bash
cd AgentFox/demo
./demo.sh
```

## 🌟 Join the Journey

AgentFox is open source (MIT License) because I believe the best tools for AI agents should be built collaboratively. Whether you're:
- Building AI agents that need web access
- Frustrated with slow browser automation
- Want to contribute to agentic AI infrastructure

**Come build with us:** https://github.com/MaenExists/AgentFox

Feedback, issues, and contributions welcome! Let's make web interaction fast, reliable, and joyful for AI agents together. 🦊

#AIAgents #OpenSource #BrowserAutomation #AgenticAI #Rust #WebKitGTK #LLMTools