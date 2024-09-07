# RustAudioExploration <!-- omit in toc -->

This project is a try to get into digital sound synthesis. I use this project as my first project to learn Rust.
Fortunately, I came across the [tauri](https://tauri.app/) - a library for building user interfaces in Rust that compiles to HTML, JS and CSS.
Since I anyway wanted to learn frontend web development at some point, I decided that is the best chance to combine my plans.

## Table of Contents <!-- omit in toc -->

- [Tech Stack](#tech-stack)
  - [Development](#development)
    - [VS Code Extensions](#vs-code-extensions)
  - [Backend](#backend)
  - [Frontend](#frontend)
- [Resources](#resources)
  - [Rust](#rust)

## Tech Stack

### Development

I develop in [VS Code](https://code.visualstudio.com/). You can find a guide on how to set up debugging for tauri right [here](https://tauri.app/v1/guides/debugging/vs-code/).

#### VS Code Extensions

- Rust
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax)
- HTML
  - [Auto Rename Tag](https://marketplace.visualstudio.com/items?itemName=formulahendry.auto-rename-tag)
  - [HTML Boilerplate](https://marketplace.visualstudio.com/items?itemName=sidthesloth.html5-boilerplate)
  - [HTML CSS Support](https://marketplace.visualstudio.com/items?itemName=ecmel.vscode-html-css)
- TypeScript
  - [Angular Language Service](https://marketplace.visualstudio.com/items?itemName=Angular.ng-template)
  - [Angular Snippets (Version 18)](https://marketplace.visualstudio.com/items?itemName=johnpapa.Angular2)
- Debugging
  - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- Quality of life
  - [Code Spell Checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
- Theme
  - [Tokyo Night](https://marketplace.visualstudio.com/items?itemName=enkia.tokyo-night)
- Misc
  - [Markdown All In One](https://marketplace.visualstudio.com/items?itemName=yzhang.markdown-all-in-one)
  - [Project Manager](https://marketplace.visualstudio.com/items?itemName=alefragnani.project-manager)
  - [YAML](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml)

### Backend

- Rust
  - [cpal](https://github.com/RustAudio/cpal): Low-level library for audio input and output in pure Rust.

### Frontend

- Rust
  - [tauri](https://tauri.app/) (v2.0.0-rc): Frontend framework that compiles to HTML, JS, CSS.
- TypeScript
  - [Angular](https://angular.dev/) (v18): single-page web application framework
  - [PrimeNG](https://primeng.org/): Angular component library

## Resources

Of course, I didn't came up with all of this myself. I got this idea after watching [this](https://youtu.be/iA6wRgwl7k0?si=PPIlMKW-fFX2J7T-) excellent video by [Sebastian Lauge](https://www.youtube.com/@SebastianLague). I highly recommend watching all of his videos if you're into coding.

### Rust

- ["The Book"](https://doc.rust-lang.org/stable/book/): THE resource to learn Rust
