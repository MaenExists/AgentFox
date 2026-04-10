# AgentFox — Build Instructions

## Quick Start for Codex

1. **Read these files first:**
   - `AGENTS.md` — How to approach this build
   - `project.md` — What to build (spec)

2. **Create the Rust project structure:**

```bash
# Create workspace with two crates
cd /home/maen/Builds/AgentFox

# CLI crate
cargo new --bin afox

# Daemon crate  
cargo new --bin afoxd
```

3. **Pick your browser backend:**
   
   **Option A: WebKit (recommended for MVP)**
   - Use `webkit2gtk` crate with `--headless` flag
   - Pro: Lightweight, proven, works well
   - Con: GTK dependency on Linux
   
   **Option B: Chromium (heavier but reliable)**
   - Use `headless` Chrome via subprocess
   - Pro: Chrome works everywhere
   - Con: ~450MB memory, slow startup

   **Start with A** — WebKit is faster and lighter.

4. **Build incrementally:**
   - First: Get a URL to load and return page title
   - Second: Extract clickable elements
   - Third: Implement click, fill, eval
   - Fourth: Add semantic tree output

5. **Test early and often:**
   - Test with: Google, Hacker News, a login page
   - Verify click actually works (not just returns "ok")
   - Check memory usage: should be < 200MB total

---

## Your First Target

Get this working first:

```rust
// Pseudocode - make it real
fn main() {
    let browser = Browser::new_headless();
    browser.navigate("https://example.com");
    let title = browser.title();
    println!("Page title: {}", title);
}
```

If you can do that, everything else is incremental.

---

**Go build. Ship the MVP first. Polish later.**