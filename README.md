# Semgrep for Zed

[Semgrep](https://semgrep.dev/) language server integration for [Zed](https://zed.dev).

Identify bugs and security issues in your code in real-time. This extension brings the power of `semgrep lsp` directly into your editor, offering **zero-config** installation and "Professional" defaults so you see findings immediately.

## Features

-   **Zero-Config Setup**: Works out of the box.
    -   Automatically detects if `semgrep` is installed.
    -   **Auto-Install**: If missing on macOS/Linux, it attempts to install Semgrep via `brew` or `pip3`.
    -   **Smart Defaults**: Runs with `configuration: ["auto"]` and `only_git_dirty: false` so you see findings in all files immediately without needing a `.semgrep.yml`.
-   **Real-time Diagnostics**: Highlights bugs and security vulnerabilities as you type.
-   **Full Configuration**: Supports Zed's `settings.json` to override binary paths, environment variables, and scan rules.

## Installation

1.  Open Zed.
2.  Press `cmd-shift-x` to open the Extensions view.
3.  Search for `Semgrep` and click Install.

*Note: The extension will attempt to verify your `semgrep` installation on first run. If you are on Windows, or if auto-install fails, you will need to install Semgrep manually.*

### Manual Installation (If needed)

If the auto-installer cannot run on your system and Semgrep is not found:

-   **macOS**: `brew install semgrep`
-   **Linux/WSL**: `pip install semgrep` (or check [Semgrep Installation Docs](https://semgrep.dev/docs/getting-started/))
-   **Windows**: Install via WSL or Pip, then configure the path in Zed settings.

## Configuration

You can customize the extension in your Zed `settings.json`.

### Recommended Settings

The extension uses sensible defaults, but you can override them.

```json
{
  "lsp": {
    "semgrep": {
      "initialization_options": {
        "scan": {
          // Choose specific rulesets (defaults to ["auto"])
          "configuration": ["p/security-audit", "p/secrets"],
          // Set to true to only scan modified lines (less noise)
          "only_git_dirty": false
        }
      }
    }
  }
}
```

### Advanced Settings (Binary Path)

If `semgrep` is in a non-standard location or you want to use a specific version:

```json
{
  "lsp": {
    "semgrep": {
      "binary": {
        "path": "/path/to/custom/semgrep",
        "arguments": ["lsp", "--debug"],
        "env": {
          "SEMGREP_APP_TOKEN": "your-token-here"
        }
      }
    }
  }
}
```

## Troubleshooting

### "Semgrep not found"
If you see this error in the binary, it means the auto-installer (brew/pip) failed.
1.  Open a terminal.
2.  Run `semgrep --version` to verify it's installed.
3.  If not, run `brew install semgrep` or `pip3 install semgrep`.

### No Underlines / Diagnostics?
1.  **Check the logs**: Run `zed: open log` in the command palette.
2.  **Verify file type**: Semgrep only scans supported languages (JS, TS, Python, Go, Java, etc.).
3.  **Check Rules**: Ensure your configuration isn't empty. The default is `auto`, but if you have a local `.semgrep.yml` that matches nothing, you won't see results.

## Contributing

Issues and Pull Requests are welcome!

1.  Clone the repo.
2.  Run `cargo check` to verify the Rust code.
3.  Submit a PR.

## License

MIT