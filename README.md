# 轻量级通用 Agent 平台

> 一个轻量级、可扩展的通用 Agent 平台。前端 TypeScript + React，后端 Rust + Axum，遵循 **SDD（Specification-Driven Development）** 流程。
> 
> 本仓库当前处于 **M1 - 项目骨架** 阶段。详细规范见 [`.trae/documents/`](./.trae/documents/)。

## 目录结构

```
.
├── .trae/
│   ├── documents/         # SDD 文档（spec / plan / tasks / prd / tech-arch）
│   └── rules/             # 项目规则
├── backend/               # Rust 后端（axum + tokio + sqlx）
├── frontend/              # TypeScript 前端（Vite + React + Tailwind）
└── shared/                # 前后端共享类型
```

## 快速开始

### 1. 启动后端

```bash
cd backend
cp .env.example .env       # 按需修改 OPENAI_API_KEY 等
cargo run
```

后端默认监听 <http://localhost:8080>，健康检查 `GET /api/health`。

### 2. 启动前端

```bash
cd frontend
pnpm install
pnpm dev
```

前端默认运行在 <http://localhost:5173>，已通过 Vite proxy 将 `/api/*` 反代到后端。

### 3. 不配置 LLM API Key？

未设置 `OPENAI_API_KEY` 时，后端使用内置的 `MockProvider`，会返回带工具列表的固定回复，便于离线开发。

## 常用脚本

| 模块 | 命令 | 说明 |
|------|------|------|
| backend | `cargo run` | 启动服务 |
| backend | `cargo test` | 运行测试 |
| backend | `cargo build --release` | 打包二进制 |
| frontend | `pnpm dev` | 启动 Vite |
| frontend | `pnpm build` | 构建生产包 |
| frontend | `pnpm test` | 运行 Vitest |
| frontend | `pnpm lint` | ESLint |

## 文档

- 规范（要构建什么）：[`.trae/documents/spec.md`](./.trae/documents/spec.md)
- 计划（如何构建）：[`.trae/documents/plan.md`](./.trae/documents/plan.md)
- 任务：[`.trae/documents/tasks.md`](./.trae/documents/tasks.md)
- 产品需求：[`.trae/documents/prd.md`](./.trae/documents/prd.md)
- 技术架构：[`.trae/documents/technical-architecture.md`](./.trae/documents/technical-architecture.md)
- 项目规则：[`.trae/rules/project_rules.md`](./.trae/rules/project_rules.md)

## 路线图

| 里程碑 | 内容 |
|--------|------|
| **M1** ✅ | 目录结构 + 文档 + 后端/前端可启动 |
| M2 | 持久化（Session/Message CRUD + SQLite） |
| M3 | Agent 内核（Runtime + Mock/OpenAI Provider + 工具调用闭环） |
| M4 | 流式 UI（SSE + 前端打字机 + 停止） |
| M5 | 真实模型接入 + 端到端测试 |
| M6 | 体验打磨 |

## License

MIT
