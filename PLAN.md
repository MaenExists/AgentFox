# AgentFox Project Status

AgentFox is a high-velocity, persistent browser runtime built specifically for AI agents.

## Current State

- **Architecture**: Rust-based CLI (`afox`) and Daemon (`afoxd`) using WebKitGTK.
- **Implemented Commands**:
    - `search <query>`: Smart navigation (URL or search).
    - `open <url>`: Navigate to URL.
    - `snap`: Get semantic JSON snapshot.
    - `view`: Get semantic Markdown snapshot.
    - `click <id>`: Realistic click interaction.
    - `fill <id> <text>`: Input text into fields.
    - `text <id>`: Extract text from element.
    - `eval <code>`: Execute JavaScript.
    - `ping`: Check daemon status.
    - `quit`: Shutdown daemon.
- **Protocol**: Shared JSON-based IPC over Unix domain socket.
- **Installation**: `install.sh` provided for easy setup.

## Technical Details

- **Backend**: WebKitGTK.
- **Persistence**: Browser state is maintained by `afoxd` across CLI calls.
- **Interactivity**: Uses `data-afox-id` for stable element addressing during a session.

## Success Criteria Checklist

- [x] CLI commands operate against a persistent daemon.
- [x] Output is useful to an LLM (Markdown via `view`).
- [x] Basic interaction works on real sites.
- [x] Command-to-command interaction is fast.
- [x] Memory use is relatively low compared to Chromium.

## Known Limitations / Future Work

- [ ] Better handling of multi-page/tab contexts (currently seems to use a single WebView).
- [ ] Improved element stabilization across partial page updates.
- [ ] Performance benchmarking against Playwright/Puppeteer.
- [ ] Support for more complex interactions (hover, drag-and-drop, etc.).
- [ ] More robust error handling for edge cases in page loading.
