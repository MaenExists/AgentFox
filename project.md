# AgentFox — Project Specification

> **Status**: MVP Build
> **Goal**: A working lightweight browser CLI that AI agents can use

---

## 1. What is AgentFox?

**AgentFox (afox)** is a CLI tool that gives AI agents the ability to browse the web, interact with websites, click buttons, fill forms, and extract data — but way faster and lighter than existing tools like Puppeteer or Playwright.

It works like this:

```
AI Agent
    ↓ (shell command)
afox open https://google.com
    ↓ (IPC)
Afox Daemon (running in background)
    ↓
Browser Engine (headless)
    ↓
Result returned to agent
```

---

## 2. Why This Exists

- **Puppeteer/Playwright are too heavy** — They run full Chrome, use ~450MB+ memory per instance
- **Agents don't need visual rendering** — They just need DOM access, JS execution, and interaction
- **Speed matters** — Agents make hundreds of requests; slow browser tools kill productivity
- **CLI feel** — Agents should call it like they call `curl` or `wget`

---

## 3. MVP Scope

### Commands to Build

| Command | Description |
|---------|-------------|
| `afox open <url>` | Navigate to a URL |
| `afox snap` | Get a semantic tree of the current page |
| `afox click <element-id>` | Click an element by ID |
| `afox fill <element-id> <text>` | Fill an input field |
| `afox text <element-id>` | Get text content of an element |
| `afox eval <js-code>` | Run JavaScript in page context |
| `afox quit` | Shutdown the daemon |

### What "Snap" Returns

A semantic tree, NOT raw HTML:

```json
{
  "url": "https://example.com",
  "title": "Example Domain",
  "elements": [
    {"id": "e1", "role": "heading", "text": "Example Domain"},
    {"id": "e2", "role": "link", "text": "More information...", "href": "https://www.iana.org/-example"},
    {"id": "e3", "role": "paragraph", "text": "This domain is for use in illustrative examples..."}
  ]
}
```

**Why semantic?** Because raw HTML is useless to an LLM. This format is readable, actionable, and small (~90-95% smaller than raw HTML).

### What "Click" Does

It fires the **full event sequence** that real browsers fire (not just `click`):

1. `pointerdown`
2. `mousedown`
3. `focus`
4. `mouseup`
5. `click`

This ensures React/Vue/Angular apps don't break.

---

## 4. Technical Design

### Stack

- **CLI**: Rust (single binary, no dependencies)
- **Daemon**: Rust with tokio (async, stays alive)
- **Browser Backend**: WebKit2 via `webkit2gtk` (headless) — lightweight, proven
- **IPC**: Unix socket with JSON messages

### Project Structure

```
AgentFox/
├── cli/
│   ├── Cargo.toml
│   └── src/main.rs       # CLI entry point
├── daemon/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs       # Daemon entry point
│       └── browser.rs    # Browser wrapper
│       └── command.rs    # Command handlers
└── README.md
```

### Daemon Behavior

1. Starts on first CLI call (or manual start)
2. Creates a Unix socket at `/tmp/afox.sock`
3. Waits for commands, processes them, returns JSON
4. Can handle multiple browser contexts (future)
5. Logs to `/tmp/afox.log` for debugging

### CLI Behavior

1. Parses command-line args
2. Connects to daemon socket
3. Sends JSON command
4. Prints result to stdout
5. Exits (no daemon needed in same process)

---

## 5. Build Requirements

### System Dependencies (Ubuntu/Debian)

```bash
# Install WebKit and dependencies
sudo apt install libwebkit2gtk-4.1-dev libssl-dev libsoup-3.0-dev

# Install Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build Commands

```bash
# Build daemon
cd daemon && cargo build --release

# Build CLI
cd cli && cargo build --release

# Or build both with Makefile (create one)
```

### Run It

```bash
# Start daemon (background)
./target/release/afoxd &

# Use it
./target/release/afox open https://example.com
./target/release/afox snap
./target/release/afox click e2

# Kill daemon
./target/release/afox quit
```

---

## 6. Error Handling

- If daemon isn't running, CLI starts it automatically
- Invalid element IDs return clear error: `"Element 'e5' not found"`
- Network errors return: `"Failed to load: <url> - <error>"`
- Timeout after 30 seconds for any page operation

---

## 7. Out of Scope (For Now)

These come AFTER the MVP works:

- Multiple contexts (session isolation)
- Session save/load (cookies, localStorage)
- Network interception
- Screenshots
- Chrome fallback
- Tests, CI/CD, packaging

---

## 8. Success Criteria

The MVP is done when:

- [ ] `afox open <url>` loads a page and returns success
- [ ] `afox snap` returns a semantic tree with clickable element IDs
- [ ] `afox click eN` actually clicks things (links, buttons work)
- [ ] `afox fill eN "text"` fills input fields
- [ ] Works on low-spec hardware (< 200MB RAM for daemon + browser)
- [ ] Each command responds in < 2 seconds

---

## 9. Example Session

```bash
$ afox open https://news.ycombinator.com
{"status": "ok", "url": "https://news.ycombinator.com", "title": "Hacker News"}

$ afox snap
{
  "url": "https://news.ycombinator.com",
  "title": "Hacker News",
  "elements": [
    {"id": "e1", "role": "heading", "text": "Hacker News"},
    {"id": "e2", "role": "link", "text": "new", "href": "..."},
    {"id": "e3", "role": "link", "text": "past", "href": "..."},
    {"id": "e4", "role": "link", "text": "Ask HN", "href": "..."},
    {"id": "e5", "role": "link", "text": "Show HN", "href": "..."},
    ...
  ]
}

$ afox click e2   # Click "new" link
{"status": "ok", "url": "https://news.ycombinator.com/new"}

$ afox snap
{
  "url": "https://news.ycombinator.com/new",
  "title": "New Stories | Hacker News",
  "elements": [...]
}
```

---

**Ready to build. Start with getting a headless browser to load a URL and return content. Everything else builds on that.**