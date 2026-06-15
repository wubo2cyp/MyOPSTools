# Frontend - agent platform

See [README.md](../README.md) for project-level instructions.

## Quick start

```bash
pnpm install
pnpm dev
```

App is served on <http://localhost:5173>.

## Scripts

| Command | Description |
|---------|-------------|
| `pnpm dev` | Start Vite dev server with HMR |
| `pnpm build` | Type-check and build for production |
| `pnpm preview` | Preview the production build |
| `pnpm lint` | Run ESLint |
| `pnpm test` | Run unit tests with Vitest (43 tests) |

## Layout

```
src/
  pages/      # Top-level route components
  components/ # Reusable UI components
  hooks/      # Custom React hooks (useRunStream, useFilteredSessions, ...)
  store/      # Zustand stores (chat / theme / toast)
  api/        # HTTP / SSE client (apiClient + sse parser)
  lib/        # Pure utilities (SSE parser, requestLogger ring buffer)
  styles/     # Global styles (Tailwind layers + dark variables)
  test/       # Vitest setup (matchMedia mock, etc.)
```

## M6 highlights

- **主题切换**：`store/themeStore.ts` + `components/ThemeToggle.tsx`；`tailwind.config.js` 启用 `darkMode: "class"`，`index.html` 内联脚本在 React 渲染前写入 `dark` class 防 FOUC。
- **Toast / ErrorBanner**：`store/toastStore.ts` 是 FIFO 队列（5 条上限 + 4 秒自动消失），`api/client.ts` 自动 push 非 2xx 错误；`components/ErrorBanner.tsx` 负责聊天流式失败的横条展示与重试。
- **会话搜索/排序**：`hooks/useFilteredSessions.ts` 是 `useMemo` 包装的纯函数；`components/SessionSearchBar.tsx` 提供输入框 + 4 种排序下拉。
- **可观测性**：`lib/requestLogger.ts` 是 100 条环形缓冲区，订阅者模式；`components/RequestLogPanel.tsx` 渲染右侧抽屉与 3 个统计卡片。
