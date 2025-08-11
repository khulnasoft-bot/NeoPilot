# Warp

A blazingly fast, Rust-powered terminal with AI features.

## Features

*   **GPU-accelerated rendering:** Provides a smooth and responsive terminal experience.
*   **AI-powered command suggestions:** Get intelligent suggestions for your next command.
*   **Workflows and notebooks:** Automate your tasks and organize your commands.
*   **Cross-platform:** Runs on macOS, Linux, and Windows.
*   **Highly customizable:** Customize themes, keybindings, and more.

## Project Structure

The Warp project is a monorepo with the following key crates:

*   `warp_core`: Core data structures and business logic.
*   `warp_terminal`: The main terminal implementation.
*   `ui`: A custom UI framework for rendering the terminal.
*   `warp_util`: Utility functions and helpers.
*   `ai`: AI-related features and logic.

## Getting Started

### Prerequisites

*   [Rust toolchain](https://www.rust-lang.org/tools/install)

### Building

1.  Clone the repository:

    ```bash
    git clone https://github.com/your-username/warp-app.git
    ```

2.  Build the project:

    ```bash
    cargo build --release
    ```

### Running

Once the project is built, you can run the application from the `target/release` directory:

```bash
./warp
```

## Contributing

Contributions are welcome! Please see the `CONTRIBUTING.md` file for more details.
