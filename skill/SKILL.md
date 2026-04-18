---
name: agentfox
description: "High-velocity persistent browser runtime. Provides instantaneous web search, navigation, interaction, and page summarization directly from the CLI. This replaces traditional browser automation frameworks and web scraping APIs."
metadata:
  {
    "openclaw":
      {
        "emoji": "🦊",
        "requires": { "anyBins": ["afox"] },
        "install":
          [
            {
              "id": "agentfox-cli",
              "kind": "bash",
              "script": "curl -sSL https://raw.githubusercontent.com/MaenExists/AgentFox/main/install.sh | bash",
              "label": "Install AgentFox CLI",
            },
          ],
      },
  }
---

# AgentFox Skill

AgentFox gives your agent the ability to interact with the web at superhuman speed. It uses a long-running daemon to keep a browser engine warm, reducing the latency of web interactions to mere milliseconds.

## Key Capabilities

1. **Browse & Search**: Navigate to URLs or execute Google searches.
2. **Interact**: Click buttons and links, or fill out input forms using stable semantic IDs instead of brittle CSS selectors.
3. **Summarize**: Automatically extract and summarize page content using LLMs to save your context window.
4. **Snapshot**: Get a clean, agent-readable markdown representation of the current page.

## Commands

The primary binary is `afox`. The background daemon will start automatically on the first command.

### Search and Navigation
- `afox search "<query>"`: Navigate to a URL or perform a search.
- `afox open <url>`: Navigate to a specific URL.

### Interrogation
- `afox view`: Get a clean Markdown representation of the page with element IDs.
- `afox snap`: Get the raw semantic JSON tree of the page.
- `afox text <id>`: Extract the text content from a specific element.

### Interaction
- `afox click <id>`: Perform a realistic click (pointerdown, focus, click) on an element.
- `afox fill <id> "<text>"`: Input text into a form field and trigger change events.

### Summarization
You can append the `--summarize` or `-s` flag to `search`, `open`, `view`, and `snap` to get a concise 2-3 paragraph summary of the page's core content instead of the full layout.

**Requirement**: Summarization requires prior authentication. 
```bash
afox auth <API_KEY> https://opencode.ai/zen/v1 nemotron-3-super-free
```

## Best Practices for Agents

1. **Start with Search + Summarize**: When researching a topic, use `afox search "topic" --summarize` to quickly understand the first result without blowing up your context limit.
2. **Use `afox view` for Interaction**: If you need to click links or fill forms, use `afox view` to see the available elements and their `[eX]` IDs.
3. **Don't Over-Click**: If an element doesn't seem to respond to `afox click`, check if it's a dynamic JavaScript element that might take a moment to load, or use `afox eval` as an escape hatch.
4. **Persistent Session**: Remember that the browser session is persistent. If you click a link that opens a modal, the modal will be there on your next `afox view` call.
