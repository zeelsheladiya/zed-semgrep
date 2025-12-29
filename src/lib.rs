use zed_extension_api::{self as zed, Result};

struct SemgrepExtension;

impl zed::Extension for SemgrepExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("semgrep")
            .ok_or_else(|| "Semgrep not found. Please install Semgrep and ensure it is in your PATH.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({ "scan": {} })))
    }
}

zed::register_extension!(SemgrepExtension);
