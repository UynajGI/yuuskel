# yuuskel

✨ **Your Universal Skeleton** — 一键生成结构清晰、环境就绪的通用项目骨架。

> `UynajGI` = Yuunagi 的个人前缀（也谐音 "you" → **为你而生**）
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

`yuuskel` 提供多种安装方式：

---

### ✅ 推荐方式：通过 `cargo` 一键安装（需 Rust）

`yuuskel` 已发布到 [crates.io](https://crates.io/crates/yuuskel)，只需一行命令即可安装：

```bash
cargo install yuuskel
```

> 💡 **前提**：你需要安装 [Rust 工具链](https://rustup.rs/)。
> 若尚未安装，请运行以下命令（适用于 Linux、macOS、WSL）：
>
> ```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> ```
>
> Windows 用户请下载并运行 [rustup-init.exe](https://win.rustup.rs/)，然后按提示操作。

安装完成后，`yuuskel` 命令将自动加入你的 `PATH`，可直接在终端使用：

```bash
yuuskel
```

---

### 📥 方式二：下载预编译二进制文件

前往 [Releases 页面](https://github.com/UynajGI/yuuskel/releases) 下载对应系统的可执行文件：

- **Linux / macOS**: [`yuuskel`](https://github.com/UynajGI/yuuskel/releases/latest/download/yuuskel)
- **Windows**: [`yuuskel.exe`](https://github.com/UynajGI/yuuskel/releases/latest/download/yuuskel.exe)

> 💡 提示：右键链接 → “链接另存为” 即可下载

#### 赋予执行权限（仅 Linux / macOS）

```bash
chmod +x yuuskel
```

#### 添加到系统 PATH

- **Linux / macOS**：
  ```bash
  sudo mv yuuskel /usr/local/bin/
  ```

- **Windows**：
  将 `yuuskel.exe` 复制到 `C:\Windows` 或任意已加入 `PATH` 的目录（如 `C:\Users\<你的用户名>\.cargo\bin`）。

之后即可在任意目录运行：

```bash
yuuskel
```

---

### 🔧 方式三：从源码安装

```bash
git clone https://github.com/UynajGI/yuuskel.git
cd yuuskel
cargo install --path .
```

> 同样需要先安装 [Rust 工具链](https://rustup.rs/)（见上文）。

---

无论你选择哪种方式，安装完成后都可以通过以下命令验证：

```bash
yuuskel --help
```

或直接运行：

```bash
yuuskel
```

---

## 🔧 贡献代码

欢迎任何形式的贡献，包括但不限于：

- 提交 Bug 报告和功能请求
- 提交代码修复（通过 Pull Request）
- 完善文档（如添加使用示例、更新说明）
- 翻译项目到其他语言
- 提交新的项目模板（如 Rust 项目模板）

贡献代码时请遵守 [Rust 社区行为准则](https://www.rust-lang.org/policies/code-of-conduct)。

## 📝 许可协议

`yuuskel` 使用 [MIT 许可协议](https://github.com/UynajGI/yuuskel/main/License) 开源。

## 🤝 联系作者

- 邮箱：[yuunagi@outlook.com](mailto:yuunagi@outlook.com)
- GitHub：[UynajGI](https://github.com/UynajGI)
