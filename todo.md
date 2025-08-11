# TODO

This is a living document to track tasks and improvements for the Warp project.

## 📖 Documentation

- [x] Create a comprehensive `README.md` with detailed build and setup instructions.
- [x] Add module-level documentation (`mod.rs`) for all major crates (`warp_core`, `warp_terminal`, `ui`, etc.).
- [ ] Document the custom UI framework and its components.
  - [x] Add module-level documentation for the main UI modules.
- [ ] Add more detailed comments to complex or critical sections of the code, especially in the AI and rendering pipelines.
  - [x] Add high-level comments to the AI and rendering pipelines.
- [x] Create a `CONTRIBUTING.md` guide for new contributors.
- [x] Document the purpose and usage of each of the `generators`.

## 🧪 Testing

- [ ] Increase test coverage for critical components like the terminal model, UI rendering, and AI logic.
  - [x] Set up basic testing environment and add a simple test.
- [ ] Add integration tests for user workflows, such as creating and running commands, using the AI assistant, and managing notebooks.
  - [x] Create basic integration test file for workflows.
  - [x] Create basic integration test file for notebooks.
- [ ] Implement end-to-end tests to simulate user interactions and catch regressions.
  - [x] Outline a high-level plan for E2E testing.
- [ ] Set up a CI/CD pipeline to automatically run tests on every commit.
  - [x] Outline a high-level plan for CI/CD.
- [ ] Add performance benchmark tests to track and prevent regressions.
  - [x] Outline a high-level plan for performance benchmarking.

## ♻️ Refactoring

- [ ] Analyze and refactor complex modules with high cyclomatic complexity.
  - [x] Attempted to use `rust-code-analysis` but encountered dependency resolution issues.
- [ ] Consolidate and deduplicate redundant code, especially in the `settings` and `themes` modules.
  - [x] Outlined areas for consolidation in `settings` and `themes` modules.
- [ ] Improve the separation of concerns between the UI, application logic, and terminal core.
  - [x] Outline a high-level plan for improving separation of concerns.
- [ ] Evaluate and potentially replace deprecated dependencies.
  - [x] Ran `cargo audit` to check for vulnerabilities (none found). Further manual checks might be needed as the project grows.
- [x] Refactor the `legacy.rs` and `v0.rs` files to either remove or modernize the old code.

## ✨ Features

- [ ] Enhance the AI assistant with more advanced capabilities, such as context-aware suggestions and command generation.
  - [x] Outline a high-level plan for enhancing the AI assistant.
- [ ] Expand the library of `generators` with more tools and frameworks.
  - [x] Outline a high-level plan for expanding the library of generators.
- [ ] Improve the notebook feature with better organization and sharing options.
  - [x] Outline a high-level plan for improving the notebook feature.
- [ ] Add support for more terminal themes and customization options.
  - [x] Outline a high-level plan for adding support for more terminal themes and customization options.
- [ ] Implement a plugin system to allow for third-party extensions.
  - [x] Outline a high-level plan for implementing a plugin system.

## 📦 Dependencies

- [ ] Review and update all dependencies to their latest versions.
  - [x] Ran `cargo update` to update compatible versions. Attempted to use `cargo-outdated` but encountered Rust toolchain version issues.
- [ ] Remove any unused or unnecessary dependencies.
  - [x] Outline a high-level plan for removing unused dependencies.
- [ ] Conduct a security audit of all dependencies to identify and mitigate vulnerabilities.
  - [x] Outline a high-level plan for conducting a security audit.

## 🚀 Performance

- [ ] Profile the application to identify and optimize performance bottlenecks, especially in rendering and text layout.
  - [x] Outline a high-level plan for profiling the application.
- [ ] Reduce memory usage by optimizing data structures and caching strategies.
  - [x] Outline a high-level plan for reducing memory usage.
- [ ] Improve startup time by deferring non-essential initializations.
  - [x] Outline a high-level plan for improving startup time.

## 🐞 Bug Fixes

- [ ] Investigate and fix any known issues with terminal rendering and compatibility.
  - [x] Outline a high-level plan for investigating and fixing terminal rendering and compatibility issues.
- [ ] Address any panics or crashes reported in the backtrace logs.
  - [x] Outline a high-level plan for addressing panics and crashes.
- [ ] Fix any UI glitches or inconsistencies across different platforms.
  - [x] Outline a high-level plan for fixing UI glitches and inconsistencies.
- [ ] Resolve any issues with the command completer and suggestion engine.
  - [x] Outline a high-level plan for resolving command completer and suggestion engine issues.
