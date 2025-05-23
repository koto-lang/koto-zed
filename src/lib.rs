use std::fs;
use zed::LanguageServerId;
use zed_extension_api::process::Command;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct KotoZedExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for KotoZedExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Command> {
        let command = self.language_server_binary_path(language_server_id, worktree)?;
        println!("starting koto-ls: {}", command); // TEMPORARY
        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

impl KotoZedExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let mut pre_release = false;
        if let Some(settings) = LspSettings::for_worktree("koto-ls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings)
        {
            if let Some(val) = settings.get("pre_release") {
                if let Some(val) = val.as_bool() {
                    pre_release = val;
                }
            }
        }

        if let Some(path) = worktree.which("koto-ls") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "koto-lang/koto-ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "koto-ls-{arch}-{os}.{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-musl",
                zed::Os::Windows => "pc-windows-msvc",
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("koto-ls-{}", release.version);
        let binary_path = format!("{version_dir}/koto-ls");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

zed::register_extension!(KotoZedExtension);
