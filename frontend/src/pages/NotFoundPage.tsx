import { Link } from "react-router-dom";

export function NotFoundPage() {
  return (
    <div className="flex h-screen w-screen flex-col items-center justify-center gap-3 bg-zinc-50 text-zinc-700">
      <span className="font-mono text-xs uppercase tracking-widest text-zinc-400">404</span>
      <h1 className="text-2xl font-semibold">页面不存在</h1>
      <Link to="/" className="text-sm text-accent hover:underline">
        返回首页
      </Link>
    </div>
  );
}
