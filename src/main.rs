// 引入必要的库
use std::fs;
use std::process;
use std::path::PathBuf;
use dialoguer::{ Input, Select };
use dialoguer::{ theme::ColorfulTheme, Confirm };
use colored::*;

const USAGE_MD: &str = include_str!("USAGE.md");

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

fn main() {
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    if let Err(e) = run() {
        eprintln!("❌ 初始化失败: {}", e.to_string().red());
        process::exit(1);
    }
}

fn run() -> std::io::Result<()> {
    println!("{}", LOGO.green().bold());
    println!("🛠️  {} — 初始化通用项目结构", "yuuskel".cyan().bold());

    // 选择模式
    let selection = Select::new()
        .with_prompt("❓ 初始化方式")
        .item("新建项目文件夹")
        .item("在当前目录初始化")
        .default(0)
        .interact()?;

    let target_dir = if selection == 1 {
        std::env::current_dir()?
    } else {
        let default_name = "my_project";
        let name: String = Input::new()
            .with_prompt("📁 项目文件夹名称")
            .default(default_name.to_string())
            .validate_with(|input: &String| {
                if input.len() > 100 {
                    Err("项目名称过长")
                } else if validate_project_name(input) {
                    Ok(())
                } else {
                    Err("项目名称包含非法字符")
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

    println!("✅ 目标目录: {}", target_dir.display().to_string().cyan());

    if target_dir.exists() && selection == 0 {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("⚠️  目标文件夹已存在，是否继续？")
            .default(false)
            .interact()?;

        if !overwrite {
            println!("❌ {}", "操作已取消".red());
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
                println!("➕ 补充目录: {}", d.yellow());
            }
        }
    }

    // === 询问是否使用环境变量前缀 ===
    let use_prefix = Select::new()
        .with_prompt("🔤 是否为环境变量添加项目前缀？（避免多项目冲突）")
        .item("否（使用通用名称，如 OUTPUT_DIR）")
        .item("是（如 MYPROJ_OUTPUT_DIR）")
        .default(0)
        .interact()?;

    let prefix = if use_prefix == 1 {
        let folder_name = target_dir.file_name().unwrap_or_default().to_string_lossy();
        let default_prefix: String = folder_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
            .collect();

        let prefix_input: String = Input::new()
            .with_prompt("🔤 项目前缀（建议大写，如 MYTOOL）")
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

    // 生成带前缀的 .env 内容
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

    let mut env_content = format!("PROJECT_ROOT={}\n", abs_str);
    for (var_name, dir_path) in &env_vars {
        env_content.push_str(&format!("{}{}={}/{}\n", prefix, var_name, abs_str, dir_path));
    }

    fs::write(target_dir.join(".env"), env_content)?;
    if is_existing {
        println!("🔄 更新: {}", ".env".blue());
    }

    // 覆盖写入 USAGE.md
    fs::write(target_dir.join("USAGE.md"), USAGE_MD)?;
    if is_existing {
        println!("🔄 更新: {}", "USAGE.md".blue());
    }

    // === 动态生成 README.md（仅当不存在时）===
    let readme_path = target_dir.join("README.md");
    if !readme_path.exists() {
        const README_TEMPLATE: &str = include_str!("README.tpl.md");

        let readme_content = README_TEMPLATE.replace(
            "{{output_dir}}",
            &format!("{}/output", abs_str)
        );

        fs::write(&readme_path, readme_content)?;
        println!("➕ 创建: {}", "README.md".green());
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
        if is_existing {
            println!("➕ 创建: {}", ".gitignore".green());
        }
    }

    // 询问是否初始化 Git
    let init_git = Select::new()
        .with_prompt("❓ 是否初始化 Git 仓库？")
        .item("是")
        .item("否")
        .default(1)
        .interact()?;
    let mut git_success = false;
    if init_git == 0 {
        match std::process::Command::new("git").arg("init").current_dir(&target_dir).output() {
            Ok(output) if output.status.success() => {
                println!("📦 {}", "Git 仓库已初始化".green());
                git_success = true;
            }
            Ok(output) => {
                eprintln!("⚠️  Git 初始化失败: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("⚠️  无法执行 git 命令（可能未安装 Git）: {}", e);
            }
        }
    }
    if git_success {
        let add_commit = Confirm::new()
            .with_prompt("💾 是否创建初始提交？")
            .default(true)
            .interact()?;
        if add_commit {
            // git add .
            // git commit -m "chore: initialize project with yuuskel"
            if
                let Ok(output) = std::process::Command
                    ::new("git")
                    .arg("add")
                    .arg(".")
                    .current_dir(&target_dir)
                    .output()
            {
                if !output.status.success() {
                    eprintln!("⚠️  Git 添加失败: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
            if
                let Ok(output) = std::process::Command
                    ::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg("chore: initialize project with yuuskel")
                    .current_dir(&target_dir)
                    .output()
            {
                if !output.status.success() {
                    eprintln!("⚠️  Git 提交失败: {}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    }

    // === 许可证选择（仅新建项目时询问）===
    let license = if !is_existing {
        let license_sel = Select::new()
            .with_prompt("📜 选择开源许可证（可选）")
            .item("跳过（不生成 LICENSE）")
            .item("MIT")
            .item("Apache-2.0")
            .item("GNU AGPLv3")
            .item("GNU GPLv3")
            .item("GNU LGPLv3")
            .item("Mozilla Public License 2.0")
            .item("Boost Software License 1.0")
            .item("Unlicense")
            .item("Proprietary（专有）")
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
            println!("📜 创建: {} ({})", "LICENSE".green(), name.cyan());
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
        // 尝试获取版本（若编译时不可用则回退）
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
        // 为避免引入 chrono 依赖，此处省略时间戳（或可选）
        // 若你已添加 chrono 依赖，可取消注释以下两行
        // let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        // 但为保持轻量，我们暂不记录时间

        let license_value = match &chosen_license {
            Some(name) => format!("\"{}\"", name),
            None => "false".to_string(),
        };

        let prefix_value = if prefix.is_empty() {
            "false".to_string()
        } else {
            format!("\"{}\"", prefix.trim_end_matches('_'))
        };

        // 手动构造 dirs 列表的 TOML 字符串（避免依赖 toml crate）
        let dirs_list = dirs
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        let metadata_content = format!(
            r#"# 由 yuuskel 自动生成，用于记录项目初始化信息
# 请勿手动修改（除非你知道自己在做什么）

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
        println!("\n✅ {}", "项目结构已增量更新！".green().bold());
    } else {
        println!("\n✅ {}", "通用项目初始化完成！".green().bold());
    }
    println!("📄 查看使用指南: {}/{}", target_dir.display(), "USAGE.md".cyan());
    println!("📄 项目入口: {}/{}", target_dir.display(), "README.md".cyan());
    println!("⚙️  环境变量路径: {}/{}", target_dir.display(), ".env".cyan());
    if !prefix.is_empty() {
        println!("🔑 环境变量已添加前缀: {}", prefix.trim_end_matches('_').yellow().bold());
    }
    println!("{}", "💡 提示：在脚本中通过 dotenv 加载路径，避免硬编码！".dimmed());
    Ok(())
}
