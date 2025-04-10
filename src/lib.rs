use std::fs;
//use zed::lsp::{Completion, CompletionKind};
use zed::{/*CodeLabel, CodeLabelSpan,*/ LanguageServerId};
use zed_extension_api::process::Command;
use zed_extension_api::{self as zed, Result};

struct KotoZedExtension {
    cached_binary_path: Option<String>,
}

impl KotoZedExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
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
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "koto-ls-{version}-{arch}-{os}.tar.gz",
            version = release.version,
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
                zed::DownloadedFileType::GzipTar,
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
        println!("hello from language_server_command");
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["lsp".to_string()], // TODO: are there args for kptp-ls ?
            env: Default::default(),
        })
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // TODO
        println!("here comes completion {:?}", completion);
        None
    }
}

zed::register_extension!(KotoZedExtension);

// fn label_for_completion(
//         &self,
//         _language_server_id: &zed::LanguageServerId,
//         completion: zed::lsp::Completion,
//     ) -> Option<CodeLabel> {
//         let arrow = " → ";

//         match completion.kind? {
//             CompletionKind::Class => Some(CodeLabel {
//                 filter_range: (0..completion.label.len()).into(),
//                 spans: vec![CodeLabelSpan::literal(
//                     completion.label,
//                     Some("type".into()),
//                 )],
//                 code: String::new(),
//             }),
//             CompletionKind::Function | CompletionKind::Constructor | CompletionKind::Method => {
//                 let mut parts = completion.detail.as_ref()?.split(arrow);
//                 let (name, _) = completion.label.split_once('(')?;
//                 let parameter_list = parts.next()?;
//                 let return_type = parts.next()?;
//                 let fn_name = " a";
//                 let fat_arrow = " => ";
//                 let call_expr = "();";

//                 let code =
//                     format!("{return_type}{fn_name}{parameter_list}{fat_arrow}{name}{call_expr}");

//                 let parameter_list_start = return_type.len() + fn_name.len();

//                 Some(CodeLabel {
//                     spans: vec![
//                         CodeLabelSpan::code_range(
//                             code.len() - call_expr.len() - name.len()..code.len() - call_expr.len(),
//                         ),
//                         CodeLabelSpan::code_range(
//                             parameter_list_start..parameter_list_start + parameter_list.len(),
//                         ),
//                         CodeLabelSpan::literal(arrow, None),
//                         CodeLabelSpan::code_range(0..return_type.len()),
//                     ],
//                     filter_range: (0..name.len()).into(),
//                     code,
//                 })
//             }
//             CompletionKind::Property => {
//                 let class_start = "class A {";
//                 let get = " get ";
//                 let property_end = " => a; }";
//                 let ty = completion.detail?;
//                 let name = completion.label;

//                 let code = format!("{class_start}{ty}{get}{name}{property_end}");
//                 let name_start = class_start.len() + ty.len() + get.len();

//                 Some(CodeLabel {
//                     spans: vec![
//                         CodeLabelSpan::code_range(name_start..name_start + name.len()),
//                         CodeLabelSpan::literal(arrow, None),
//                         CodeLabelSpan::code_range(class_start.len()..class_start.len() + ty.len()),
//                     ],
//                     filter_range: (0..name.len()).into(),
//                     code,
//                 })
//             }
//             CompletionKind::Variable => {
//                 let name = completion.label;

//                 Some(CodeLabel {
//                     filter_range: (0..name.len()).into(),
//                     spans: vec![CodeLabelSpan::literal(name, Some("variable".into()))],
//                     code: String::new(),
//                 })
//             }
//             _ => None,
//         }
//     }
// }
