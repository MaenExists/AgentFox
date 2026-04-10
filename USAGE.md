# AgentFox Developer Guide

This guide provides deep technical insights into using AgentFox effectively in your agent loops.

## Architecture: The Daemon Model

AgentFox splits the browser into two parts:
1.  **`afoxd` (The Engine):** A long-running process that manages the WebKit instance, keeps the JS environment alive, and maintains page state.
2.  **`afox` (The Interface):** A thin CLI client that sends JSON-encoded commands over a Unix Domain Socket (`/tmp/afox.sock`).

### Why this matters for agents:
Traditional tools spin up a new browser or reconnect an expensive CDP session for every action. AgentFox keeps the socket open and the browser hot, reducing interaction latency by up to **90%**.

## Semantic Snapshotting (`snap`)

The `snap` command is the heart of AgentFox. Instead of forcing your LLM to parse 1MB of raw HTML, AgentFox does the heavy lifting:
- **Filtering:** Removes invisible elements, noise, and non-interactive boilerplate.
- **Roles:** Assigns roles like `link`, `button`, `input`, and `heading`.
- **Stable IDs:** Elements are assigned IDs (`e1`, `e2`, etc.) that remain stable as long as the page doesn't undergo a major navigation.

### Strategy for Prompting:
When passing a snapshot to an LLM, instruct it to:
> "Analyze the provided page snapshot and respond with the command `afox click <id>` or `afox fill <id> <text>` to achieve the goal."

## Realistic Interactions

AgentFox doesn't just dispatch "click" events. It triggers the full browser event lifecycle:
1. `pointerdown`
2. `mousedown`
3. `focus`
4. `mouseup`
5. `click`

This ensures compatibility with modern React/Next.js/Vue applications that rely on complex event listeners for interaction.

## Best Practices

- **Warm Starts:** Always start `afoxd` in the background before your agent begins its task.
- **Error Handling:** If `afox` returns an error (e.g., "Element not found"), your agent should perform a new `snap` to see if the DOM has shifted.
- **Memory Management:** The daemon is lightweight, but if you're running long sessions, a periodic `afox quit` and restart can ensure a fresh state.

## Integration Examples

### Python (using `subprocess`)
```python
import subprocess
import json

def get_snapshot():
    result = subprocess.run(["afox", "snap"], capture_output=True, text=True)
    return json.loads(result.stdout)

def click_element(element_id):
    subprocess.run(["afox", "click", element_id])
```

### Shell Loop
```bash
# Simple auto-browse loop
afox open https://news.ycombinator.com
while true; do
  afox snap
  # Your agent logic here
  sleep 2
done
```
