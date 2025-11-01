// 引入必要的库
use std::fs;
use std::process;
use std::path::PathBuf;
use dialoguer::{ Input, Select };
use dialoguer::{ theme::ColorfulTheme, Confirm };
use colored::*;

const LOGO: &str =
    r##"
                        _        _
  _   _ _   _ _   _ ___| | _____| |
 | | | | | | | | | / __| |/ / _ \ |
 | |_| | |_| | |_| \__ \   <  __/ |
  \__, |\__,_|\__,_|___/_|\_\___|_|
  |___/
"##;

fn validate_project_name(name: &str) -> bool {
    !name.is_empty() &&
        !name.contains(std::path::MAIN_SEPARATOR) &&
        !name.contains('/') &&
        !name.contains('\\') &&
        name != "." &&
        name != ".."
}

#[derive(Clone, Copy)]
enum Language {
    En,
    Zh,
}

impl Language {
    fn all() -> Vec<(&'static str, Self)> {
        vec![("English", Language::En), ("中文", Language::Zh)]
    }

    fn msg(&self, key: MsgKey) -> &'static str {
        match (self, key) {
            // Logo 和固定字符串可复用
            (_, MsgKey::UsageHelp) =>
                "yuuskel — Initialize standardized project structure\n\nUsage: yuuskel",

            // 中文 vs 英文
            (Language::Zh, MsgKey::Title) => "🛠️  yuuskel — 初始化通用项目结构",
            (Language::En, MsgKey::Title) =>
                "🛠️  yuuskel — Initialize standardized project structure",

            (Language::Zh, MsgKey::InitModePrompt) => "❓ 初始化方式",
            (Language::En, MsgKey::InitModePrompt) => "❓ Initialization mode",

            (Language::Zh, MsgKey::NewItemProject) => "新建项目文件夹",
            (Language::En, MsgKey::NewItemProject) => "Create new project folder",

            (Language::Zh, MsgKey::InitInCurrent) => "在当前目录初始化",
            (Language::En, MsgKey::InitInCurrent) => "Initialize in current directory",

            (Language::Zh, MsgKey::ProjectNamePrompt) => "📁 项目文件夹名称",
            (Language::En, MsgKey::ProjectNamePrompt) => "📁 Project folder name",

            (Language::Zh, MsgKey::NameTooLong) => "项目名称过长（最大支持100个字符）",
            (Language::En, MsgKey::NameTooLong) => "Project name too long (max 100 characters)",

            (Language::Zh, MsgKey::InvalidChars) => "项目名称包含非法字符",
            (Language::En, MsgKey::InvalidChars) => "Project name contains invalid characters",

            (Language::Zh, MsgKey::TargetDir) => "✅ 目标目录: ",
            (Language::En, MsgKey::TargetDir) => "✅ Target directory: ",

            (Language::Zh, MsgKey::DirExistsPrompt) => "⚠️  目标文件夹已存在，是否继续？",
            (Language::En, MsgKey::DirExistsPrompt) =>
                "⚠️  Target folder already exists. Continue?",

            (Language::Zh, MsgKey::Cancelled) => "❌ 操作已取消",
            (Language::En, MsgKey::Cancelled) => "❌ Operation cancelled",

            (Language::Zh, MsgKey::CreateDir) => "➕ 创建目录: ",
            (Language::En, MsgKey::CreateDir) => "➕ Creating directory: ",

            (Language::Zh, MsgKey::AddDir) => "➕ 补充目录: ",
            (Language::En, MsgKey::AddDir) => "➕ Adding missing directory: ",

            (Language::Zh, MsgKey::EnvPrefixPrompt) =>
                "🔤 是否为环境变量添加项目前缀？（避免多项目冲突）",
            (Language::En, MsgKey::EnvPrefixPrompt) =>
                "🔤 Add prefix to env vars? (Avoid conflicts across projects)",

            (Language::Zh, MsgKey::NoPrefix) => "否（使用通用名称，如 OUTPUT_DIR）",
            (Language::En, MsgKey::NoPrefix) => "No (use generic names like OUTPUT_DIR)",

            (Language::Zh, MsgKey::WithPrefix) => "是（如 MYPROJ_OUTPUT_DIR）",
            (Language::En, MsgKey::WithPrefix) => "Yes (e.g., MYPROJ_OUTPUT_DIR)",

            (Language::Zh, MsgKey::PrefixPrompt) => "🔤 项目前缀（建议大写，如 MYTOOL）",
            (Language::En, MsgKey::PrefixPrompt) =>
                "🔤 Project prefix (uppercase recommended, e.g., MYTOOL)",

            (Language::Zh, MsgKey::UpdateDotEnv) => "🔄 更新: ",
            (Language::En, MsgKey::UpdateDotEnv) => "🔄 Updating: ",

            (Language::Zh, MsgKey::SkipUsageMd) => "ℹ️  USAGE.md 已存在，跳过更新",
            (Language::En, MsgKey::SkipUsageMd) => "ℹ️  USAGE.md already exists, skipping update",

            (Language::Zh, MsgKey::GitInitPrompt) => "❓ 是否初始化 Git 仓库？",
            (Language::En, MsgKey::GitInitPrompt) => "❓ Initialize Git repository?",

            (Language::Zh, MsgKey::Yes) => "是",
            (Language::En, MsgKey::Yes) => "Yes",

            (Language::Zh, MsgKey::No) => "否",
            (Language::En, MsgKey::No) => "No",

            (Language::Zh, MsgKey::GitInitialized) => "📦 Git 仓库已初始化",
            (Language::En, MsgKey::GitInitialized) => "📦 Git repository initialized",

            (Language::Zh, MsgKey::GitConfigMissing) =>
                "⚠️  Git 用户信息未配置，跳过初始提交\n💡 运行以下命令设置：\n  git config --global user.name \"Your Name\"\n  git config --global user.email \"you@example.com\"",
            (Language::En, MsgKey::GitConfigMissing) =>
                "⚠️  Git user info not configured, skipping initial commit\n💡 Run these commands:\n  git config --global user.name \"Your Name\"\n  git config --global user.email \"you@example.com\"",

            (Language::Zh, MsgKey::InitialCommitPrompt) => "💾 是否创建初始提交？",
            (Language::En, MsgKey::InitialCommitPrompt) => "💾 Create initial commit?",

            (Language::Zh, MsgKey::GitAddFailed) =>
                "⚠️  Git 添加失败: {}\n💡 建议检查：1. 工作区文件权限 2. Git 配置（user.name/user.email）",
            (Language::En, MsgKey::GitAddFailed) =>
                "⚠️  Git add failed: {}\n💡 Check: 1. File permissions 2. Git config (user.name/user.email)",

            (Language::Zh, MsgKey::GitCommitFailed) => "⚠️  Git 提交失败: {}",
            (Language::En, MsgKey::GitCommitFailed) => "⚠️  Git commit failed: {}",

            (Language::Zh, MsgKey::CommitSuccess) => "💾 初始提交创建成功",
            (Language::En, MsgKey::CommitSuccess) => "💾 Initial commit created successfully",

            (Language::Zh, MsgKey::LicensePrompt) => "📜 选择开源许可证（可选）",
            (Language::En, MsgKey::LicensePrompt) => "📜 Choose an open-source license (optional)",

            (Language::Zh, MsgKey::SkipLicense) => "跳过（不生成 LICENSE）",
            (Language::En, MsgKey::SkipLicense) => "Skip (do not generate LICENSE)",

            (Language::Zh, MsgKey::Proprietary) => "Proprietary（专有）",
            (Language::En, MsgKey::Proprietary) => "Proprietary",

            (Language::Zh, MsgKey::IncrementalUpdateDone) => "✅ 项目结构已增量更新！",
            (Language::En, MsgKey::IncrementalUpdateDone) =>
                "✅ Project structure incrementally updated!",

            (Language::Zh, MsgKey::InitDone) => "✅ 通用项目初始化完成！",
            (Language::En, MsgKey::InitDone) => "✅ Standardized project initialized!",

            (Language::Zh, MsgKey::GuidePath) => "📄 查看使用指南: {}/{}",
            (Language::En, MsgKey::GuidePath) => "📄 Usage guide: {}/{}",

            (Language::Zh, MsgKey::ReadmePath) => "📄 项目入口: {}/{}",
            (Language::En, MsgKey::ReadmePath) => "📄 Project entry: {}/{}",

            (Language::Zh, MsgKey::EnvPath) => "⚙️  环境变量路径: {}/{}",
            (Language::En, MsgKey::EnvPath) => "⚙️  Env file path: {}/{}",

            (Language::Zh, MsgKey::PrefixAdded) => "🔑 环境变量已添加前缀: ",
            (Language::En, MsgKey::PrefixAdded) => "🔑 Env vars prefixed with: ",

            (Language::Zh, MsgKey::DotenvTip) =>
                "💡 提示：在脚本中通过 dotenv 加载路径，避免硬编码！",
            (Language::En, MsgKey::DotenvTip) =>
                "💡 Tip: Load paths via dotenv in scripts to avoid hardcoding!",
        }
    }
}

#[derive(Clone, Copy)]
enum MsgKey {
    UsageHelp,
    Title,
    InitModePrompt,
    NewItemProject,
    InitInCurrent,
    ProjectNamePrompt,
    NameTooLong,
    InvalidChars,
    TargetDir,
    DirExistsPrompt,
    Cancelled,
    CreateDir,
    AddDir,
    EnvPrefixPrompt,
    NoPrefix,
    WithPrefix,
    PrefixPrompt,
    UpdateDotEnv,
    SkipUsageMd,
    GitInitPrompt,
    Yes,
    No,
    GitInitialized,
    GitConfigMissing,
    InitialCommitPrompt,
    GitAddFailed,
    GitCommitFailed,
    CommitSuccess,
    LicensePrompt,
    SkipLicense,
    Proprietary,
    IncrementalUpdateDone,
    InitDone,
    GuidePath,
    ReadmePath,
    EnvPath,
    PrefixAdded,
    DotenvTip,
}

fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "--version" => {
                println!("yuuskel {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--help" => {
                // Help 用英文（国际惯例）
                println!("{}", Language::En.msg(MsgKey::UsageHelp));
                return;
            }
            _ => {}
        }
    }

    println!("{}", LOGO.green().bold());
    // 👇 第一步：选择语言
    let lang_options = Language::all();
    let lang_selection = Select::new()
        .with_prompt("🌐 Select your language")
        .items(
            &lang_options
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
        )
        .default(0) // 默认 English
        .interact()
        .unwrap_or(0); // 安全兜底

    let lang = lang_options[lang_selection].1;

    if let Err(e) = run(lang) {
        // 错误信息也用所选语言
        let error_msg = match lang {
            Language::En => format!("❌ Initialization failed: {}", e),
            Language::Zh => format!("❌ 初始化失败: {}", e),
        };
        eprintln!("{}", error_msg.red());
        process::exit(1);
    }
}
fn run(lang: Language) -> std::io::Result<()> {
    println!("{}", lang.msg(MsgKey::Title).cyan().bold());

    // 选择模式
    let selection = Select::new()
        .with_prompt(lang.msg(MsgKey::InitModePrompt))
        .item(lang.msg(MsgKey::NewItemProject))
        .item(lang.msg(MsgKey::InitInCurrent))
        .default(0)
        .interact()?;

    let target_dir = if selection == 1 {
        std::env::current_dir()?
    } else {
        let default_name = "my_project";
        let name: String = Input::new()
            .with_prompt(lang.msg(MsgKey::ProjectNamePrompt))
            .default(default_name.to_string())
            .validate_with(|input: &String| {
                if input.len() > 100 {
                    Err(lang.msg(MsgKey::NameTooLong))
                } else if validate_project_name(input) {
                    Ok(())
                } else {
                    Err(lang.msg(MsgKey::InvalidChars))
                }
            })
            .interact_text()?;

        let path = PathBuf::from(&name);
        if path.is_absolute() {
            path
        } else {
            std::env::current_dir()?.join(name)
        }
    };

    println!("{}{}", lang.msg(MsgKey::TargetDir), target_dir.display().to_string().cyan());

    if target_dir.exists() && selection == 0 {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(lang.msg(MsgKey::DirExistsPrompt))
            .default(false)
            .interact()?;

        if !overwrite {
            println!("❌ {}", lang.msg(MsgKey::Cancelled).red());
            return Ok(());
        }
    }

    let is_existing = target_dir.exists();

    // 通用项目目录结构
    let dirs = ["input", "output", "assets/temp", "src", "scripts", "configs", "docs", "notebooks"];

    // 创建缺失的目录（增量安全）
    for &d in &dirs {
        let path = target_dir.join(d);
        if !path.exists() {
            fs::create_dir_all(&path)?;
            if is_existing {
                println!("{}{}", lang.msg(MsgKey::AddDir), d.yellow());
            } else {
                println!("{}{}", lang.msg(MsgKey::CreateDir), d.green());
            }
        }
    }

    // === 询问是否使用环境变量前缀 ===
    let use_prefix = Select::new()
        .with_prompt(lang.msg(MsgKey::EnvPrefixPrompt))
        .item(lang.msg(MsgKey::NoPrefix))
        .item(lang.msg(MsgKey::WithPrefix))
        .default(0)
        .interact()?;

    let prefix = if use_prefix == 1 {
        let folder_name = target_dir.file_name().unwrap_or_default().to_string_lossy();
        let default_prefix: String = folder_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
            .collect();

        let prefix_input: String = Input::new()
            .with_prompt(lang.msg(MsgKey::PrefixPrompt))
            .default(default_prefix)
            .interact_text()?;
        format!("{}_", prefix_input.to_uppercase())
    } else {
        String::new()
    };

    // 先确保目录存在（仅新建模式）
    if selection == 0 {
        fs::create_dir_all(&target_dir)?;
    }
    // 再 canonicalize（此时目录一定存在）
    let abs_path = target_dir.canonicalize().unwrap_or_else(|_| target_dir.clone());
    let abs_str = abs_path.to_string_lossy().replace('\\', "/");

    // === 安全增量更新 .env（保留用户自定义内容）===
    let env_path = target_dir.join(".env");
    let env_vars = [
        ("INPUT_DIR", "input"),
        ("OUTPUT_DIR", "output"),
        ("ASSETS_DIR", "assets"),
        ("TEMP_ASSETS_DIR", "assets/temp"),
        ("SRC_DIR", "src"),
        ("SCRIPTS_DIR", "scripts"),
        ("CONFIGS_DIR", "configs"),
        ("DOCS_DIR", "docs"),
        ("NOTEBOOKS_DIR", "notebooks"),
    ];

    // 读取现有内容（如果存在）
    let existing_content = fs::read_to_string(&env_path).unwrap_or_default();
    let mut lines: Vec<String> = existing_content
        .lines()
        .map(|s| s.to_string())
        .collect();

    // 构建需要更新的键集合（用于识别哪些行要替换）
    let managed_keys: std::collections::HashSet<String> = std::iter
        ::once("PROJECT_ROOT".to_string())
        .chain(env_vars.iter().map(|(k, _)| format!("{}{}", prefix, k)))
        .collect();

    // 过滤掉已存在的 managed keys（避免重复）
    lines.retain(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return true; // 保留空行和注释
        }
        // 检查是否是 managed key（格式：KEY=...）
        if let Some(eq_pos) = trimmed.find('=') {
            let key = &trimmed[..eq_pos];
            !managed_keys.contains(key)
        } else {
            true // 无效行也保留（用户可能有特殊内容）
        }
    });

    // 添加新的 managed 变量（PROJECT_ROOT + 所有 _DIR）
    lines.push(format!("PROJECT_ROOT=\"{}\"", abs_str));
    for (var_name, dir_path) in &env_vars {
        lines.push(format!("{}{}=\"{}/{}\"", prefix, var_name, abs_str, dir_path));
    }

    // 写回文件
    fs::write(&env_path, lines.join("\n") + "\n")?;
    if is_existing {
        println!("{}{}", lang.msg(MsgKey::UpdateDotEnv), ".env".blue());
    } else {
        println!("➕ {}", ".env".green());
    }

    // === 写入 USAGE.md（根据语言）===
    let usage_path = target_dir.join("USAGE.md");
    if !usage_path.exists() {
        let usage_content = match lang {
            Language::En => include_str!("docs/usage.en.md"),
            Language::Zh => include_str!("docs/usage.zh.md"),
        };
        fs::write(usage_path, usage_content)?;
        println!("➕ {}", "USAGE.md".green());
    } else if is_existing {
        println!("{}", lang.msg(MsgKey::SkipUsageMd).blue());
    }

    // === 动态生成 README.md（根据语言）===
    let readme_path = target_dir.join("README.md");
    if !readme_path.exists() {
        let readme_template = match lang {
            Language::En => include_str!("docs/readme.en.md"),
            Language::Zh => include_str!("docs/readme.zh.md"),
        };

        let output_dir_path = format!("{}/output", abs_str);
        let readme_content = readme_template.replace("{{output_dir}}", &output_dir_path);

        fs::write(&readme_path, readme_content)?;
        println!("➕ {}", "README.md".green());
    }

    // 仅当 .gitignore 不存在时创建
    let gitignore_path = target_dir.join(".gitignore");
    if !gitignore_path.exists() {
        let gitignore =
            r#"# Local config
.env

# Inputs & outputs (often large or sensitive)
input/
output/
assets/

# Temp & caches
*.tmp
*.log
__pycache__/
*.pyc
.ipynb_checkpoints/
.DS_Store
build/
dist/
"#;
        fs::write(&gitignore_path, gitignore)?;
        println!("➕ {}", ".gitignore".green());
    }

    // 询问是否初始化 Git
    let init_git = Select::new()
        .with_prompt(lang.msg(MsgKey::GitInitPrompt))
        .item(lang.msg(MsgKey::Yes))
        .item(lang.msg(MsgKey::No))
        .default(1)
        .interact()?;
    let mut git_success = false;
    if init_git == 0 {
        match std::process::Command::new("git").arg("init").current_dir(&target_dir).output() {
            Ok(output) if output.status.success() => {
                println!("📦 {}", lang.msg(MsgKey::GitInitialized).green());
                git_success = true;
            }
            Ok(output) => {
                eprintln!("⚠️  Git init failed: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("⚠️  Failed to run git (is Git installed?): {}", e);
            }
        }
    }

    // 检查 git config
    let has_user = std::process::Command
        ::new("git")
        .args(["config", "user.name"])
        .current_dir(&target_dir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let has_email = std::process::Command
        ::new("git")
        .args(["config", "user.email"])
        .current_dir(&target_dir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !has_user || !has_email {
        eprintln!("{}", lang.msg(MsgKey::GitConfigMissing));
    } else {
        if git_success {
            let add_commit = Confirm::new()
                .with_prompt(lang.msg(MsgKey::InitialCommitPrompt))
                .default(true)
                .interact()?;
            if add_commit {
                if
                    let Ok(output) = std::process::Command
                        ::new("git")
                        .arg("add")
                        .arg(".")
                        .current_dir(&target_dir)
                        .output()
                {
                    if !output.status.success() {
                        let msg = lang
                            .msg(MsgKey::GitAddFailed)
                            .replace("{}", &String::from_utf8_lossy(&output.stderr));
                        eprintln!("{}", msg);
                    }
                }
                if
                    let Ok(output) = std::process::Command
                        ::new("git")
                        .args(["commit", "-m", "chore: initialize project with yuuskel"])
                        .current_dir(&target_dir)
                        .output()
                {
                    if output.status.success() {
                        println!("💾 {}", lang.msg(MsgKey::CommitSuccess));
                    } else {
                        let msg = lang
                            .msg(MsgKey::GitCommitFailed)
                            .replace("{}", &String::from_utf8_lossy(&output.stderr));
                        eprintln!("{}", msg);
                    }
                }
            }
        }
    }

    // === 许可证选择（仅新建项目时询问）===
    let license = if !is_existing {
        let license_sel = Select::new()
            .with_prompt(lang.msg(MsgKey::LicensePrompt))
            .item(lang.msg(MsgKey::SkipLicense))
            .item("MIT")
            .item("Apache-2.0")
            .item("GNU AGPLv3")
            .item("GNU GPLv3")
            .item("GNU LGPLv3")
            .item("Mozilla Public License 2.0")
            .item("Boost Software License 1.0")
            .item("Unlicense")
            .item(lang.msg(MsgKey::Proprietary))
            .default(0)
            .interact()?;

        match license_sel {
            0 => None,
            1 => Some(("MIT", include_str!("licenses/mit"))),
            2 => Some(("Apache-2.0", include_str!("licenses/apache-2.0"))),
            3 => Some(("AGPL-3.0", include_str!("licenses/agpl-3.0"))),
            4 => Some(("GPL-3.0", include_str!("licenses/gpl-3.0"))),
            5 => Some(("LGPL-3.0", include_str!("licenses/lgpl-3.0"))),
            6 => Some(("MPL-2.0", include_str!("licenses/mpl-2.0"))),
            7 => Some(("BSL-1.0", include_str!("licenses/bsl-1.0"))),
            8 => Some(("Unlicense", include_str!("licenses/unlicense"))),
            9 => Some(("Proprietary", "All rights reserved.\n")),
            _ => None,
        }
    } else {
        None
    };

    let chosen_license = if let Some((name, text)) = license {
        let license_path = target_dir.join("LICENSE");
        if !license_path.exists() {
            fs::write(&license_path, text)?;
            println!("{} {} ({})", "📜", "LICENSE".green(), name.cyan());
            Some(name)
        } else {
            Some(name)
        }
    } else {
        None
    };

    // === 记录元数据到 yuuskel.toml（静默，仅首次创建）===
    let metadata_path = target_dir.join("yuuskel.toml");
    if !metadata_path.exists() {
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
        let license_value = match &chosen_license {
            Some(name) => format!("\"{}\"", name),
            None => "false".to_string(),
        };
        let prefix_value = if prefix.is_empty() {
            "false".to_string()
        } else {
            format!("\"{}\"", prefix.trim_end_matches('_'))
        };
        let dirs_list = dirs
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        let metadata_content = format!(
            r#"# Generated by yuuskel — do not edit manually unless you know what you're doing

[yuuskel]
version = "{version}"
prefix = {prefix_value}
git_initialized = {git_success}
license = {license_value}
dirs = [{dirs_list}]
"#,
            version = version,
            prefix_value = prefix_value,
            git_success = git_success,
            license_value = license_value,
            dirs_list = dirs_list
        );

        fs::write(&metadata_path, metadata_content).ok(); // 静默失败
    }

    if is_existing {
        println!("\n✅ {}", lang.msg(MsgKey::IncrementalUpdateDone).green().bold());
    } else {
        println!("\n✅ {}", lang.msg(MsgKey::InitDone).green().bold());
    }
    // 处理 GuidePath 消息（手动替换两个占位符）
    let guide_path_msg = lang.msg(MsgKey::GuidePath);
    let guide_path_output = guide_path_msg
        .replace("{}", &target_dir.display().to_string())
        .replace("{}", &"USAGE.md".cyan().to_string());
    println!("{}", guide_path_output);

    // 处理 ReadmePath 消息
    let readme_path_msg = lang.msg(MsgKey::ReadmePath);
    let readme_path_output = readme_path_msg
        .replace("{}", &target_dir.display().to_string())
        .replace("{}", &"README.md".cyan().to_string());
    println!("{}", readme_path_output);

    // 处理 EnvPath 消息
    let env_path_msg = lang.msg(MsgKey::EnvPath);
    let env_path_output = env_path_msg
        .replace("{}", &target_dir.display().to_string())
        .replace("{}", &".env".cyan().to_string());
    println!("{}", env_path_output);

    if !prefix.is_empty() {
        println!(
            "{}{}",
            lang.msg(MsgKey::PrefixAdded),
            prefix.trim_end_matches('_').yellow().bold()
        );
    }
    println!("{}", lang.msg(MsgKey::DotenvTip).dimmed());

    Ok(())
}
