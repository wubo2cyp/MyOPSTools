import { apiGet } from "./client";
import type { ToolDef } from "@shared/types";

export const listTools = () => apiGet<ToolDef[]>("/tools");
