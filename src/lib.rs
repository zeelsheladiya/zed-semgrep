use serde_json::{json, Value};
use zed_extension_api::{self as zed, Result};

struct SemgrepExtension;

/// Merge `overlay` into `base` (deep merge for JSON objects).
fn deep_merge(base: &mut Value, overlay: Value) {
    match overlay {
        Value::Object(overlay_map) => {
            if !base.is_object() {
                *base = Value::Object(serde_json::Map::new());
            }
            let base_map = base.as_object_mut().expect("base is object");
            for (k, v) in overlay_map {
                match base_map.get_mut(&k) {
                    Some(existing) => deep_merge(existing, v),
                    None => {
                        base_map.insert(k, v);
                    }
                }
            }
        }
        other => {
            *base = other;
        }
    }
}

impl zed::Extension for SemgrepExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Read user settings for overrides
        let lsp_settings = zed::settings::LspSettings::for_worktree("semgrep", worktree).ok();
        let cmd_settings = lsp_settings.as_ref().and_then(|s| s.binary.as_ref());
        let semgrep_cmd = cmd_settings
             .and_then(|b| b.path.clone())
             .unwrap_or_else(|| "semgrep".to_string());

        let mut args = cmd_settings
            .and_then(|b| b.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_string()]);
        
        // Ensure "lsp" is the default argument if none provided or empty
        if args.is_empty() {
            args.push("lsp".to_string());
        }

        let env = cmd_settings
            .and_then(|b| b.env.clone())
            .unwrap_or_default()
            .into_iter()
            .collect();

        // Check for the binary in PATH or worktree.
        if let Some(path) = worktree.which(&semgrep_cmd) {
            return Ok(zed::Command {
                command: path,
                args,
                env,
            });
        }


        // On Windows, auto-install via shell script is not reliable.
        let (platform, _) = zed::current_platform();
        if platform == zed::Os::Windows {
            return Ok(zed::Command {
                command: semgrep_cmd,
                args,
                env,
            });
        }

        // For macOS/Linux, try to auto-install if the default "semgrep" command is missing.
        // If the user specified a custom path, we respect it and let it fail naturally if invalid.
        if semgrep_cmd != "semgrep" {
             return Ok(zed::Command {
                command: semgrep_cmd,
                args,
                env,
            });
        }

        let script = r#"
if ! command -v semgrep >/dev/null 2>&1; then
    echo "Semgrep not found. Attempting auto-installation..." >&2
    if command -v brew >/dev/null 2>&1; then
        echo "Installing via Homebrew..." >&2
        brew install semgrep >&2
    elif command -v pip3 >/dev/null 2>&1; then
        echo "Installing via pip3..." >&2
        pip3 install --user semgrep >&2
        export PATH="$(python3 -m site --user-base)/bin:$PATH"
    else
        echo "Error: Semgrep not found and no suitable package manager (brew, pip3) detected." >&2
        echo "Please install semgrep manually." >&2
        exit 1
    fi
fi

if ! command -v semgrep >/dev/null 2>&1; then
    if command -v python3 >/dev/null 2>&1; then
        exec python3 -m semgrep lsp
    fi
    echo "Error: Semgrep installation appeared to succeed but 'semgrep' binary is still not found." >&2
    exit 1
fi

exec semgrep lsp
"#;
        // Use a shell wrapper to handle the conditional install before starting the LSP.
        Ok(zed::Command {
            command: "/bin/sh".to_string(),
            args: vec!["-c".to_string(), script.to_string()],
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<Value>> {
        // Defaults chosen to make diagnostics appear with minimal/no configuration:
        // - configuration: ["auto"] mirrors Semgrep's VS Code docs behavior
        // - only_git_dirty: false so you see underlines even on clean files
        let mut init = json!({
            "scan": {
                "configuration": ["auto"],
                "only_git_dirty": false
            }
        });

        if let Ok(lsp_settings) = zed::settings::LspSettings::for_worktree("semgrep", worktree) {
            if let Some(user_init) = lsp_settings.initialization_options {
                deep_merge(&mut init, user_init);
            }
        }

        Ok(Some(init))
    }
}

zed::register_extension!(SemgrepExtension);
