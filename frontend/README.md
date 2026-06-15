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
| `pnpm test` | Run unit tests with Vitest |

## Layout

```
src/
  pages/      # Top-level route components
  components/ # Reusable UI components
  hooks/      # Custom React hooks
  store/      # Zustand stores
  api/        # HTTP / SSE client
  lib/        # Pure utilities (e.g. SSE parser)
  styles/     # Global styles
```
