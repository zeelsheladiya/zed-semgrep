# Semgrep for Zed

[Semgrep](https://semgrep.dev/) language server support for Zed.

## Features

- **LSP Support**: Integrates Semgrep's language server for real-time static analysis.
- **Multi-language**: Supports JavaScript, TypeScript, Python, Go, Java, Ruby, Rust, C/C++, C#, PHP, Dockerfile, JSON, YAML, Terraform.

## Installation

1. Install Semgrep:
   ```bash
   pip install semgrep
   # or
   brew install semgrep
   ```
2. Install this extension in Zed.

## Configuration

You can configure the extension in your Zed settings.

### Binary Path

If `semgrep` is not in your PATH, you can specify the path to the executable:

```json
{
  "semgrep": {
    "binary_path": "/path/to/semgrep"
  }
}
```

## Troubleshooting

- **Semgrep not found**: Ensure `semgrep` is installed and available in your PATH. You can verify this by running `semgrep --version` in your terminal.
- **No diagnostics**: Ensure your project has semgrep rules configured (e.g., via `.semgrep.yml` or logged in to Semgrep App).
  - Example `.semgrep.yml`:
    ```yaml
    rules:
      - id: test-rule
        patterns:
          - pattern: console.log(...)
        message: "Found a console.log!"
        languages: [javascript]
        severity: WARNING
    ```