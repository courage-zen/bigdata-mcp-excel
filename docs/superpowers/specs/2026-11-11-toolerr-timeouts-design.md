# Tool-Result-Layer Error Codes + Layered Timeouts

excel-mcp 对照《MCP 服务 S3 使用标准》（2026-11-11）的架构级补齐：§五
机器可读错误码（tool 结果层双通道）与 §四分层超时。快修档（env 标准名、
黑名单补齐、消息净化）已落在 0.2.0 main。

## 背景与已决契约

标准点名 excel-mcp 短板：“无任何超时配置、无机器可读错误码”。现状：

- 24 处 `McpError::invalid_params/internal_error`，业务失败走 JSON-RPC
  协议层——标准明确“不用协议错误承载业务失败”。
- `StorageError` 三分类在映射时压平成 message，调用方无法程序化区分。
- 全仓无任何超时：client 构建无 connect/read 超时，下载无墙钟，
  `spawn_blocking` 引擎调用无处理时限（object_store 默认请求超时 30s
  存在但从未显式配置，也不区分层）。

三个契约决定（已与用户确认）：

1. **错误载荷 = 单 JSON 文本块**：`is_error: true` + 一个 text block，
   内容 `{"error_code": "<SNAKE_CODE>", "error": "<human message>"}`。
   agent 端解析逻辑与成功路径同构（都是 parse text 为 JSON）。
   bigdata-mcp-email 先例。
2. **体内全量 tool 层**：tool 方法体内可控的全部失败（key 校验、S3
   IO、引擎、超时）→ tool 结果层带 error_code；serde 反序列化失败
   （缺字段/类型错，rmcp 框架行为）→ 保持协议层 invalid_params。
   边界即“我们代码内 vs 框架”。
3. **超时 = 代码常量**：四层值写成常量，不进 env。标准即默认即全部。

## 设计

### 1. 错误类型与码表（`src/toolerr.rs`，新模块）

```rust
pub enum ToolError {
    // --- 标准 §五 六个 S3 码 ---
    S3Unavailable,          // S3 未配置（缺必填 env）
    S3ObjectNotFound,       // 对象不存在
    S3FetchFailed,          // 下载阶段其他失败（网络、403 等）
    S3UploadFailed,         // 上传失败
    S3ObjectTooLarge(u64),  // 超限；负载带 limit 值
    S3InvalidKey(String),   // key 校验不通过；已净化消息
    // --- 服务码（三工具通用） ---
    InvalidArguments(String), // data 解析失败、sheet 名不存在等参数错
    EngineTimeout,           // 处理墙钟超时
    Internal(String),        // 兜底；已净化消息
}

impl ToolError {
    pub fn error_code(&self) -> &'static str; // "S3_UNAVAILABLE" 等
}
```

- `Display` 输出人可读消息（全部净化：不回显被拒 key、不点 bucket/
  endpoint 名——`S3InvalidKey` 载荷就是已净化的消息字符串）。
- `From<StorageError>`、`From<InvalidKey>`；引擎错误由 tool 层当场
  `map_err` 成 `InvalidArguments`（解析/参数类）或 `Internal`。
- 超时变体映射：client 构建失败 → `S3Unavailable`；下载网络错 →
  `S3FetchFailed`；上传错 → `S3UploadFailed`；两层墙钟超时 →
  `EngineTimeout`（处理）/ `S3FetchFailed`（下载，语义是“取不回来”）。

### 2. 超时分层（`src/storage.rs` + tool 层）

常量（标准 §四 表格即代码）：

```rust
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(60);
const DOWNLOAD_WALL: Duration = Duration::from_secs(60);
const PROCESSING_WALL: Duration = Duration::from_secs(30);
```

- `storage::client()`：`AmazonS3Builder::new().with_client_options(
  ClientOptions::default().with_connect_timeout(CONNECT_TIMEOUT)
  .with_timeout(READ_TIMEOUT))`。
- `fetch_to_tempfile` 内部包 `tokio::time::timeout(DOWNLOAD_WALL, …)`
  整个取流循环；超时 → `StorageError::Timeout`（新变体 → 映射
  `S3FetchFailed`）。
- 处理墙钟在 tool 层：三个 `spawn_blocking` 调用外各包
  `tokio::time::timeout(PROCESSING_WALL, …)`，超时 → `EngineTimeout`。
  引擎本身不知超时（`spawn_blocking` 任务被放弃后 worker 线程继续跑
  完再退出——阻塞池任务无法强杀，这是接受的行为：调用方 30s 拿到
  错误，不等待病态工作簿）。

### 3. Tool 层改造（`src/tools.rs`）

- 每个 `#[tool]` 方法体收敛为薄包装：内层私有函数返回
  `Result<serde_json::Value, ToolError>`，`#[tool]` 方法渲染：

```rust
fn render(value: Result<serde_json::Value, ToolError>) -> CallToolResult {
    match value {
        Ok(v) => CallToolResult::success(vec![ContentBlock::text(v.to_string())]),
        Err(e) => CallToolResult {
            is_error: Some(true),
            content: vec![ContentBlock::text(
                json!({"error_code": e.error_code(), "error": e.to_string()}).to_string(),
            )],
        },
    }
}
```

- 三工具的内层函数按现逻辑平移，错误点全部换 `?`/`map_err(ToolError
  ::from)`。24 处 `McpError::*` 全部消失（仅 `render` 附近保留零处）。
- 错误消息文案沿用现有（已净化的）字符串，改动是**通道**不是文案。

### 4. 测试与文档

- 单测（tools.rs 现有 5 个错误路径测试）：断言从 `McpError` message
  改为 parse text JSON 后断言 `error_code` 精确值 + `error` 非空 +
  `is_error == true`。
- 新增：`toolerr` 码表测试（每变体 error_code 字符串、Display 不含
  bucket/endpoint/被拒 key 字样——对全部变体参数化跑）。
- 超时单测：`fetch_to_tempfile` 墙钟——用自定义 `ObjectStore` mock
  挂起流（`futures::stream::pending()`）断言 60s 超时走
  `StorageError::Timeout`（用 `#[tokio::test(start_paused = true)]`
  时间冻结，不真等 60s）；处理墙钟同法冻结 30s。
- 集成测试：现有 7 个全保留（断言 `s3_key` 的成功路径不变）；新增
  一个 `error_code` 断言例（traversal key 被拒 → text 里
  `S3_INVALID_KEY`）。
- `.env.example` / README：配置表不动（无新 env）；README 增“错误
  契约”小节（码表 + isError 双通道说明 + serde 层例外说明）。
- 语义变更（协议层 → tool 层）→ 0.3.0。

## 非目标

- 强杀引擎线程（无法实现，见 §2；调用方拿到错误即返回）。
- 错误码进 env / 超时进 env（已决：常量）。
- 重试/退避逻辑（标准未要求）。
- serde 层错误搬进 tool 层（rmcp 框架边界，已决保持协议层）。
