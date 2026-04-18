# 🦊 AgentFox: The Browser Built for AI Agents

## 🚀 Announcement: AgentFox is Now Open Source!

After months of building and testing, I'm thrilled to announce that **AgentFox** is now available as open-source software! 🎉

AgentFox is a high-velocity, persistent browser runtime built specifically for AI agents - not humans, not testers, but for the agentic loop.

### Why AgentFox Exists

Traditional browsers are slow and clunky for AI agents. Testing frameworks like Playwright are overkill and designed for different use cases. AgentFox fills the gap with:

- **⚡ Blazing Fast:** ~10ms command latency once loaded (no cold starts!)
- **👁️ Agent-Native Snapshots:** `afox view` returns clean Markdown with stable IDs
- **🎯 Direct-ID Interaction:** No brittle CSS selectors - use `[e1]`, `[e2]` style IDs
- **💾 Persistent Sessions:** Maintain cookies, logins, and JS state across calls
- **🪶 Lightweight:** Rust + WebKitGTK (lower footprint than Chromium)
- **🔧 Zero Config:** Daemon auto-starts, just works

### Perfect For AI Agent Workflows

AgentFox optimizes the **Inspect → Reason → Act** cycle:

1. **Search & Navigate:** `afox search "latest AI news"`
2. **Inspect:** `afox view` → Clean Markdown tree with interactive elements
3. **Act:** `afox click e15`, `afox fill e21 "my query"`

### Getting Started

One-line installation:
```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```

*(Requires Rust and WebKitGTK dev headers)*

### Try the Demo
See AgentFox in action:
```bash
cd AgentFox/demo
./demo.sh
```

### Real-World Example
```bash
# Search for latest AI agent frameworks
afox search "open source ai agent frameworks 2026"

# Get clean, parsable view
afox view
# Returns markdown like:
# ## [e10] Search Results
# - [e15] (link) [Agents are the future](https://example.com/article)
# - [e21] (input) Search: [ ... ]

# Interact instantly
afox click e15
afox fill e21 "agentfox vs playwright"
afox eval "document.title"
```

### Why Open Source?

I built AgentFox to solve my own pain points as an AI agent developer, but I know many others face the same challenges. By making it OSS:

- 🔧 Community can improve and extend it
- 🛡️ Transparency - you can trust what runs in your agent workflows
- 🤝 Collaborative evolution - shaped by real agent use cases
- 🆓 Free for everyone to use and build upon

### Join the Journey

AgentFox is still early but already proving useful for:
- Real-time web search and data fetching
- Automated form filling and interaction
- Monitoring and scraping dynamic sites
- Building AI agent workflows that need web access

**Give it a try:** https://github.com/MaenExists/AgentFox

Feedback, issues, and contributions welcome! Let's build the best browser for AI agents together. 🦊

#AIAgents #OpenSource #BrowserAutomation #WebScraping #AgenticAI #Rust #WebKitGTK