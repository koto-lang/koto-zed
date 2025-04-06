use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct KotoZedExtension {}

impl KotoZedExtension {}

impl zed::Extension for KotoZedExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        println!("hello from zed-ext");
        panic!()
    }
}

zed::register_extension!(KotoZedExtension);
