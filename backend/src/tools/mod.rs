//! Built-in tool implementations.

use crate::agent::ToolRegistry;
use crate::tools::{echo::EchoTool, http_get::HttpGetTool, time::TimeTool};

pub mod echo;
pub mod http_get;
pub mod time;

pub fn builtin_tools() -> ToolRegistry {
    let reg = ToolRegistry::new();
    reg.register(EchoTool);
    reg.register(TimeTool);
    reg.register(HttpGetTool::default());
    reg
}
