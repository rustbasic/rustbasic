# ezChat

**ezChat** is a lightweight AI chat application built with **Rust**, **egui**, and **WASM (WebAssembly)**. It can run as a browser-based PWA and also supports native desktop development builds.

| Status | Primary Link | Mirror Link |
| :---: | :---: | :---: |
| **Stable** | [https://rustbasic.github.io](https://rustbasic.github.io) | [https://rustbasic.github.io/ezchat](https://rustbasic.github.io/ezchat) |

### 🚀 Live Demo: [https://rustbasic.github.io/](https://rustbasic.github.io/)

![ezChat Screenshot](ezchat-v063.png)

## Overview

ezChat is a Rust/egui chat client focused on AI conversation, voice, avatars, and assisted workflows. The web version is compiled to WebAssembly and can be installed as a PWA, while the native build is useful for local development and deeper tool integration.

*   **Fast & Safe**: Built with the safety and speed of Rust.
*   **Multi-Platform**: Supports browser/WASM and native desktop builds.
*   **PWA Supported**: Installable in supported browsers with offline caching.

## Key Features

### 🤖 Multi-AI Support
*   [x] **Google Gemini** supported.
*   [x] **OpenAI ChatGPT** supported. (Requires OpenAI API key)
*   [x] **XAI Grok** supported. (Requires XAI API key)

### 🎙️ Voice & Media
*   [x] **Voice Recognition**: Talk to the AI instead of typing. (Requires OpenAI API key)
*   [x] **Voice Output**: Listen to the AI's responses. (Requires OpenAI API key)
*   [x] **Image Generation**: Create images with prompts or voice commands. (Requires OpenAI API key)

### 🧑‍🎨 Avatar Experience
*   [x] Supports custom avatar packages with axis metadata.
*   [x] Supports blink and mouth overlay assets.
*   [x] Supports lip-sync style mouth animation for chat responses.

### 💬 Chat Experience
*   [x] Supports waiting room and multiple chat rooms.
*   [x] Supports various conversation topics and themes.
*   [x] Clean and responsive UI using `egui`.
*   [x] Optional auto-continue flow for longer assisted conversations.

### 🛠️ Assisted Workflow
*   [x] Agent-oriented workflow support for programming and project tasks.
*   [x] Local/native builds can use deeper file and tool integration during development.

## Technical Stack
*   **Language**: [Rust](https://www.rust-lang.org/)
*   **UI Framework**: [egui](https://github.com/emilk/egui)
*   **Targets**: Native desktop and WebAssembly (WASM)
*   **Deployment**: GitHub Pages (PWA)