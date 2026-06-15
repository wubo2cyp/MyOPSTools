# 轻量级通用 Agent 平台

> 一个轻量级、可扩展的通用 Agent 平台。前端 TypeScript + React，后端 Rust + Axum，遵循 **SDD（Specification-Driven Development）** 流程。
>
> 当前已完成的里程碑：**M1 - M6**。规范与计划见 [`.trae/documents/`](./.trae/documents/)。

## ✨ 功能一览

- 💬 **多轮对话**：会话与消息全部持久化到 SQLite，可随时回看。
- 🤖 **Tool-Use Agent**：内置 `echo` / `get_current_time` / `http_get` 三个工具，支持 `think → tool → think` 闭环。
- 🧠 **模型可插拔**：`ModelProvider` trait 内置 `MockProvider`（离线开发）与 `OpenAIProvider`（兼容 OpenAI 协议）。
- ⚡ **流式输出**：后端通过 SSE 推送 `message.delta` / `tool.call` / `tool.result` / `run.finished`，前端打字机渲染。
- 🛑 **可中断运行**：客户端可随时调用 `POST /runs/:id/cancel` 终止当前 Run，Agent 在下一次 LLM 推理前优雅退出。
- 🏷️ **自动标题**：首条消息后，Agent 会用 LLM 为新会话生成标题。
- 🌓 **亮 / 暗主题**：右上角一键切换，写入 `localStorage`，首次渲染前内联脚本避免 FOUC。
- 🔔 **Toast 通知**：统一的错误/成功提示（FIFO 上限 5 条，4 秒自动消失），所有网络错误自动入栈。
- 🚨 **错误横幅**：聊天失败时显示可重试 / 关闭的横条，不再伪装成普通消息。
- 🔎 **会话搜索 / 排序**：侧边栏顶部输入框实时过滤标题，4 种排序（最近更新 / 创建时间 / 标题 / 消息数）。
- 📊 **请求日志面板**：右侧抽屉查看最近 100 条请求（方法、路径、状态、耗时、错误）；顶部 3 个统计卡片（总数 / 成功 / 失败），每 10 秒自动刷新后端健康状态。

## 📁 目录结构

```
.
├── .trae/
│   ├── documents/         # SDD 文档（spec / plan / tasks / prd / tech-arch）
│   └── rules/             # 项目规则
├── backend/               # Rust 后端（axum + tokio + sqlx + SSE）
├── frontend/              # TypeScript 前端（Vite + React + Zustand + Tailwind）
└── shared/                # 前后端共享类型（手写对齐，TS 优先）
```

## 🚀 快速开始

### 环境要求

- Rust 1.78+
- Node.js 20+ 与 pnpm 9+
- SQLite（由 sqlx 直接读写本地文件）

### 1. 启动后端

```bash
cd backend
cp .env.example .env       # 按需修改 OPENAI_API_KEY 等
cargo run                 # 默认监听 0.0.0.0:8080
```

启动时若 `RUN_MIGRATIONS=true`（默认），会自动执行 `migrations/` 下的 SQL 文件。

健康检查：

```bash
curl http://localhost:8080/api/health
# {"status":"ok"}
```

### 2. 启动前端

```bash
cd frontend
pnpm install
pnpm dev                  # 默认运行在 http://localhost:5173
```

Vite 已配置 `/api/*` 反代到 `http://localhost:8080`，前端代码里直接写相对路径即可。

### 3. 不配置 LLM API Key？

未设置 `OPENAI_API_KEY` 时，后端自动使用 `MockProvider`，会返回带工具列表的固定回复，便于离线开发。`cargo test` 也是基于 MockProvider 跑的。

## ⚙️ 配置项（`backend/.env`）

| 变量 | 默认 | 说明 |
|------|------|------|
| `BIND_ADDR` | `0.0.0.0:8080` | HTTP 监听地址 |
| `DATABASE_URL` | `sqlite://data/agent.db` | sqlx 数据库 URL |
| `RUN_MIGRATIONS` | `true` | 启动时自动执行 SQL 迁移 |
| `CORS_ALLOW_ORIGIN` | `http://localhost:5173` | CORS 白名单 |
| `OPENAI_API_KEY` | _空_ | 留空则使用 `MockProvider` |
| `OPENAI_BASE_URL` | `https://api.openai.com/v1` | 兼容 OpenAI 协议的接入点 |
| `OPENAI_MODEL` | `gpt-4o-mini` | 模型名 |
| `OPENAI_TEMPERATURE` | _空_ | 留空则不传，使用 provider 默认 |
| `AGENT_SYSTEM_PROMPT` | _中文_ | 每次 Run 的系统提示词 |
| `AGENT_MAX_TOOL_CALLS` | `10` | 单次 Run 最多工具调用次数（防失控） |
| `AGENT_TOOL_TIMEOUT_MS` | `30000` | 单次工具执行超时（毫秒） |
| `AGENT_AUTO_TITLE` | `true` | 首条消息后自动生成会话标题 |
| `RUST_LOG` | `info` | tracing-subscriber EnvFilter |

> ⚠️ 不要把任何 `.env` 提交到仓库。`.gitignore` 已忽略。

## 📡 API 速查

后端前缀 `/api`，所有响应均为 JSON 格式。

| Method | Path | 说明 |
|--------|------|------|
| GET | `/health` | 健康检查 |
| GET | `/sessions` | 列出当前用户全部会话 |
| POST | `/sessions` | 创建会话，body `{ title?, agent_id? }` |
| GET | `/sessions/:id` | 获取会话详情 |
| PUT | `/sessions/:id` | 重命名会话 |
| DELETE | `/sessions/:id` | 删除会话 |
| GET | `/sessions/:id/messages` | 列出消息 |
| POST | `/sessions/:id/messages` | 手动追加消息 |
| POST | `/sessions/:id/runs` | 启动一次 Agent Run，返回 SSE 流 |
| POST | `/sessions/:id/runs/:run_id/cancel` | 取消运行中的 Run |
| GET | `/tools` | 列出内置工具 |

### 启动一次 Run（SSE 协议）

请求：

```bash
curl -N -X POST http://localhost:8080/api/sessions/<sid>/runs \
  -H 'Content-Type: application/json' \
  -d '{"user_message":"你好"}'
```

事件流（每行 `event:` 头 + `data:` 负载）：

```
event: run.started
data: {"run_id":"<uuid>"}

event: message.delta
data: {"delta":"你好！"}

event: tool.call
data: {"id":"call_xxx","name":"get_current_time","arguments":{}}

event: tool.result
data: {"call_id":"call_xxx","output":"2026-06-15T17:00:00Z"}

event: message.delta
data: {"delta":"现在时间是 ..."}

event: run.finished
data: {"run_id":"<uuid>","status":"ok"}
```

`status` 取值：`ok` / `cancelled` / `max_iterations` / `error`。

### 取消一次 Run

```bash
curl -X POST http://localhost:8080/api/sessions/<sid>/runs/<run_id>/cancel
# {"cancelled":"<run_id>"}
```

返回 `404` 表示该 Run 已结束或不存在。取消后 SSE 流会收到 `run.finished { status: "cancelled" }`。

## 🧱 架构概览

```
┌────────────┐   SSE    ┌──────────────────┐
│  React UI  │ ◀──────▶ │  axum routes     │
│  Zustand   │          │   /runs (SSE)    │
└────────────┘          │   /sessions /... │
                        └────────┬─────────┘
                                 │
                          ┌──────▼──────┐
                          │ AgentRuntime│ think → tool → think
                          └──────┬──────┘
                                 │
              ┌──────────────────┼──────────────────┐
              ▼                  ▼                  ▼
        ModelProvider       ToolRegistry        RunRegistry
        (Mock / OpenAI)     (echo/time/http)    (oneshot cancel)
```

**关键模块**

- [`backend/src/agent/runtime.rs`](./backend/src/agent/runtime.rs) — 驱动主循环；支持 `run_with_cancel(history, tx, cancel_future)`。
- [`backend/src/agent/run_registry.rs`](./backend/src/agent/run_registry.rs) — 内存中的 `run_id → oneshot::Sender` 映射。
- [`backend/src/agent/title.rs`](./backend/src/agent/title.rs) — 调用 LLM 为新会话生成标题。
- [`backend/src/routes/runs.rs`](./backend/src/routes/runs.rs) — SSE 端点；预生成 `run_id` 以便 cancel 立即生效。
- [`backend/src/config.rs`](./backend/src/config.rs) — 加载 `.env` 并解析所有可调参数。
- [`frontend/src/hooks/useRunStream.ts`](./frontend/src/hooks/useRunStream.ts) — 解析 SSE 并 dispatch 到 Zustand store。
- [`frontend/src/store/chatStore.ts`](./frontend/src/store/chatStore.ts) — 维护消息流、tool calls、运行态。

## 🎨 体验打磨（M6）

| 能力 | 实现位置 | 关键点 |
|------|----------|--------|
| 暗色主题 | `frontend/src/store/themeStore.ts`、`components/ThemeToggle.tsx`、`tailwind.config.js` | `darkMode: "class"`，`localStorage` 持久化，`index.html` 内联脚本在 React 渲染前设 `dark` class 防 FOUC |
| Toast 通知 | `frontend/src/store/toastStore.ts`、`components/Toast.tsx` | FIFO 上限 5 条，4 秒自动消失，`api/client.ts` 拦截所有非 2xx 请求并 push |
| 错误横幅 | `frontend/src/components/ErrorBanner.tsx`、`store/chatStore.ts` | 流式失败时显示，提供"重试发送"和"关闭"按钮；不再伪装成普通消息 |
| 会话搜索/排序 | `frontend/src/hooks/useFilteredSessions.ts`、`components/SessionSearchBar.tsx` | `useMemo` 包裹，4 种排序：`updated` / `created` / `title` / `messageCount` |
| 请求可观测性 | `frontend/src/lib/requestLogger.ts`、`components/RequestLogPanel.tsx` | 100 条环形缓冲区，订阅者模式；右侧抽屉显示方法 / 路径 / 状态 / 耗时 / 错误；顶部 3 个统计卡片 |

`requestLogger` 在 `api/client.ts` 的 fetch 拦截器里被调用，所有 `apiClient` 出去的请求都会被记录，方便排查后端连通性问题。

## 🧪 测试

```bash
cd backend && cargo test          # 13 个单元/集成测试
cd frontend && pnpm test          # 43 个 Vitest 单元测试
```

测试覆盖：

- `agent::run_registry` — 注册、取消、清理
- `agent::runtime` — MockProvider 闭环 + 取消信号
- `repo::session` / `repo::message` — SQLite CRUD
- `tools::echo` — JSON Schema 校验 + invoke
- `config` — `.env` 解析
- `frontend/store/toastStore` — Toast 队列 FIFO
- `frontend/store/chatStore` — 错误状态 + 消息流
- `frontend/store/themeStore` / `ThemeToggle` — 主题切换
- `frontend/components/{Toast,ErrorBanner,SessionSearchBar,RequestLogPanel}` — 组件渲染
- `frontend/hooks/useFilteredSessions` — 搜索/排序
- `frontend/lib/requestLogger` — 环形缓冲区

## 🛠️ 常用脚本

| 模块 | 命令 | 说明 |
|------|------|------|
| backend | `cargo run` | 启动服务 |
| backend | `cargo test` | 运行测试 |
| backend | `cargo build --release` | 打包二进制 |
| backend | `SQLX_OFFLINE=true cargo build` | 使用 `.sqlx/` 缓存离线编译 |
| frontend | `pnpm dev` | 启动 Vite dev server |
| frontend | `pnpm build` | 类型检查 + 生产构建 |
| frontend | `pnpm test` | 运行 Vitest |
| frontend | `pnpm lint` | ESLint |

## 🐛 常见问题

**Q: `cargo build` 报 `(code: 14) unable to open database file`？**
A: 后端运行时会自动创建 `data/agent.db`，但要求 `data/` 目录可写。如果放在只读目录里请先 `mkdir -p data/`。

**Q: SSE 中文乱码？**
A: 检查 `Content-Type: text/event-stream; charset=utf-8` 响应头；如自建反代，请关闭对 SSE 的缓冲。

**Q: 想换模型？**
A: 把 `OPENAI_API_KEY` 填上即可，任意兼容 `/v1/chat/completions` 的服务（OpenAI / DeepSeek / 月之暗面等）都能直接接上。

## 📚 文档

- 规范（要构建什么）：[`.trae/documents/spec.md`](./.trae/documents/spec.md)
- 计划（如何构建）：[`.trae/documents/plan.md`](./.trae/documents/plan.md)
- 任务：[`.trae/documents/tasks.md`](./.trae/documents/tasks.md)
- 产品需求：[`.trae/documents/prd.md`](./.trae/documents/prd.md)
- 技术架构：[`.trae/documents/technical-architecture.md`](./.trae/documents/technical-architecture.md)
- 项目规则：[`.trae/rules/project_rules.md`](./.trae/rules/project_rules.md)

## 🛣️ 路线图

| 里程碑 | 内容 | 状态 |
|--------|------|------|
| **M1** | 目录结构 + 文档 + 后端/前端可启动 | ✅ |
| **M2** | 持久化（Session/Message CRUD + SQLite） | ✅ |
| **M3** | Agent 内核（Runtime + Mock/OpenAI Provider + 工具调用闭环） | ✅ |
| **M4** | 流式 UI（SSE + 前端打字机 + 停止） | ✅ |
| **M5** | 真实模型接入 + 取消运行 + 自动标题 | ✅ |
| **M6** | 体验打磨（错误态/空态/主题/可观测性） | ✅ |

## License

MIT
