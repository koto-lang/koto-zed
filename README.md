# koto-zed
Support for Koto in Zed

### Features
- Diagnostics
- Syntax highlighting
- Comment toggeling
- Auto-closing brackets
- Code Folding
- Zed commands: `Go to Definition`, `Find all references`, `Rename Symbol` (these work fine in pure Koto scripts)

## Maybe comig sson
- Outline (in theory, easy to add, but whatever treesitter query I try to put into `languages/koto/outline.scm` just makes the extension not working anymore at all)

## Being worked on, but still at very, very early stage
- Hover
- Custom settings (e.g.: enable/disble download and installation of pre-release versions of the language server)
- Making langauag server aware of what is made available to Koto runtime via Rust API
- Autoformat

## Installation (as Zed Dev Extension for now)
If no locally installed language server `koto-ls` binary gets detected, the extension will try to automatically download and install the latest version of the language server from Github.
1. Download and unzip koto-zed: https://github.com/rsaccon/koto-zed/archive/refs/heads/main.zip
2. Install Dev Extension (from Command Palette or Menu `Zed > Extensions`)
