# Project Rules

> 本项目遵循 **SDD（Specification-Driven Development）** 流程。所有变更必须先更新 `.trae/documents/` 下的相应文档，再修改代码。

## 文档优先级
1. `spec.md` —— 要构建什么（Single Source of Truth）
2. `plan.md` —— 如何构建
3. `tasks.md` —— 任务分解与状态
4. `prd.md` / `technical-architecture.md` —— 配套说明

## 改动流程
- 任何功能新增 / 变更：先改 `spec.md` → 同步 `plan.md` / `tasks.md` → 写代码
- 任何技术栈变更：先改 `plan.md` 和 `technical-architecture.md` → 再改代码

## 代码组织
- 后端：Rust 1.78+ edition 2021，使用 `axum` + `sqlx` + `tokio`
- 前端：TypeScript 5 + Vite 5 + React 18 + Tailwind 3 + Zustand
- 包管理：前端 `pnpm`，后端 `cargo`
- 共享类型：放在 `shared/types.ts`，前端直接 import，后端以注释/手写结构体对齐

## 提交规范（建议）
- `feat(scope): ...` 新功能
- `fix(scope): ...` 修复
- `docs: ...` 文档
- `chore: ...` 杂项
- `refactor: ...` 重构

## 禁止
- ❌ 提交 `.env` 或任何密钥
- ❌ 直接修改已合并的 SQL 迁移文件
- ❌ 跳过 `spec.md` 讨论直接改架构
