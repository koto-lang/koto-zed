# koto-zed
Support for Koto in Zed

### Features
- Diagnostics
- Syntax highlighting
- Comment toggeling
- Auto-closing brackets
- Code Folding
- Zed commands: `Go to Definition`, `Find all references`, `Rename Symbol` (these work fine in pure Koto scripts)

## To troubleshoot (when koto script uses anything defined in Rust)
- Zed commands `Go to Definition` and `Find all references` silently fail
- Zed command `Rename Symbol` cause the following error notification: `Error: Prepare rename via koto_ls failed: No reference found at position`

## To add
- Outline (whatever I tried to put into `languages/koto/outline.scm` just made the extension not working anymore at all)
- Autoformat (has to be implemented at the language server)

## Nice to have but not posssible yet
- Hover (AFAIK not supported by zed for extensions and also there is no doc format spec for Koto)

## Installation (while this extension is not released and published yet)
1. Install Koto language server locally: `cargo install koto-ls` or `cargo install --git https://github.com/koto-lang/koto-ls.git` if you are using the unreleased version of koto
2. Download and unzip koto-zed: https://github.com/rsaccon/koto-zed/archive/refs/heads/main.zip
4. Install Dev Extension (from Command Palette or via GUI at Zed Extensions)
