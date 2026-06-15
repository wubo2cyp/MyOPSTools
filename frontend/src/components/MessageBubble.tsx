import { Bot, User, Wrench } from "lucide-react";
import type { ReactNode } from "react";
import type { Message } from "@shared/types";

interface Props {
  message: Message;
}

const ICONS: Record<Message["role"], ReactNode> = {
  system: <Wrench size={14} />,
  user: <User size={14} />,
  assistant: <Bot size={14} />,
  tool: <Wrench size={14} />,
};

const LABELS: Record<Message["role"], string> = {
  system: "System",
  user: "You",
  assistant: "Agent",
  tool: "Tool",
};

export function MessageBubble({ message }: Props) {
  const isUser = message.role === "user";
  return (
    <div className={`flex gap-3 ${isUser ? "flex-row-reverse" : ""}`}>
      <div
        className={`flex h-7 w-7 shrink-0 items-center justify-center rounded-full ${
          isUser ? "bg-zinc-900 text-white" : "bg-accent/10 text-accent"
        }`}
        aria-hidden
      >
        {ICONS[message.role]}
      </div>
      <div
        className={`max-w-[80%] whitespace-pre-wrap rounded-2xl px-4 py-2.5 text-sm leading-relaxed shadow-sm ${
          isUser
            ? "bg-zinc-900 text-zinc-50"
            : "bg-white text-zinc-800 ring-1 ring-zinc-200"
        }`}
      >
        <div className="mb-1 text-[10px] font-medium uppercase tracking-widest opacity-60">
          {LABELS[message.role]}
        </div>
        {message.content}
      </div>
    </div>
  );
}
