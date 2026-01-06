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


        // If the user specified a custom path, we respect it and let it fail naturally if invalid.
        if semgrep_cmd != "semgrep" {
             return Ok(zed::Command {
                command: semgrep_cmd,
                args,
                env,
            });
        }

        Err("semgrep not found! Please install semgrep manually (e.g. `brew install semgrep` or `pip install semgrep`) and ensure it is in your PATH.".into())
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
