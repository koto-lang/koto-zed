# koto-zed
Support for [Koto](https://github.com/koto-lang/koto) in Zed

Published at [Zed Extension Gallery](https://zed.dev/extensions?query=Koto&filter=language-servers).

### Features
- Diagnostics
- Syntax highlighting
- Comment toggeling
- Auto-closing brackets
- Code Folding
- Outline Panel
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
Right from within Zed, for detailed platform-specific instructions see the [Zed documentation](https://zed.dev/docs/extensions/installing-extensions#installing-extensions).
