//! Desktop IPC commands。See Architecture §5.2。
//! 前端 → 后端的命令边界。命令只做参数解包 + 调用引擎/存储，不含业务逻辑。

pub mod reminder;
pub mod settings;
