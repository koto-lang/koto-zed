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

## Nice to have but not posssible yet
- Hover (AFAIK not supported by zed for extensions and also there is no doc format spec for Koto)

## Installation (as Zed Dev Extension for now)
If no locally installed language server `koto-ls` binary gets detected, the extension will try to automatically download and install the latest version of the language server from Github.
1. Download and unzip koto-zed: https://github.com/rsaccon/koto-zed/archive/refs/heads/main.zip
2. Install Dev Extension (from Command Palette or Menu `Zed > Extensions`)



```yml
name: Release

permissions:
  contents: write

on:
  push:
    tags:
      - v[0-9]+.*

jobs:
  create-release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/create-gh-release-action@v1
        with:
          # (optional) Path to changelog.
          # changelog: CHANGELOG.md
          # (required) GitHub token for creating GitHub Releases.
          token: ${{ secrets.GITHUB_TOKEN }}

  upload-assets:
    needs: create-release
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: taiki-e/upload-rust-binary-action@v1
        with:
          # (required) Comma-separated list of binary names (non-extension portion of filename) to build and upload.
          # Note that glob pattern is not supported yet.
          bin: koto-ls
          # (optional) Target triple, default is host triple.
          # This is optional but it is recommended that this always be set to
          # clarify which target you are building for if macOS is included in
          # the matrix because GitHub Actions changed the default architecture
          # of macos-latest since macos-14.
          target: ${{ matrix.target }}
          # (optional) On which platform to distribute the `.tar.gz` file.
          # [default value: unix]
          # [possible values: all, unix, windows, none]
          tar: unix
          # (optional) On which platform to distribute the `.zip` file.
          # [default value: windows]
          # [possible values: all, unix, windows, none]
          zip: windows
          # (required) GitHub token for uploading assets to GitHub Releases.
          token: ${{ secrets.GITHUB_TOKEN }}

```
