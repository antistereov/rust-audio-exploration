# RustAudioExploration <!-- omit in toc -->

This project is a try to get into digital sound synthesis, learn [Rust](https://www.rust-lang.org/) and frontend web development using [Angular](https://angular.dev/).

In my real life, I'm actually a musician and I have always enjoyed electronic music and digital sound synthesis quite a lot - from a user perspective.
I knew that at some point, I want to get into the fundamentals of digital sound synthesis - and even sound synthesis in general but I had no idea of how and when.
At the sime, I've wanted to learn Rust but never came up with a project idea.

Fortunately, after watching [this](https://youtu.be/iA6wRgwl7k0) excellent video by [Sebastian Lauge](https://www.youtube.com/@SebastianLague) (I highly recommend watching all of his videos if you're into coding!) it became quite obvious to me how I can achieve all of my goals at the same time. I will build my own synthesizer using Rust.

Another thing I've always wanted to do was frontend web development (since other projects forced me to do it). So I was blown away after I came across [tauri](https://tauri.app/). This way I can build my frontend using [Angular](https://angular.dev/) (this is used by the frontend guys at the company that I work at, so I thought they can help me here), my backend using Rust and get into sound synthesis while working on this project.

Please don't take this project too seriously. It's just a fun side project for me. I'm happy for any tips, tricks and comments on my work!

If you want to check out my music:

- [Spotify](https://open.spotify.com/artist/7sqTmMbUpkrMB4thBUrah7?si=o70wji4GTDaF-tuC-Bh9YA)
- [YouTube](https://www.youtube.com/@antistereov)

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
