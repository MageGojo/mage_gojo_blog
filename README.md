# 布努瑶文化轻量 SSR 博客

一个基于 **Rust** 的轻量级 SSR 博客项目，目标是以工程化、可长期维护的方式记录与展示 **布努瑶文化**相关内容。项目强调：结构清晰、依赖克制、可直接部署、可渐进扩展。

本项目并非通用 CMS，而是一个**可作为个人文化博客或技术博客基础骨架**的示例实现。

---

## 项目目标

* 使用 **Rust + Dioxus SSR** 实现服务端渲染（SEO 友好）
* 保持前端与后端在同一语言生态中，避免复杂前后端分离
* 前端样式采用 **Tailwind CSS**，构建流程尽量轻量（基于 Deno）
* 代码结构清晰，便于后续扩展为 Markdown 博客或轻量 CMS

---

## 技术栈

* **后端 / SSR**

  * Rust
  * Actix-web
  * Dioxus（`dioxus-ssr`）

* **前端样式与脚本**

  * Tailwind CSS（通过 Deno 任务或 npx）
  * TypeScript（`main.ts`，用于简单交互）

* **构建与工具**

  * Cargo
  * Deno（支持 `npm:` 导入）

---

## 项目结构说明

```
.
├── src/
│   ├── main.rs              # 程序入口，Actix 启动与配置
│   ├── routers/
│   │   └── route.rs         # 路由定义（Actix handlers）
│   ├── layouts/
│   │   └── default.rs       # 页面通用 HTML Layout
│   ├── components/          # 可复用 Dioxus UI 组件
│   ├── views/               # 页面级视图（Home / Post 等）
│   └── utils/               # 辅助类型与工具（语言、配置等）
│
├── assets/
│   ├── output.css           # Tailwind 编译后的 CSS
│   ├── main.js              # 前端交互脚本（由 main.ts 构建）
│   └── images/              # 图片、字体等静态资源
│
├── input.css                # Tailwind CSS 入口文件
├── main.ts                  # 前端 TypeScript 脚本
├── Cargo.toml
├── deno.json
└── README.md
```

---

## 本地开发

### 1. 克隆项目

```bash
git clone <your-repo-url>
cd <project-name>
```

---

### 2. 构建 / 监听 Tailwind CSS

#### 使用 Deno（推荐）

> 需要 Deno 支持 `npm:` 导入

```bash
deno task tw:watch
```

#### 或使用 Node / npm

```bash
# 一次性构建
npx tailwindcss -i ./input.css -o ./assets/output.css --minify

# 或监听修改
npx tailwindcss -i ./input.css -o ./assets/output.css --watch
```

> 说明：仓库中**已包含生成后的 `assets/output.css`**，因此首次运行服务时并不强制要求先构建 CSS；但如需修改样式或 Tailwind 配置，建议开启 watch。

---

### 3. 启动服务器

```bash
# 默认监听 127.0.0.1:8888
cargo run

# 指定 host / port
cargo run -- --host 0.0.0.0 --port 8080
```

启动成功后可访问：

* [http://127.0.0.1:8888](http://127.0.0.1:8888)

---

## 代码要点说明

### `src/main.rs`

* 使用 `clap` 解析 CLI 参数（host / port）
* 初始化 `env_logger`
* 注册：

  * 页面路由（如 `home_route`）
  * 静态资源服务：`/assets` → `./assets`

---

### `src/routers/route.rs`

* 使用 Actix handler 定义路由
* 通过：

```rust
dioxus_ssr::render_element(HomeView())
```

将 Dioxus 组件渲染为 HTML 字符串并直接返回给客户端

---

### `src/layouts/default.rs`

* 定义统一 HTML 页面结构（`<html>` / `<head>` / `<body>`）
* 注入：

  * `/assets/output.css`
  * `/assets/main.js`
* 支持通过 `PageLang` 设置 `lang` 属性

---

### `src/components/*`

* 所有 UI 区块均为 Dioxus 组件
* 便于复用、组合与后期拆分
* 示例：

  * 页头组件：大图背景 + 打字机占位文本
  * 主内容组件：文章卡片、侧边栏小部件

---

### `main.ts`

* 页面加载完成后执行
* 提供：

  * 打字机文字效果（`#typewriter-text`）
  * 滚动到内容区域的辅助函数（`scrollToContent()`）

---

## 如何添加新页面 / 内容

1. 在 `src/views/` 中新增视图文件（如 `post.rs`）
2. 使用 Dioxus 编写页面组件
3. 在 `src/routers/` 中注册对应路由
4. 如需新样式，修改 `input.css` 并重新构建 `output.css`

---

## 打包与部署建议

* 生产环境建议：

```bash
cargo build --release
```

* 部署时需要：

  * release 可执行文件
  * `assets/` 目录（CSS / JS / 图片等）

* 可选部署方式：

  * systemd
  * Docker
  * 任意进程管理器（pm2 / supervisor 等）

* 建议在生产环境前置：

  * Nginx / Caddy（HTTPS + 反向代理）

---

## 常见问题

### 页面无样式 / 样式不生效？

* 确认 `assets/output.css` 是否存在
* 确认 `default.rs` 中引用路径为 `/assets/output.css`
* 确认 Actix 静态文件服务路径配置正确

---

### 如何修改页头打字机文字？

* 修改 `main.ts` 中的 `texts` 数组
* 或调整 `headers_component.rs` 中对应元素的 id / class

---

## 扩展建议

* 引入 Markdown 文章加载器（`content/*.md` → HTML）
* 增加缓存 / gzip 中间件提升性能
* 容器化部署（Docker）
* 后期可扩展为轻量 CMS 或多语言博客

---

## 许可证

本项目当前为示例与个人作品展示用途。

如需公开发布或二次分发，请补充合适的开源许可证（如 **MIT** 或 **Apache-2.0**）。

---

## 致谢

感谢以下优秀的开源项目：

* Actix-web
* Dioxus
* Tailwind CSS
* Rust 社区
