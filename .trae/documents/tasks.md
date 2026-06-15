# 任务分解：轻量级通用 Agent 平台

> 与 [`spec.md`](./spec.md) 和 [`plan.md`](./plan.md) 配套。任务按里程碑（M1–M6）分组。
> 标记说明：⬜ 未开始 / 🟡 进行中 / ✅ 完成 / ❌ 阻塞

---

## M1 - 项目骨架（本次任务范围）

- ✅ T1.1 创建 `.trae/documents/` 目录及 SDD 文档
- ✅ T1.2 创建 `.trae/rules/project_rules.md`
- ✅ T1.3 创建 `backend/` Rust crate 骨架（Cargo.toml、main.rs、模块占位）
- ✅ T1.4 创建 `frontend/` Vite + React + TS 骨架
- ✅ T1.5 创建 `shared/` 类型定义占位
- ✅ T1.6 创建仓库根级文件：`.gitignore`、`.editorconfig`、`README.md`
- ✅ T1.7 后端 `cargo build` 通过
- ⬜ T1.8 前端 `pnpm install` + `pnpm build` 通过

## M2 - 持久化

- ✅ T2.1 编写 SQL 迁移：`users` / `sessions` / `messages` / `agents`
- ✅ T2.2 实现 `repo::SessionRepo` / `repo::MessageRepo`
- ✅ T2.3 实现 `routes::sessions` 增删改查
- ✅ T2.4 编写集成测试（in-memory SQLite）
- ✅ T2.5 HTTP 端点验证通过（Session/Message CRUD）

## M3 - Agent 内核

- ⬜ T3.1 定义 `agent::Tool` trait 与 `ToolRegistry`
- ⬜ T3.2 实现 `model::ModelProvider` trait
- ⬜ T3.3 实现 `model::MockProvider`（离线可用）
- ⬜ T3.4 实现 `model::OpenAIProvider`
- ⬜ T3.5 实现 `agent::Runtime` 循环
- ⬜ T3.6 内置工具：`echo` / `get_current_time` / `http_get`
- ⬜ T3.7 单元测试

## M4 - 流式 UI

- ⬜ T4.1 后端 `/runs` 端点实现 SSE
- ⬜ T4.2 前端 `apiClient` + `sse.ts` 解析器
- ⬜ T4.3 前端 `useRunStream` Hook
- ⬜ T4.4 `ChatPanel` 流式渲染
- ⬜ T4.5 `ToolCallCard` 工具调用可视化
- ⬜ T4.6 停止按钮

## M5 - 真实模型

- ⬜ T5.1 `.env.example` 完善
- ⬜ T5.2 README 启动文档
- ⬜ T5.3 端到端冒烟测试

## M6 - 打磨

- ⬜ T6.1 错误态/空态/加载态
- ⬜ T6.2 简单主题切换（亮/暗）
- ⬜ T6.3 会话标题自动生成（用 LLM）

---

## 当前 Sprint

> **M2 - 持久化**：已完成 Session/Message CRUD 实现、集成测试和 HTTP 端点验证。下一步更新前端以调用真实 API。
