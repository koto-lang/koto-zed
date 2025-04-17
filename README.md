# koto-zed
Support for Koto in Zed

### Features
- Diagnostics
- Syntax highlighting
- Comment toggeling
- Auto-closing brackets
- Code Folding
- Zed commands: `Go to Definition`, `Find all references`, `Rename Symbol` (these work fine in pure Koto scripts)

## To add
- Outline (whatever I tried to put into `languages/koto/outline.scm` just made the extension not working anymore at all)
- Autoformat (has to be implemented at the language server)

## Nice to have, later on
- Hover (Not supported yet by Koto language server, not sure whether it is supported by zed for extensions and also there is no doc format spec for Koto)

## Installation (as Zed Dev Extension for now)
If no locally installed language server `koto-ls` binary gets detected, the extension will try to automatically download and install the latest version of the language server from Github.
1. Download and unzip koto-zed: https://github.com/rsaccon/koto-zed/archive/refs/heads/main.zip
2. Install Dev Extension (from Command Palette or Menu `Zed > Extensions`)
