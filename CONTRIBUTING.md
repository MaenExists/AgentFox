# Contributing to AgentFox

We welcome contributions! Whether you're fixing a bug, adding a feature, or improving documentation, here's how you can help:

## Development Setup

1.  **Clone the repo:**
    ```bash
    git clone https://github.com/user/AgentFox.git
    cd AgentFox
    ```
2.  **Install dependencies:**
    - Rust (latest stable)
    - WebKitGTK development headers (`libwebkit2gtk-4.1-dev` on Linux).
3.  **Run in dev mode:**
    ```bash
    cargo run -p afoxd &
    cargo run -p afox ping
    ```

## Submitting a PR

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature/my-new-feature`).
3.  Ensure code is formatted: `cargo fmt`.
4.  Run tests: `cargo test`.
5.  Commit changes with clear messages.
6.  Open a Pull Request!

## Code of Conduct

Please be respectful and collaborative. We aim to build a high-quality tool for the agentic community.
