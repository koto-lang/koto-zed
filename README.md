# koto-zed
Support for Koto in Zed

### Features
- Diagnostics
- Syntax highlighting
- Comment toggeling
- Auto-closing brackets
- Code Folding
- Outline Panel (only top level assignments for now)
- Zed commands: `Go to Definition`, `Find all references`, `Rename Symbol`

## Language server
The extension will make use of `koto-ls` if it's available in your path, otherwise it will try to download and install the latest release from Github. If you want to use latest pre-release (if available), then you have
to enable it in the `lsp` section of your `settings.json`:
```
"lsp": {
  "koto-ls": {
    "settings": {
      "pre_release": true
    }
  }
}
```

## Installation
As Zed Dev Extension for now (once it is published to Zed extensions, it can be selected to be installed right from within the editor).
1. Download and unzip koto-zed: https://github.com/rsaccon/koto-zed/archive/refs/heads/main.zip
2. Install Dev Extension (from Command Palette or Menu `Zed > Extensions`)

## Roadmap
- Hover
- Making langauag server aware of what is made available to Koto runtime via Rust API
- Autoformat
