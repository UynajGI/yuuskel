# yuuskel

✨ **Your Universal Skeleton** — 一键生成结构清晰、环境就绪的通用项目骨架。

> `yuu` = Yuunagi 的个人前缀（也谐音 "you" → **为你而生**）
>
> `skel` = skeleton（项目骨架，源自 Unix `/etc/skel`）

---

## 🚀 为什么需要 yuuskel？

每次开始一个新项目，你是否都要：

- 手动创建 `input/`, `output/`, `src/`, `scripts/` 等目录？
- 复制 `.gitignore` 和 `.env` 模板？
- 写 README 和 USAGE 说明？
- 担心路径硬编码？

**yuuskel 帮你一键搞定！**
它不是又一个框架脚手架，而是**通用项目结构的初始化器**——适用于数据分析、脚本工程、实验项目、个人工具等任何需要“输入-处理-输出”流程的场景。

---

## 🌟 核心特性

- **交互式引导**：无需记忆命令，按提示操作即可
- **标准目录结构**：

  ```plaintext
  project/
  ├── input/          # 原始数据
  ├── output/         # 生成结果
  ├── assets/         # 静态资源
  ├── src/            # 源代码
  ├── scripts/        # 脚本
  ├── configs/        # 配置文件
  ├── docs/           # 文档
  ├── notebooks/      # Jupyter 笔记本
  ├── .env            # 环境变量（含绝对路径）
  ├── USAGE.md        # 使用指南
  └── README.md       # 项目入口
  ```

- **智能环境变量**：
  - 自动生成 `.env` 文件，包含所有目录的**绝对路径**
  - 支持项目前缀（如 `MYPROJ_OUTPUT_DIR`），避免多项目冲突
- **增量安全更新**：已在存在的项目可安全补充缺失结构
- **可选 Git 初始化 + 初始提交**
- **可选开源许可证**（MIT, Apache-2.0, GPL 等）
- **彩色终端 + Emoji 引导**：清晰直观
- **轻量无依赖**：仅需 `dialoguer` 和 `colored`

---

## 📦 安装

### 1. 下载可执行文件

前往 [Releases 页面](https://github.com/yuu/yuuskel/releases) 下载对应系统的文件：

- **Linux / macOS**: [`yuuskel`](https://github.com/yuu/yuuskel/releases/latest/download/yuuskel)
- **Windows**: [`yuuskel.exe`](https://github.com/yuu/yuuskel/releases/latest/download/yuuskel.exe)

> 💡 提示：右键链接 → “链接另存为” 即可下载

### 2. 赋予执行权限（仅 Linux/macOS）

```bash
chmod +x yuuskel
```

### 3. 移动到系统 PATH（可选但推荐）

```bash
# Linux/macOS
sudo mv yuuskel /usr/local/bin/

# Windows
# 将 yuuskel.exe 复制到 C:\Windows 或任意 PATH 目录
```

现在你就可以在终端直接运行：

```bash
yuuskel
```

> 需要 [Rust 工具链](https://rustup.rs/)（运行 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 安装）

### 🔧 备选：从源码安装（需要 Rust）

```bash
git clone https://github.com/yuu/yuuskel.git
cd yuuskel
cargo install --path .
```

> 需要 [Rust 工具链](https://rustup.rs/)（运行 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 安装）

---

## 🖼️ 使用示例

```bash
$ yuuskel

                        _        _
  _   _ _   _ _   _ ___| | _____| |
 | | | | | | | | | / __| |/ / _ \ |
 | |_| | |_| | |_| \__ \   <  __/ |
  \__, |\__,_|\__,_|___/_|\_\___|_|
  |___/

🛠️  yuuskel — 初始化通用项目结构

❓ 初始化方式
❯ 新建项目文件夹
  在当前目录初始化

📁 项目文件夹名称 [my_project]: my_analysis
✅ 目标目录: /home/user/my_analysis
🔤 是否为环境变量添加项目前缀？（避免多项目冲突）
❯ 否（使用通用名称，如 OUTPUT_DIR）
  是（如 MYPROJ_OUTPUT_DIR）

...

✅ 通用项目初始化完成！
📄 查看使用指南: /home/user/my_analysis/USAGE.md
📄 项目入口: /home/user/my_analysis/README.md
⚙️  环境变量路径: /home/user/my_analysis/.env
💡 提示：在脚本中通过 dotenv
