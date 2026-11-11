# MCP 服务 S3 使用标准（S3 Usage Standard for MCP Servers）

把三个现有服务（excel-mcp、markitdown-mcp、bigdata-mcp-email）的 S3 实践
合成为一份行为契约。此后新开 MCP 服务涉及 S3 时照此实现，不再各自发明
配置命名、key 校验、错误码与限制语义。

定位：规范定义**可观察行为**，不规定实现细节（语言、S3 库、客户端生命
周期均自由）。各语言栈照契约逐条实现。

## 背景

三个项目对 S3 的用法已同源（同一个 multica 后端 + MinIO），但细节各自
演化：

- excel-mcp（Rust，object_store）：唯一读写双向；env 命名与
  markitdown-mcp 刻意对齐；0.2.0 起要求显式 `output_key`（`S3_OUTPUT_PREFIX`
  已删除）。短板：无任何超时配置、无机器可读错误码。
- markitdown-mcp（Python，boto3）：纯读；分层超时最完整；错误信息净化
  （不泄漏 bucket/endpoint/key）；内容层安全防护深厚。
- bigdata-mcp-email（Go，minio-go）：纯读（S3→SMTP 附件）；机器可读错误
  码分类的先行者；但 env 私有命名（`S3_ACCESS_KEY`）、YAML 优先于 env、
  region 硬编码、key 校验最薄弱。

本标准以 excel-mcp 为骨架，补入另两者的长处。

## 一、配置契约（环境变量）

配置一律经环境变量，不得引入 YAML 等第二配置源叠加 env 覆盖层。

| 变量 | 必填 | 语义 |
|---|---|---|
| `S3_BUCKET` | 是 | 唯一 bucket，读写共用 |
| `AWS_ACCESS_KEY_ID` | 是 | 静态访问密钥（不支持实例角色） |
| `AWS_SECRET_ACCESS_KEY` | 是 | 静态私钥 |
| `S3_REGION` | 否 | 回退链 `S3_REGION` → `AWS_REGION` → `us-east-1` |
| `AWS_ENDPOINT_URL` | 否 | S3 兼容端点（MinIO/R2）；空 = 原生 AWS S3 |
| `S3_USE_PATH_STYLE` | 否 | 取值 `1/true/yes`、`0/false/no`；未设置时：配了 `AWS_ENDPOINT_URL` 则默认 path-style，未配则 virtual-host |
| `S3_KEY_PREFIXES` | 否 | 逗号分隔 key 前缀白名单，段边界匹配；空 = 全 bucket 可用 |
| `S3_MAX_FILE_BYTES` | 否 | 单对象读/写大小上限，默认 104857600（100 MiB） |
| `S3_INLINE_MAX_BYTES` | 否 | 小文件结果内联阈值，默认 2097152（2 MiB）；仅写工具适用 |

语义规则：

1. **三态不存在**：必填三项（bucket + 凭据）齐备 = S3 可用；缺任一 =
   S3 功能禁用（其余非 S3 功能不受影响）。不存在"配置了但残缺"的第三态。
   禁用是**软失败**：服务正常启动，S3 依赖的 tool 调用返回
   `S3_UNAVAILABLE`。
2. 命名对齐 AWS CLI 习惯（`AWS_*`），与现有 excel-mcp/markitdown-mcp 部署
   共享同一份 secret。
3. Region 回退链与 path-style 推断规则与 excel-mcp / markitdown-mcp 现状
   完全一致，部署清单可直接复制。
4. `S3_MAX_FILE_BYTES` 是规范统一命名。现有 excel-mcp 的
   `EXCEL_MAX_FILE_BYTES` / `EXCEL_INLINE_MAX_BYTES` 为私有命名，对齐时迁移
   至标准名（旧名可留一个版本周期的兼容读取）。
5. 客户端生命周期（启动单例 vs 每请求新建 vs 懒导入）是实现细节，本规范
   不约束。

## 二、Key 契约

### 2.1 共同规则

- **Bucket 与凭据永远来自服务端配置**；tool 调用方只提供 key，协议中不
  出现 bucket、凭据、region 等服务端参数。
- Key 空间约定 `workspaces/<ws-id>/<name>`。该约定由 server instructions
  引导（excel-mcp 现行做法），非协议强制——服务不解析也不校验 workspace
  段，key 对服务是不透明的。

### 2.2 输入侧（`s3_key`）

接受两种形式，服务端归一化：

- 裸 key：`workspaces/ws-1/report.xlsx`
- 完整对象存储 URL：调用方可能粘贴附件 JSON 里的 presigned URL，服务须
  剥离 scheme/host/query（含 `X-Amz-Signature` 等签名参数）后还原出 key；
  支持 virtual-host 形式（`<bucket>.<domain>/key`）与 path-style 形式
  （`<endpoint>/<bucket>/key`，首段须等于配置 bucket）。

下载前校验（全部通过才开始网络请求）：

1. 非空；不以 `/` 开头；无 `//`；不含 `\`（反斜杠先归一化）
2. 无 `.`/`..`/空段
3. 危险扩展名黑名单（`.exe`/`.msi`/`.bat`/`.sh`/`.so`/`.dll`/`.deb`/`.rpm`
   等，按 key 后缀判定）
4. `S3_KEY_PREFIXES` 白名单：段边界匹配（前缀 `workspaces/ws-1` 匹配
   `workspaces/ws-1/a.xlsx`，不匹配 `workspaces/ws-11/a.xlsx`）
5. 不通过 → 返回 `S3_INVALID_KEY`，不回显被拒 key 的完整内容

### 2.3 输出侧（`output_key`）

- **服务端永不自动生成 key、永不使用默认前缀。** 写工具的 `output_key`
  是必填参数；缺失是反序列化错误，在引擎与 S3 工作开始之前失败。
- 校验与输入对称：同一套黑名单、同一份白名单。**写权限不得宽于读权限**
  ——白名单开启时，输出 key 必须落在同一段边界匹配的白名单内。
- 扩展名按服务输出类型强制：服务在 tool 描述里声明输出扩展名（excel-mcp
  为 `.xlsx`），输出 key 须以该扩展名结尾（大小写不敏感）。
- 上传幂等：已存在同 key 对象直接覆盖。

### 2.4 返回值

- 写工具成功返回 `{"s3_key": <key>, ...业务字段}`，**永不**返回
  presigned URL、bucket、endpoint 或本地路径。
- ≤ `S3_INLINE_MAX_BYTES` 的结果可附带 `content_b64` 内联（标准 base64，
  调用方零次额外往返即可拿到文件）。超过则只回 key。
- 读工具按服务语义返回（结构化 JSON、Markdown 文本等），不得泄漏临时
  文件路径。

## 三、IO 模式

**读**（所有涉及 S3 输入的 tool）：

1. 校验 key（见 2.2）
2. 元数据预检：对象大小 > `S3_MAX_FILE_BYTES` → 直接
   `S3_OBJECT_TOO_LARGE`，不起流
3. 流式下载（建议 1 MiB chunk）到临时文件；临时文件带服务名前缀与
   key 后缀（如 `excel-mcp-in-*.xlsx`）；流中累计字节复查上限
4. 处理完成后临时文件自动清理（失败路径同样清理）
5. 超时值见第四节

**写**：

1. 处理完成，得到输出字节
2. 大小校验（> `S3_MAX_FILE_BYTES` → `S3_OBJECT_TOO_LARGE`，不上传）
3. 整体上传；已存在 key 覆盖

**处理**（CPU/解析阶段，若有）：墙钟 30s，超时中止并返回错误（不含
部分结果）。

服务无本地持久状态：pod 重启不丢语义（临时文件是 call-scoped 的，本就
不跨请求）。

## 四、限制与超时

补 excel-mcp 的配置空白，采纳 markitdown-mcp 验证过的分层超时为标准值：

| 层 | 值 | 说明 |
|---|---|---|
| connect | 10s | 建连超时 |
| read | 60s | 单次网络读超时 |
| 下载墙钟 | 60s | 整次下载的 monotonic 时限，流中逐 chunk 检查 |
| 处理 | 30s | CPU/解析阶段墙钟，超时中止 |

- 大小上限 `S3_MAX_FILE_BYTES` 双向（读+写）强制，元数据预检 + 流中复查
  两道。
- 文本型输出超限截断并加显式标记（如 `[Output truncated due to size
  limits]`）为可选条目。

## 五、错误契约

规范错误码集（snake_case，置于 tool 结果层而非 MCP 协议层）：

| 错误码 | 触发 | 对应先例 |
|---|---|---|
| `S3_UNAVAILABLE` | S3 未配置（缺必填 env） | bigdata-mcp-email |
| `S3_OBJECT_NOT_FOUND` | 对象不存在 | bigdata-mcp-email |
| `S3_FETCH_FAILED` | 下载阶段其他失败（网络、403 等） | bigdata-mcp-email |
| `S3_UPLOAD_FAILED` | 上传失败 | 新增（excel-mcp 补） |
| `S3_OBJECT_TOO_LARGE` | 超过 `S3_MAX_FILE_BYTES` | 三者综合 |
| `S3_INVALID_KEY` | key 校验不通过（安全拒绝） | 综合 excel-mcp/markitdown-mcp |

规则：

- 错误在 **tool 结果层**返回：structured JSON + 同内容 text 双通道
  （bigdata-mcp-email 现行做法），**不**用 MCP 协议级错误（JSON-RPC
  error）承载业务失败。
- 错误 message 净化：不泄漏 bucket、endpoint、完整 key、凭据片段。
  安全拒绝（`S3_INVALID_KEY`）不回显被拒 key。
- `GET /health` 返回 `{"s3": "configured" | "not-configured"}`（k8s 探针
  可观测 S3 配置态）。
- 每个错误码在各语言实现中必须可区分（枚举/常量），不允许只返回字符串
  message。

## 六、先例溯源

| 规范条目 | 已验证先例 |
|---|---|
| env 变量集与回退链/path-style 推断 | excel-mcp（`src/config.rs`）、markitdown-mcp（`s3source.py`） |
| 显式 `output_key` + 读写对称白名单 | excel-mcp 0.2.0（`src/s3keys.rs`） |
| URL→key 归一化（剥签名/host） | excel-mcp（`src/s3keys.rs:65-109`）、markitdown-mcp（`security.py:472-498`） |
| 段边界白名单匹配 | excel-mcp、markitdown-mcp |
| 分层超时（connect/read/墙钟/处理） | markitdown-mcp |
| 错误码分类 + structured/text 双通道 | bigdata-mcp-email |
| 大小预检 + 流中复查 | excel-mcp（预检+复查）、markitdown-mcp（流中） |
| 临时文件前缀/后缀 + 自动清理 | excel-mcp、markitdown-mcp |
| `content_b64` 小文件内联 | excel-mcp（`EXCEL_INLINE_MAX_BYTES`） |
| 错误净化、`/health` s3 状态 | markitdown-mcp、excel-mcp |
| YAML 配置层（反面案例，不采纳） | bigdata-mcp-email |
| 私有 env 命名（反面案例，不采纳） | bigdata-mcp-email |

## 附：三项目现状差异矩阵

| 维度 | excel-mcp | markitdown-mcp | bigdata-mcp-email |
|---|---|---|---|
| 语言/库 | Rust / object_store | Python / boto3 | Go / minio-go |
| 读写 | 双向 | 只读 | 只读 |
| env 命名 | 标准命名 | 标准命名 | 私有命名（`S3_ACCESS_KEY` 等） |
| 配置源 | env | env | YAML + env 覆盖（env 须 YAML 有 `s3:` 段才生效） |
| region | 回退链 | 回退链 | 硬编码 `us-east-1` |
| path-style 推断 | 有 | 有 | 无（minio-go 默认 path） |
| key 白名单 | 读写对称 | 只读 | 无 |
| 超时 | 无 | 四层 | Get 30s + ReadHeader 10s |
| 错误码 | 无 | 净化但无码 | 六类可读码 |
| 小文件内联 | `content_b64` ≤2 MiB | 无 | 无 |
