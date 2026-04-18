# AgentFox: The Browser Built Specifically for AI Agents

## Why Another Browser Tool?

If you've worked with AI agents that need to interact with the web, you've probably hit this wall: traditional browsers are slow, testing frameworks are overkill, and parsing HTML is a nightmare for LLMs.

AgentFox solves this by being the first browser runtime built **exclusively for AI agents** - not humans, not testers, but for the agentic loop.

## The Problem with Existing Tools

- **Traditional browsers (Chrome/Firefox):** Designed for humans, slow startup, heavy resource usage
- **Testing frameworks (Playwright/Puppeteer):** Built for QA, complex APIs, over-engineered for agent needs
- **Text browsers (lynx/elinks):** Can't handle modern JS sites, no interaction capabilities
- **Raw HTTP clients (curl/wget):** No JS execution, no form handling, no SPA support

## The AgentFox Solution

AgentFox reimagines web interaction from the ground up for AI agents:

### ⚡ Performance That Matters
- **~10ms command latency** after initial load (no cold starts!)
- Persistent daemon keeps browser hot between agent actions
- Startup time under 500ms (vs seconds for traditional tools)

### 👁️ Agent-Optimized Data Extraction
Forget scraping 1MB of HTML. `afox view` returns:
```markdown
# Google Search
URL: https://www.google.com/search?q=...

## [e10] Search Results
- [e15] (link) [Agents are the future](https://example.com/article)
- [e21] (input) Search: [ ... ]
- [e25] <button> Search </button>
```
Clean, semantic markdown with **stable element IDs** that LLMs can actually reason over.

### 🎯 Precise, Reliable Interaction
No more brittle CSS selectors that break when a site updates:
```bash
afox click e15      # Click by stable ID
afox fill e21 "query"  # Fill form by stable ID
afox eval "document.title"  # Execute JS when needed
```

### 💾 Smart State Management
- Persistent cookies, localStorage, and JS state across calls
- Login once, stay logged in for your agent's entire session
- Automatic daemon management - starts when needed, stops when done

### 🪶 Lightweight by Design
- Built with Rust and WebKitGTK
- Significantly lower memory/CPU footprint than Chromium-based tools
- Single binary distribution, zero configuration needed

## Real-World Agent Workflows

### 1. Real-Time Research Assistant
```bash
# Get latest AI news
afox search "AI agent frameworks April 2026"
afox view  # Get clean, parsable results
afox click e12  # Click most promising result
afox view  # Extract article content
afox text e8   # Get key paragraphs
```

### 2. Automated Form Filling & Submission
```bash
# Navigate to job application
afox open https://company.com/careers/apply
afox view  # See form structure
afox fill e5 "Maen"          # First name
afox fill e6 "AgentFox Dev"  # Last name  
afox fill e7 "maen@example.com"  # Email
afox click e10               # Submit
```

### 3. Dynamic Site Monitoring
```bash
# Watch for price changes
afox open https://shop.example.com/product/123
while true; do
  afox view
  # Agent checks price element
  if [ "$(afox text e42)" != "$last_price" ]; then
    # Alert on price change
    send_alert "Price changed: $(afox text e42)"
    last_price="$(afox text e42)"
  fi
  sleep 300  # Check every 5 minutes
done
```

## Getting Started

### Installation (One-Liner)
```bash
curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash
```
*(Requires Rust and WebKitGTK dev headers: `sudo apt install libwebkit2gtk-4.1-dev rustc cargo`)*

### Basic Usage
```bash
# Search the web
afox search "latest developments in multimodal AI"

# Inspect the page (agent-friendly output)
afox view

# Interact with elements
afox click e15      # Click link/button by ID
afox fill e21 "query"  # Fill input by ID
afox text e8        # Extract text content
afox eval "window.localStorage.length"  # Execute JS

# Clean shutdown when done
afox quit
```

## Why Open Source AgentFox?

I built AgentFox to solve my own frustrations as an AI agent developer, but I quickly realized this is a universal problem in the agentic AI space. By making it open source:

### 🤝 Community-Driven Evolution
- Features shaped by real agent use cases, not theoretical needs
- Community can add integrations (LangChain, LlamaIndex, etc.)
- Multi-language SDKs (Python, Node.js, etc.) from community contributions

### 🔒 Trust & Transparency
- You can audit exactly what browser engine your agents are using
- No black boxes or telemetry - pure tool for your workflows
- Security-conscious design with minimal attack surface

### 🚀 Accelerated Innovation
- Faster development through community contributions
- Cross-pollination of ideas from different agent architectures
- Standardized interface that agent frameworks can build upon

## The Vision

AgentFox aims to become the **standard browser interface for AI agents** - the equivalent of what `curl` is for HTTP requests, but for full browser interaction.

Imagine agent frameworks like:
- AutoGPT / BabyAGI having built-in AgentFox support
- LangChain agents using `afox view` as their primary web scraping tool
- Multi-agent systems sharing persistent browser sessions for coordinated tasks

## Join Us

**Repository:** https://github.com/MaenExists/AgentFox  
**Documentation:** https://github.com/MaenExists/AgentFox/tree/main/docs  
**Installation:** `curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash`

### We're Looking For:
- 🐛 Bug reporters and testers (try it in your agent workflows!)
- 💡 Feature suggestions (what would make your agent's web life easier?)
- 👩‍💻 Contributors (Rust, WebKitGTK, documentation, examples)
- 📣 Advocates (if you find it useful, share it with your agent-dev friends)

### Try It Today
1. Install: `curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash`
2. Run the demo: `cd AgentFox/demo && ./demo.sh`
3. Build something amazing with your AI agents

Let's build the best web interaction tool for AI agents together. Because the future of AI isn't just about what models can think—it's about what they can **do** on the web. 🦊

#AIAgents #OpenSource #BrowserAutomation #WebScraping #AgenticAI #Rust #WebKitGTK #LLMTools #WebAutomation