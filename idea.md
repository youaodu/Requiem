# Requiem - 轻量级 HTTP 请求工具

**创建日期**: 2025-10-14
**版本**: Draft 2
**状态**: 需求定义完成

---

## 📋 项目概述

### 项目名称
**Requiem** - 寓意"安魂曲"，也暗含 Request（请求）的意思

### 项目定位
面向开发者的**极简、高性能、原生 HTTP 客户端**，对标 Postman 但更轻量、更快、更 Geek。

### 核心价值主张
- 🪶 **极致轻量**: 内存占用 < 200MB（目标 < 100MB），是 Postman 的 1/3-1/5
- ⚡ **闪电启动**: < 1秒冷启动，告别漫长等待
- 🔌 **离线优先**: 无需登录、云服务，完全本地化
- 📁 **Git 友好**: 纯文本存储，天然支持版本控制
- 🎯 **功能完整**: API调试、管理、性能测试、文档生成一体化

---

## 🎯 核心问题与解决方案

### 现有工具的痛点

| 工具 | 内存占用 | 启动时间 | 主要问题 |
|------|---------|---------|---------|
| Postman | 300-500MB | 5-10秒 | 臃肿、强制登录、功能过载 |
| Insomnia | 200-400MB | 3-5秒 | 仍然偏重、Electron架构 |
| Bruno | 150-300MB | 2-4秒 | 较好但仍是Electron |
| HTTPie Desktop | 180-350MB | 3-5秒 | 功能相对简单 |

### Requiem 的解决方案

```
技术栈: Rust + iced (纯原生)
  ↓
预期内存: 50-100MB (极致优化后 < 200MB)
  ↓
启动时间: < 1秒
  ↓
结果: 性能提升 3-5 倍
```

**关键优势**:
- 使用 Rust 的零成本抽象，无 GC 开销
- iced 原生渲染，无 Chromium 内核负担
- 精心设计的数据结构，最小化内存分配
- 懒加载机制，按需加载功能模块

---

## 👥 目标用户与使用场景

### 目标用户
- **后端开发者**: 日常 API 调试与测试
- **前端开发者**: 接口联调与数据验证
- **测试工程师**: API 自动化测试
- **技术文档编写者**: API 文档生成与维护
- **性能工程师**: API 性能测试与优化

### 核心使用场景

#### 1. API 调试
```
场景: 开发新接口，需要快速测试参数和响应
需求: 快速构建请求、查看响应、保存历史
```

#### 2. API 管理
```
场景: 维护多个项目的 API 集合
需求: 按公司/项目/模块组织、环境变量管理
```

#### 3. 性能测试
```
场景: 验证接口性能，发现瓶颈
需求: 并发测试、压力曲线、延迟分布（P50/P95/P99）
```

#### 4. 文档演示
```
场景: 向团队展示 API 使用方法
需求: 生成美观的 HTML 文档，包含示例和说明
```

---

## 🔧 技术栈选型

### 核心技术
- **语言**: Rust
  - 内存安全、零成本抽象
  - 高性能、低内存占用
  - 丰富的网络库生态（reqwest, tokio）

- **UI 框架**: iced
  - 纯原生渲染，无 Web 引擎开销
  - Elm 架构，状态管理清晰
  - 跨平台（Linux, macOS, Windows）
  - 活跃维护，社区成长中

### 为什么选择 Rust + iced？

#### 对比其他方案

| 方案 | 内存占用 | 开发效率 | 性能 | 跨平台 | 评价 |
|------|---------|---------|------|--------|------|
| Electron | ❌ 高 (200-500MB) | ✅ 高 | ❌ 低 | ✅ 好 | 太臃肿 |
| Tauri | ⚠️ 中 (80-150MB) | ✅ 高 | ⚠️ 中 | ✅ 好 | 仍依赖 WebView |
| Flutter | ⚠️ 中 (100-200MB) | ✅ 高 | ✅ 高 | ✅ 好 | 备选方案 |
| Qt/C++ | ✅ 低 (30-80MB) | ❌ 低 | ✅ 高 | ✅ 好 | 开发复杂 |
| **Rust + iced** | ✅ **低 (50-100MB)** | ⚠️ **中** | ✅ **高** | ✅ **好** | ✅ **最优** |

### 依赖库预选

```toml
[dependencies]
# UI框架
iced = "0.12"
iced_aw = "0.8"  # 额外组件

# HTTP客户端
reqwest = { version = "0.11", features = ["json", "multipart", "stream"] }
tokio = { version = "1", features = ["full"] }

# 序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"

# 语法高亮
syntect = "5"

# 性能测试引擎（Blast模式）
tokio-metrics = "0.3"
hdrhistogram = "7"

# 文档生成
tera = "1"  # 模板引擎
pulldown-cmark = "0.9"  # Markdown渲染

# 其他
anyhow = "1"
chrono = "0.4"
uuid = "1"
```

---

## 🎨 UI/UX 设计

### 布局方案：三栏经典布局

```
┌─────────────────────────────────────────────────────────────┐
│  Requiem                                    [_ □ ×]         │
├────────────┬──────────────────────┬─────────────────────────┤
│            │                      │                         │
│  [集合管理]  │    [请求编辑器]       │      [响应面板]         │
│            │                      │                         │
│  📁 公司A   │  GET  🔽             │  Status: 200 OK         │
│   ├─项目1   │  https://api.com/   │  Time: 234ms            │
│   │ ├─Users │                      │  Size: 1.2KB            │
│   │ │ ├─获取 │  ┌─ Headers         │                         │
│   │ │ └─创建 │  ├─ Body            │  ┌─ Body               │
│   │ └─Payments│ ├─ Auth           │  ├─ Headers             │
│   └─项目2   │  ├─ Query Params    │  ├─ Cookies            │
│            │  └─ Scripts          │  └─ Timeline           │
│  📁 公司B   │                      │                         │
│   └─项目3   │  [Send] [Save]      │  {                      │
│            │                      │    "status": "success"  │
│  🌍 环境    │                      │  }                      │
│   ├─Dev    │                      │                         │
│   ├─Staging│                      │  [Copy] [Format] [...]  │
│   └─Prod   │                      │                         │
│            │                      │                         │
│  📊 性能测试 │                      │                         │
│  📝 文档生成 │                      │                         │
│            │                      │                         │
└────────────┴──────────────────────┴─────────────────────────┘
```

### 设计原则

#### 1. 极简美学
- **配色**: 基于材质设计，支持亮/暗主题
- **字体**: JetBrains Mono（代码）+ Inter（UI）
- **间距**: 8px 网格系统，保持视觉呼吸感
- **图标**: Lucide Icons（简洁现代）

#### 2. 效率优先
- **快捷键**: 全局快捷键体系（Vim风格可选）
- **搜索**: Cmd/Ctrl + P 快速跳转到任意请求
- **历史**: Cmd/Ctrl + H 快速访问历史记录
- **环境切换**: Cmd/Ctrl + E 快速切换环境

#### 3. 信息密度
- **紧凑模式**: 可选择紧凑视图，显示更多内容
- **自适应**: 根据窗口大小自动调整布局
- **折叠**: 所有面板可折叠，最大化工作区

---

## 📂 文件结构与数据格式

### 工作空间结构

```
~/.requiem/                      # 全局配置目录
├── config.json                  # 全局设置
├── themes/                      # 自定义主题
├── templates/                   # 文档模板
└── plugins/                     # 插件（未来）

~/workspace/                     # 用户工作空间
├── CompanyA/                    # 公司/组织级别
│   ├── Project1/                # 项目级别
│   │   ├── collections/         # 请求集合
│   │   │   ├── Users/           # 模块分类
│   │   │   │   ├── get-user.json
│   │   │   │   ├── create-user.json
│   │   │   │   └── update-user.json
│   │   │   └── Payments/
│   │   │       ├── create-charge.json
│   │   │       └── refund.json
│   │   ├── environments/        # 环境配置
│   │   │   ├── dev.json
│   │   │   ├── staging.json
│   │   │   └── prod.json
│   │   ├── history/             # 请求历史（可选，gitignore）
│   │   │   └── 2025-10-14.jsonl
│   │   └── config.json          # 项目配置
│   └── Project2/
└── CompanyB/
    └── Project3/
```

### 数据格式规范

#### 1. 请求文件格式 (`*.json`)

```json
{
  "meta": {
    "id": "req_123456",
    "name": "获取用户信息",
    "description": "根据用户ID获取详细信息",
    "tags": ["user", "api"],
    "created_at": "2025-10-14T10:00:00Z",
    "updated_at": "2025-10-14T11:30:00Z"
  },
  "request": {
    "method": "GET",
    "url": "{{baseUrl}}/api/users/{{userId}}",
    "headers": [
      {
        "key": "Authorization",
        "value": "Bearer {{token}}",
        "enabled": true
      },
      {
        "key": "Content-Type",
        "value": "application/json",
        "enabled": true
      }
    ],
    "query": [
      {
        "key": "include",
        "value": "profile,settings",
        "enabled": true
      }
    ],
    "body": {
      "type": "none"  // none, json, form, multipart, raw, binary
    },
    "auth": {
      "type": "inherit"  // inherit, none, basic, bearer, oauth2
    },
    "scripts": {
      "pre_request": "// 预处理脚本\nconst timestamp = Date.now();",
      "post_response": "// 后处理脚本\nif (response.status === 200) {\n  env.set('userId', response.data.id);\n}"
    }
  },
  "examples": [
    {
      "name": "成功响应",
      "status": 200,
      "headers": {},
      "body": {
        "id": 123,
        "name": "John Doe",
        "email": "john@example.com"
      }
    }
  ],
  "docs": {
    "description": "## 获取用户信息\n\n根据用户ID获取用户的详细信息...",
    "notes": "需要管理员权限"
  }
}
```

#### 2. 环境配置格式 (`environments/*.json`)

```json
{
  "name": "Development",
  "variables": [
    {
      "key": "baseUrl",
      "value": "https://dev.api.example.com",
      "type": "default",  // default, secret
      "enabled": true
    },
    {
      "key": "token",
      "value": "dev_token_12345",
      "type": "secret",
      "enabled": true
    },
    {
      "key": "userId",
      "value": "",
      "type": "default",
      "enabled": true,
      "description": "当前操作的用户ID（运行时设置）"
    }
  ]
}
```

#### 3. 项目配置 (`config.json`)

```json
{
  "name": "Project1",
  "version": "1.0.0",
  "default_environment": "dev",
  "settings": {
    "request_timeout": 30000,
    "follow_redirects": true,
    "verify_ssl": true,
    "proxy": null
  },
  "git": {
    "ignore_history": true,
    "ignore_secrets": true
  }
}
```

### Git 工作流集成

**`.gitignore` 建议**:
```gitignore
# 忽略历史记录（包含敏感数据）
history/

# 忽略本地环境变量（如有敏感token）
environments/local.json

# 忽略临时文件
*.tmp
.DS_Store
```

**团队协作流程**:
```
1. 开发者A创建新请求 → 提交到Git
2. 开发者B拉取最新集合 → 本地添加自己的环境变量
3. 冲突解决：使用标准Git工具（meld, kdiff3等）
```

---

## ⚡ 核心功能规划

### MVP (v0.1.0) - 基础功能

#### 1. HTTP 请求管理
- [x] 支持 GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
- [x] URL 编辑与变量替换
- [x] Headers 管理
- [x] Query Parameters
- [x] Body 支持：
  - JSON
  - Form-urlencoded
  - Multipart/Form-data
  - Raw Text
  - Binary File

#### 2. 认证支持
- [x] No Auth
- [x] Bearer Token
- [x] Basic Auth
- [x] API Key
- [ ] OAuth 2.0（v0.2）

#### 3. 环境与变量
- [x] 多环境配置（Dev/Staging/Prod）
- [x] 变量插值 `{{variable}}`
- [x] 动态变量（`{{$timestamp}}`, `{{$uuid}}`）
- [x] 从响应中提取变量（点击提取）

#### 4. 请求集合
- [x] 文件夹组织（公司 > 项目 > 分类）
- [x] 拖拽排序
- [x] 搜索与过滤
- [x] 批量操作（导入/导出）

#### 5. 响应处理
- [x] 语法高亮（JSON, XML, HTML, CSS, JS）
- [x] 格式化与美化
- [x] 原始/预览模式切换
- [x] 图片预览
- [x] 响应时间、大小统计
- [x] Cookie 查看

#### 6. 历史记录
- [x] 自动保存请求历史
- [x] 按时间/项目筛选
- [x] 快速重放
- [ ] 历史对比（v0.2）

#### 7. 代码生成
- [x] cURL
- [x] JavaScript (Fetch/Axios)
- [x] Python (Requests)
- [x] Go (net/http)
- [x] Rust (reqwest)

---

### v0.2.0 - 进阶功能

#### 1. 性能测试 (Blast Mode)
```
调用独立CLI工具 (requiem-blast)
- 并发数配置 (1-10000)
- 持续时间/请求数
- 阶梯式加压
- 实时统计：
  * QPS (Queries Per Second)
  * 延迟分布 (P50, P90, P95, P99)
  * 错误率
  * 吞吐量
- 图表展示：
  * 延迟曲线
  * QPS趋势
  * 错误分布
```

**实现方案**:
```rust
// 主应用调用
let handle = tokio::spawn(async {
    let output = Command::new("requiem-blast")
        .arg("--config")
        .arg(test_config_path)
        .output()
        .await?;

    // 解析结果并展示
    parse_blast_results(output)
});
```

#### 2. API 文档生成

**文档模板规范**:
```html
<!-- templates/default.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{{ project.name }} API Documentation</title>
    <style>
        /* 默认样式 */
    </style>
</head>
<body>
    <nav>
        {% for collection in collections %}
        <div class="collection">
            <h3>{{ collection.name }}</h3>
            {% for request in collection.requests %}
            <a href="#{{ request.id }}">{{ request.name }}</a>
            {% endfor %}
        </div>
        {% endfor %}
    </nav>

    <main>
        {% for collection in collections %}
        {% for request in collection.requests %}
        <section id="{{ request.id }}">
            <h2>{{ request.method }} {{ request.name }}</h2>
            <p>{{ request.description }}</p>

            <h3>Request</h3>
            <pre><code>{{ request.method }} {{ request.url }}</code></pre>

            <h4>Headers</h4>
            <table>
                {% for header in request.headers %}
                <tr>
                    <td>{{ header.key }}</td>
                    <td>{{ header.value }}</td>
                </tr>
                {% endfor %}
            </table>

            <h3>Examples</h3>
            {% for example in request.examples %}
            <div class="example">
                <h4>{{ example.name }}</h4>
                <pre><code>{{ example.body | json }}</code></pre>
            </div>
            {% endfor %}
        </section>
        {% endfor %}
        {% endfor %}
    </main>
</body>
</html>
```

**用户自定义模板**:
- 放置在 `~/.requiem/templates/`
- 使用 Tera 模板引擎语法
- 支持变量、循环、条件渲染
- 可导入自定义CSS/JS

**导出功能**:
```
文件 → 导出文档
  ├─ 选择模板（默认/自定义）
  ├─ 选择范围（当前集合/整个项目）
  ├─ 导出路径
  └─ 生成 index.html（单文件，可离线查看）
```

#### 3. WebSocket 调试
- 连接管理
- 消息收发
- 消息历史
- JSON格式化

#### 4. GraphQL 支持
- Schema 浏览
- Query 构建器
- Variables 管理
- 响应展示

---

### v0.3.0 - 高级功能

#### 1. 请求流编排
```
场景：多步骤测试流程
1. 登录 → 提取 token
2. 创建用户 → 提取 userId
3. 更新用户资料
4. 验证更新结果
```

**实现**:
```json
{
  "flow": {
    "name": "用户完整流程测试",
    "steps": [
      {
        "request_id": "req_login",
        "extract": {
          "token": "response.body.data.token"
        }
      },
      {
        "request_id": "req_create_user",
        "extract": {
          "userId": "response.body.data.id"
        },
        "assertions": [
          "response.status === 201"
        ]
      }
    ]
  }
}
```

#### 2. Mock Server
```
从集合一键启动本地Mock服务
- 自动根据examples生成响应
- 支持延迟模拟
- 支持随机数据生成
- 可配置错误率
```

#### 3. 插件系统
```
~/.requiem/plugins/
  ├─ auth-helper/          # 认证辅助
  ├─ data-generator/       # 测试数据生成
  └─ custom-validator/     # 自定义验证器
```

#### 4. 团队协作（付费功能）
- 局域网P2P同步（无需云服务）
- 实时协作编辑
- 变更通知
- 权限管理

---

## 🏗️ 架构设计

### 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                      Requiem App                        │
├─────────────────────────────────────────────────────────┤
│  UI Layer (iced)                                        │
│  ├─ Collection Panel                                    │
│  ├─ Request Editor                                      │
│  ├─ Response Viewer                                     │
│  └─ Settings Dialog                                     │
├─────────────────────────────────────────────────────────┤
│  Application Layer                                      │
│  ├─ State Management (Elm Architecture)                │
│  ├─ Request Builder                                     │
│  ├─ Variable Resolver                                   │
│  ├─ Script Engine (Rhai)                                │
│  └─ Event Bus                                           │
├─────────────────────────────────────────────────────────┤
│  Core Layer                                             │
│  ├─ HTTP Client (reqwest)                               │
│  ├─ WebSocket Client                                    │
│  ├─ GraphQL Client                                      │
│  ├─ File Manager                                        │
│  ├─ History Manager                                     │
│  └─ Environment Manager                                 │
├─────────────────────────────────────────────────────────┤
│  Infrastructure Layer                                   │
│  ├─ Config Loader                                       │
│  ├─ Template Engine (Tera)                              │
│  ├─ Code Generator                                      │
│  ├─ Syntax Highlighter (Syntect)                        │
│  └─ File Watcher                                        │
└─────────────────────────────────────────────────────────┘

External:
  ├─ requiem-blast (性能测试CLI)
  └─ requiem-mock (Mock Server)
```

### 状态管理 (Elm Architecture)

```rust
// Message 定义
#[derive(Debug, Clone)]
enum Message {
    // Collection
    SelectRequest(RequestId),
    CreateRequest,
    DeleteRequest(RequestId),

    // Request Editor
    UpdateUrl(String),
    UpdateMethod(HttpMethod),
    AddHeader { key: String, value: String },
    UpdateBody(Body),

    // Execution
    SendRequest,
    RequestCompleted(Result<Response>),

    // Environment
    SwitchEnvironment(String),
    UpdateVariable { key: String, value: String },
}

// State 定义
struct AppState {
    workspace: Workspace,
    selected_request: Option<RequestId>,
    current_environment: String,
    request_history: Vec<HistoryEntry>,
    ui_state: UiState,
}

// Update 函数
fn update(state: &mut AppState, message: Message) -> Command<Message> {
    match message {
        Message::SendRequest => {
            let request = state.get_current_request();
            let resolved = resolve_variables(request, &state.current_environment);
            Command::perform(
                send_http_request(resolved),
                Message::RequestCompleted
            )
        }
        // ... 其他处理
    }
}
```

### 性能优化策略

#### 1. 内存优化
```rust
// 使用 Arc 共享数据，避免深拷贝
type SharedRequest = Arc<Request>;

// 懒加载响应体
struct Response {
    headers: Headers,
    status: u16,
    body: LazyBody,  // 只在需要时加载到内存
}

// 限制历史记录数量
const MAX_HISTORY_ENTRIES: usize = 1000;

// 使用内存池复用buffer
static BUFFER_POOL: Lazy<Pool<Vec<u8>>> = Lazy::new(|| {
    Pool::new(32, || Vec::with_capacity(64 * 1024))
});
```

#### 2. 响应式渲染
```rust
// 只在数据变化时重新渲染
fn view(&self) -> Element<Message> {
    if self.cache.is_valid() {
        return self.cache.view.clone();
    }

    // 重新构建视图
    let view = self.build_view();
    self.cache.update(view.clone());
    view
}

// 虚拟滚动（长列表优化）
struct VirtualList {
    total_items: usize,
    visible_range: Range<usize>,  // 只渲染可见项
}
```

#### 3. 启动优化
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 1. 异步加载配置
    let config_loader = tokio::spawn(load_config());

    // 2. 先显示窗口（占位UI）
    let window = create_window();

    // 3. 后台加载数据
    let workspace = config_loader.await?;

    // 4. 渲染完整UI
    window.update(workspace);
}
```

#### 4. 文件IO优化
```rust
// 使用内存映射读取大文件
use memmap2::Mmap;

fn load_large_response(path: &Path) -> Result<&[u8]> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    Ok(&mmap[..])
}

// 增量序列化（只保存变更部分）
fn save_request_incremental(request: &Request) -> Result<()> {
    let path = request.file_path();
    let mut file = OpenOptions::new().write(true).open(path)?;

    // 只写入变更的字段
    if request.is_dirty(Field::Url) {
        write_field(&mut file, "url", &request.url)?;
    }
    if request.is_dirty(Field::Headers) {
        write_field(&mut file, "headers", &request.headers)?;
    }

    Ok(())
}
```

---

## 💰 商业模式

### 开源 vs 付费功能划分

#### 免费开源功能（MIT License）
- ✅ 完整的HTTP请求功能
- ✅ 环境与变量管理
- ✅ 请求集合组织
- ✅ 响应查看与格式化
- ✅ 代码生成
- ✅ 请求历史
- ✅ 基础文档生成
- ✅ WebSocket 调试
- ✅ GraphQL 支持
- ✅ 基础性能测试（并发 < 100）

#### 付费高级功能（Pro License）

**Tier 1: Pro - $49/年（个人）**
- ⭐ 高级性能测试（并发 > 100，完整Blast模式）
- ⭐ 自定义文档模板
- ⭐ 请求流编排
- ⭐ Mock Server
- ⭐ 高级代码生成（更多语言和库）
- ⭐ 主题商店
- ⭐ 插件系统

**Tier 2: Team - $199/年（5人团队）**
- ⭐ Pro 的所有功能
- ⭐ 局域网P2P同步
- ⭐ 实时协作
- ⭐ 团队权限管理
- ⭐ 审计日志
- ⭐ SSO集成

**Tier 3: Enterprise - 定制报价**
- ⭐ Team 的所有功能
- ⭐ 私有部署
- ⭐ 定制开发
- ⭐ 专属技术支持
- ⭐ SLA保障

### 营收预测（乐观估计）

**第一年（v1.0发布后）**:
```
目标用户：
- GitHub Stars: 5000+
- 月活用户: 10,000
- 付费转化率: 2%
- Pro用户: 200 × $49 = $9,800/年
- Team用户: 10 × $199 = $1,990/年
---
第一年总营收: ~$12,000
```

**第二年**:
```
- 月活用户: 50,000
- 付费转化率: 3%
- Pro用户: 1,500 × $49 = $73,500/年
- Team用户: 50 × $199 = $9,950/年
---
第二年总营收: ~$83,000
```

### 推广策略

1. **开源社区**
   - GitHub重点推广
   - Hacker News发布
   - Reddit (r/rust, r/webdev, r/programming)
   - Dev.to 技术博客
   - Product Hunt 产品发布

2. **技术博客**
   - 《为什么我用Rust重写了Postman》
   - 《Requiem：内存占用只有Postman的1/5》
   - 《从零开始用iced构建跨平台桌面应用》

3. **视频教程**
   - YouTube: 产品演示、使用教程
   - Bilibili: 中文教程

4. **开发者社区**
   - 参加RustConf、DevOps会议
   - 技术播客采访
   - 在线研讨会

---

## 📅 开发路线图

### Phase 1: MVP开发（3-4个月）

**Month 1: 核心架构**
- Week 1-2: 项目搭建、UI框架选型验证
- Week 3-4: HTTP客户端封装、基础状态管理

**Month 2: 基础功能**
- Week 1: 请求编辑器（URL、Headers、Body）
- Week 2: 响应查看器（格式化、高亮）
- Week 3: 集合管理（CRUD、文件操作）
- Week 4: 环境变量系统

**Month 3: 完善功能**
- Week 1: 认证支持（Bearer、Basic、API Key）
- Week 2: 代码生成
- Week 3: 历史记录
- Week 4: Bug修复、性能优化

**Month 4: Alpha测试**
- Week 1-2: 内部测试、修复关键问题
- Week 3: 文档编写
- Week 4: Alpha发布（GitHub Release）

### Phase 2: 进阶功能（2-3个月）

**Month 5: 性能测试**
- Week 1-2: requiem-blast CLI开发
- Week 3-4: UI集成、图表展示

**Month 6: 文档生成**
- Week 1-2: 模板引擎集成、默认模板
- Week 3-4: 自定义模板支持、导出功能

**Month 7: WebSocket & GraphQL**
- Week 1-2: WebSocket 客户端
- Week 3-4: GraphQL 支持

### Phase 3: 高级功能与商业化（3-4个月）

**Month 8-9: 高级功能**
- 请求流编排
- Mock Server
- 插件系统基础

**Month 10: 付费功能开发**
- License验证系统
- 付费功能隔离
- 支付集成（Stripe/Paddle）

**Month 11: Beta测试**
- 公开Beta
- 收集反馈、快速迭代

**Month 12: v1.0正式发布**
- 营销推广
- Product Hunt发布
- Hacker News讨论

---

## 🆚 竞品分析

### 详细对比

| 特性 | Requiem | Postman | Insomnia | Bruno | HTTPie Desktop |
|------|---------|---------|----------|-------|----------------|
| **内存占用** | 50-100MB | 300-500MB | 200-400MB | 150-300MB | 180-350MB |
| **启动时间** | <1秒 | 5-10秒 | 3-5秒 | 2-4秒 | 3-5秒 |
| **技术栈** | Rust+iced | Electron | Electron | Electron | Electron |
| **离线使用** | ✅ 完全 | ⚠️ 部分 | ✅ 是 | ✅ 是 | ✅ 是 |
| **Git友好** | ✅ 纯文本 | ❌ 二进制 | ⚠️ 部分 | ✅ 纯文本 | ⚠️ 部分 |
| **开源** | ✅ MIT | ❌ 闭源 | ✅ MIT | ✅ MIT | ❌ 闭源 |
| **价格** | 免费+Pro | 免费+$12/月 | 免费 | 免费 | 免费+付费 |
| **性能测试** | ✅ 高级 | ✅ 高级 | ❌ 无 | ❌ 无 | ⚠️ 基础 |
| **文档生成** | ✅ 静态HTML | ✅ 云端 | ⚠️ 基础 | ⚠️ 基础 | ❌ 无 |
| **团队协作** | ✅ P2P | ✅ 云同步 | ❌ 无 | ⚠️ Git | ❌ 无 |

### Requiem 的竞争优势

1. **性能优势**
   - 内存占用最低（3-5倍差距）
   - 启动速度最快
   - 响应流畅度高

2. **开发者友好**
   - 完全离线
   - Git原生支持
   - 纯文本格式，易于diff和合并
   - 无需注册登录

3. **功能完整性**
   - 集成性能测试（多数竞品没有）
   - 高级文档生成
   - 灵活的自定义能力

4. **商业模式**
   - 核心功能永久免费
   - 付费功能合理且高价值
   - 不绑架用户数据

### 劣势与应对

| 劣势 | 应对策略 |
|------|---------|
| 品牌知名度低 | 开源社区推广、技术博客、口碑营销 |
| 生态不成熟 | 优先支持主流格式导入（Postman/Insomnia） |
| UI可能不如Electron精美 | 精心设计，利用iced的性能优势补偿 |
| 插件生态缺失 | 开放插件API，鼓励社区开发 |

---

## 🚧 潜在挑战与解决方案

### 技术挑战

#### 1. iced 框架成熟度
**挑战**: iced 相对年轻，组件不如成熟UI框架丰富

**解决方案**:
- 贡献给 iced 社区，推动生态发展
- 必要时自己实现缺失组件（代码编辑器、图表等）
- 关注 iced_aw（额外组件库）
- 考虑使用 egui 作为备选（更成熟但架构不同）

#### 2. 跨平台一致性
**挑战**: Linux/macOS/Windows 渲染和交互差异

**解决方案**:
- 早期同时在三平台测试
- 使用CI/CD自动化测试（GitHub Actions）
- 建立平台特定的测试用例

#### 3. 代码编辑器实现
**挑战**: 需要一个高性能的代码编辑器组件（语法高亮、自动补全）

**解决方案**:
- Phase 1: 使用简单文本框 + syntect 高亮
- Phase 2: 集成或自建轻量级编辑器
- 参考: xi-editor (Rust), CodeMirror (可借鉴思路)

#### 4. 性能测试精度
**挑战**: 高并发测试的准确性和稳定性

**解决方案**:
- 使用成熟的测试引擎（参考wrk、vegeta）
- Tokio + hdrhistogram 精确测量延迟
- 提供测试报告可信度评估

### 产品挑战

#### 1. 用户习惯迁移
**挑战**: Postman用户习惯根深蒂固

**解决方案**:
- 提供 Postman Collection 导入工具
- 制作迁移指南
- 快捷键可配置（支持Postman风格）
- 强调差异化价值（性能、隐私）

#### 2. 社区建设
**挑战**: 从零开始建立社区

**解决方案**:
- GitHub Discussions 活跃互动
- Discord 社区（实时交流）
- 定期发布 Roadmap 和进展
- 快速响应 Issue 和 PR
- 举办线上活动（Hackathon、AMA）

#### 3. 商业化平衡
**挑战**: 免费和付费功能的边界

**解决方案**:
- 核心功能永久免费，确保基础用户满意
- 付费功能是"锦上添花"，不影响核心体验
- 开源社区保持活跃，避免被视为"伪开源"
- 透明的定价和Roadmap

---

## 🎯 成功指标

### 技术指标

- **内存占用**: < 100MB（目标 < 80MB）
- **启动时间**: < 1秒（目标 < 0.5秒）
- **安装包大小**: < 50MB（目标 < 30MB）
- **响应流畅度**: 60 FPS UI渲染
- **构建时间**: < 5分钟（全量编译）

### 产品指标

**第一年（MVP后）**:
- GitHub Stars: 5,000+
- Monthly Active Users: 10,000+
- GitHub Issues 响应时间: < 48小时
- 用户留存率（30天）: > 40%
- NPS（净推荐值）: > 50

**第二年**:
- GitHub Stars: 15,000+
- Monthly Active Users: 50,000+
- 付费用户: 1,500+
- 社区贡献者: 50+
- 插件数量: 20+

### 商业指标

- 第一年营收: $10,000+
- 第二年营收: $80,000+
- 付费转化率: 2-3%
- 用户LTV（生命周期价值）: > $150
- CAC（获客成本）: < $50

---

## 🔥 关键风险

### 高风险

1. **技术选型风险**
   - iced 可能不够成熟，导致开发受阻
   - **缓解**: 预留时间做技术验证（1-2周），必要时切换到 egui

2. **市场接受度**
   - 用户不愿意从 Postman 迁移
   - **缓解**: 做好导入工具，降低迁移成本；强调独特价值

### 中风险

3. **开发周期超期**
   - Rust 学习曲线，开发效率可能低于预期
   - **缓解**: 预留 buffer 时间（30%），MVP 范围可缩减

4. **性能未达预期**
   - 内存优化困难，未能显著优于竞品
   - **缓解**: 早期做性能 profiling，持续优化

### 低风险

5. **商业化失败**
   - 付费功能吸引力不足
   - **缓解**: 开源核心功能已有价值，商业化只是加分项

---

## 🚀 下一步行动

### 立即行动（本周）

1. **技术验证**
   - [ ] 搭建 Rust + iced 基础项目
   - [ ] 实现简单的 HTTP 请求功能（reqwest）
   - [ ] 测试 iced 的代码编辑器组件可行性
   - [ ] 测量内存占用和启动时间基准

2. **设计验证**
   - [ ] 绘制详细的UI mockup（Figma）
   - [ ] 定义配色方案（亮/暗主题）
   - [ ] 设计 icon set

3. **项目启动**
   - [ ] 创建 GitHub 仓库 `requiem`
   - [ ] 编写 README 和 CONTRIBUTING
   - [ ] 设置 CI/CD（GitHub Actions）
   - [ ] 选择合适的开源协议（MIT）

### 短期目标（1个月）

1. **核心架构**
   - [ ] 实现基础状态管理（Elm架构）
   - [ ] 完成 HTTP 客户端封装
   - [ ] 实现文件读写模块
   - [ ] 搭建三栏布局 UI

2. **社区准备**
   - [ ] 编写开发文档
   - [ ] 准备技术博客草稿
   - [ ] 创建 Discord 服务器

### 中期目标（3个月）

1. **功能完成**
   - [ ] 完成 MVP 所有功能
   - [ ] 通过内部测试
   - [ ] 发布 Alpha 版本

2. **社区建设**
   - [ ] 发布技术博客
   - [ ] Hacker News 讨论
   - [ ] 邀请早期用户测试

---

## 📝 待讨论问题

1. **技术细节**
   - iced 的代码编辑器实现方案？
   - 脚本引擎选择（Rhai vs Lua vs JavaScript）？
   - 跨平台构建和分发策略？

2. **产品定位**
   - 主要面向独立开发者还是团队？
   - 是否考虑支持 gRPC/其他协议？
   - 移动端是否在规划内（远期）？

3. **商业模式**
   - 付费功能的具体划分需要更细化？
   - 是否考虑企业级 on-premise 部署？
   - 定价策略需要市场调研？

4. **开源策略**
   - 如何平衡开源贡献和商业利益？
   - 是否需要 CLA（贡献者许可协议）？
   - 如何防止竞品直接 fork 并商业化？

---

## 🎉 总结

Requiem 是一个有明确目标和差异化优势的项目：

**核心竞争力**:
- ⚡ 极致性能（内存、启动速度）
- 🔌 离线优先、隐私友好
- 📁 Git 原生支持
- 🛠️ 功能完整（测试、文档一体化）

**可行性**:
- ✅ 技术栈成熟（Rust 生态健全）
- ✅ 市场需求真实存在（Postman 臃肿是普遍痛点）
- ✅ 商业模式清晰（开源 + 付费高级功能）

**风险可控**:
- 技术选型有备选方案
- MVP 范围适中，可快速验证
- 商业化不是唯一目标，开源本身有价值

**建议**:
1. 先做 1-2 周的技术验证，确保 iced 可行
2. MVP 阶段专注核心功能，不要贪多
3. 尽早开源，积累社区
4. 保持迭代速度，快速响应反馈

---

**最后的思考**:

这个项目最大的价值不只是"又一个 HTTP 客户端"，而是：
- 证明 Rust + 原生 UI 可以做出优秀的桌面应用
- 为开发者提供真正尊重隐私、性能优先的工具
- 建立一个健康的开源社区和商业模式共存的案例

如果执行得当，Requiem 有潜力成为**下一代 API 工具的标杆**。

---

**准备好开始了吗？🚀**

