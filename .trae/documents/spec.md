# 规范：轻量级通用 Agent 平台 (Lightweight General-Purpose Agent)

> 本文件遵循 **SDD（Specification-Driven Development，规范驱动开发）** 流程定义 "要构建什么"。
> 所有后续实现都必须以本规范为唯一真相源（Single Source of Truth），任何偏离需先更新本文件并标注变更。

---

## 1. 目标与非目标

### 1.1 一句话目标
提供一个 **轻量级、可扩展的通用 Agent 平台**，让用户通过 Web 界面与 Agent 进行多轮对话，并可按需启用工具（Tools）完成具体任务。

### 1.2 范围
- ✅ Web 聊天界面（对话、消息流、工具调用可视化）
- ✅ 后端 Agent 运行时（对话状态管理、工具调用、模型接入）
- ✅ 工具注册机制（开箱即用 + 自定义扩展）
- ✅ 会话历史持久化
- ✅ REST + Server-Sent Events（流式输出）

### 1.3 非目标（v1.0 不做）
- ❌ 多租户 / 企业级权限系统
- ❌ 复杂的 Agent 工作流编排（仅支持单 Agent 顺序工具调用）
- ❌ 多模态（图片、语音、文件上传分析）
- ❌ 移动端原生 App
- ❌ 生产级可观测性（Trace / APM）

---

## 2. 用户与角色

| 角色 | 描述 | 权限 |
|------|------|------|
| 访客 | 未登录用户 | 不可用 |
| 用户 | 已注册并登录 | 创建/查看/删除自己的会话；调用 Agent |
| 管理员 | 维护工具与系统提示词 | 增删工具、查看所有会话 |

> v1.0 **不实现完整注册登录**，先用本地用户名 + 简易 Token 方案预留接口。

---

## 3. 核心概念

| 概念 | 含义 |
|------|------|
| **Agent** | 一个具备系统提示词 + 可用工具集合的 LLM 驱动体 |
| **Session** | 用户与 Agent 的一次连续对话，包含完整消息历史 |
| **Message** | 一条消息，类型可为 `user` / `assistant` / `tool` / `system` |
| **Tool** | Agent 可调用的具名函数，带 JSON Schema 输入 |
| **Run** | 一次 Agent 执行实例（可能包含多次 LLM 推理 + 工具调用） |

---

## 4. 功能需求

### 4.1 会话管理
- F-S1：用户可创建新会话（可选标题、可选 Agent）
- F-S2：用户可列出自己的所有会话
- F-S3：用户可查看会话详情（含全部消息）
- F-S4：用户可重命名 / 删除会话
- F-S5：会话消息按时间顺序持久化

### 4.2 对话与 Agent 运行
- F-A1：用户可在会话中发送消息，触发一次 Run
- F-A2：Run 支持 **流式输出**（SSE），前端可实时渲染文本与工具调用
- F-A3：当 LLM 决策调用工具时，后端执行工具并将结果回填，再次推理，直到得到最终答案
- F-A4：前端需显示中间过程：思考文本、工具调用、工具结果
- F-A5：用户可在 Run 过程中点击"停止"中断
- F-A6：同一会话内多轮消息自动保留上下文

### 4.3 工具系统
- F-T1：后端提供工具注册表（`tools/` 目录），启动时自动发现并注册
- F-T2：每个工具需声明 `name` / `description` / `input_schema`
- F-T3：工具执行需有超时与错误捕获
- F-T4：内置工具（v1.0）：`echo`（演示）、`get_current_time`（演示）、`http_get`（实用）

### 4.4 模型接入
- F-M1：抽象 `ModelProvider` trait，支持 OpenAI 兼容协议
- F-M2：通过环境变量配置 `API_KEY` / `BASE_URL` / `MODEL`
- F-M3：未配置时使用本地 Mock Provider，返回固定文本，便于离线开发

### 4.5 前端体验
- F-U1：响应式布局，桌面优先
- F-U2：左侧会话列表 + 右侧对话区
- F-U3：消息区分用户/助手/工具气泡样式
- F-U4：流式打字机效果
- F-U5：加载态、空态、错误态明确
- F-U6：可停止当前运行

---

## 5. 非功能需求

| 类别 | 指标 |
|------|------|
| 启动时间 | 后端冷启动 < 2s；前端首屏 < 1.5s（本地） |
| 并发 | 单实例支持 50 个并发 SSE 连接 |
| 资源 | 后端空闲内存 < 80MB；打包后前端 < 500KB gzip |
| 跨域 | 后端默认允许 `http://localhost:5173` |
| 配置 | 所有密钥走环境变量，仓库内 **不** 提交任何密钥 |
| 日志 | 后端使用 `tracing`，结构化 JSON 输出到 stdout |
| 错误 | 对外统一 `{ error: { code, message } }` JSON |

---

## 6. 数据模型（高层）

```text
User       (id, name, created_at)
Session    (id, user_id, title, agent_id, created_at, updated_at)
Message    (id, session_id, role, content, tool_calls?, tool_call_id?, created_at)
Run        (id, session_id, status, started_at, finished_at?)
Agent      (id, name, system_prompt, tool_names[])
Tool       (name, description, input_schema)  // 注册表，运行时只读
```

> v1.0 默认使用 **SQLite** 持久化（通过 `sqlx`），便于单机零配置运行。

---

## 7. API 概览

| 方法 | 路径 | 说明 |
|------|------|------|
| GET    | `/api/health` | 健康检查 |
| GET    | `/api/sessions` | 列出会话 |
| POST   | `/api/sessions` | 创建会话 |
| GET    | `/api/sessions/:id` | 会话详情 |
| PATCH  | `/api/sessions/:id` | 更新会话（重命名） |
| DELETE | `/api/sessions/:id` | 删除会话 |
| GET    | `/api/sessions/:id/messages` | 列出消息 |
| POST   | `/api/sessions/:id/runs` | 触发一次 Run（**SSE 流式**） |
| POST   | `/api/sessions/:id/runs/:run_id/cancel` | 取消运行 |
| GET    | `/api/tools` | 列出已注册工具 |

> API 详细 schema 见 [`technical-architecture.md`](./technical-architecture.md)。

---

## 8. 验收标准（Definition of Done）

- [ ] 后端 `cargo run` 启动成功，`/api/health` 返回 200
- [ ] 前端 `pnpm dev` 启动后可访问聊天界面
- [ ] 可在 UI 创建会话、发送消息、看到流式回复
- [ ] 至少一个内置工具（如 `get_current_time`）可被 LLM 正确调用并展示
- [ ] 关闭服务后再次启动，历史会话与消息可恢复
- [ ] `cargo test` 与 `pnpm test` 全部通过
- [ ] README 中包含本地启动步骤

---

## 9. 变更记录

| 版本 | 日期 | 变更 |
|------|------|------|
| 0.1.0 | 2026-06-15 | 初版规范 |
