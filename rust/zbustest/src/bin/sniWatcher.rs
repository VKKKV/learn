use futures_util::StreamExt;
use std::error::Error;
use zbus::{proxy, Connection, Result};

// ============================================================================
// 1. 定义 Watcher 接口
// Arch Tip: 这里我们用 org.kde... 因为它其实是事实标准，
// 但为了兼容你的 freedesktop 代码，我们保持 interface 名字，但逻辑更通用。
// ============================================================================
#[proxy(
    interface = "org.freedesktop.StatusNotifierWatcher",
    default_service = "org.freedesktop.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
trait StatusNotifierWatcher {
    fn register_status_notifier_host(&self, service: &str) -> Result<()>;

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Result<Vec<String>>;

    #[zbus(signal)]
    fn status_notifier_item_registered(&self, service: &str) -> Result<()>;
}

// ============================================================================
// 2. 定义 Item 接口 (动态代理)
// ============================================================================
#[proxy(
    interface = "org.freedesktop.StatusNotifierItem",
    assume_defaults = false
)]
trait StatusNotifierItem {
    fn activate(&self, x: i32, y: i32) -> Result<()>;
    fn context_menu(&self, x: i32, y: i32) -> Result<()>;

    #[zbus(property)]
    fn id(&self) -> Result<String>;
    #[zbus(property)]
    fn title(&self) -> Result<String>;
    #[zbus(property)]
    fn icon_name(&self) -> Result<String>;
}

// ============================================================================
// 3. 辅助函数：智能解析 Service 和 Path
// ============================================================================
// 很多 App (如 udiskie, ayatana) 注册时传的是 "bus_name/path" 的混合体
// 或者是单纯的 "bus_name" (隐含 path=/StatusNotifierItem)
fn parse_service_path(input: &str) -> (String, String) {
    if let Some((service, path)) = input.split_once('/') {
        // 如果输入是 ":1.52/org/ayatana/..."
        (service.to_string(), format!("/{}", path))
    } else if input.starts_with('/') {
        // 极其罕见的情况：Watcher 没给 bus name，只给了 path (通常意味着 sender 是所有者)
        // 这里需要更复杂的逻辑去推断 sender，但暂且返回 input
        (input.to_string(), input.to_string())
    } else {
        // 标准情况：":1.52" -> 默认路径
        (input.to_string(), "/StatusNotifierItem".to_string())
    }
}

// ============================================================================
// 4. Main Loop
// ============================================================================
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    println!(":: [Core] 初始化 D-Bus 连接...");
    let conn = Connection::session().await?;

    // 尝试连接 Watcher。如果 org.freedesktop 不存在，可以考虑 fallback 到 org.kde
    let watcher = match StatusNotifierWatcherProxy::new(&conn).await {
        Ok(w) => w,
        Err(e) => {
            eprintln!(":: [Fatal] 找不到 Watcher: {}. 你的系统里有跑 StatusNotifierHost 吗?", e);
            return Ok(());
        }
    };

    // 注册 Host (刷存在感)
    let my_name = conn.unique_name().map(|n| n.as_str()).unwrap_or("unknown");
    if let Err(e) = watcher.register_status_notifier_host(my_name).await {
        eprintln!(":: [Warn] 注册 Host 失败: {}", e);
    } else {
        println!(":: [Info] 已注册为 Host: {}", my_name);
    }

    // --- 1. 处理存量 (Existing) ---
    // 并行处理，而不是串行阻塞
    if let Ok(items) = watcher.registered_status_notifier_items().await {
        println!(":: [Scan] 发现 {} 个存量服务", items.len());
        for service in items {
            let conn_clone = conn.clone();
            tokio::spawn(async move {
                process_item(conn_clone, service).await;
            });
        }
    }

    // --- 2. 监听增量 (Real-time) ---
    println!(":: [Loop] 进入事件监听模式...");
    let mut stream = watcher.receive_status_notifier_item_registered().await?;

    while let Some(signal) = stream.next().await {
        let args = signal.args();
        match args {
            Ok(a) => {
                let raw_service = a.service.to_string();
                println!("\n:: [Event] 新服务注册: {}", raw_service);
                let conn_clone = conn.clone();
                // 立即 spawn，不要 sleep 5秒！
                tokio::spawn(async move {
                    // 短暂 yield 让对象在总线上就绪 (50ms 足够了，5秒太扯淡)
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    process_item(conn_clone, raw_service.to_string()).await;
                });
            }
            Err(e) => eprintln!(":: [Error] 解析信号失败: {}", e),
        }
    }

    Ok(())
}

// ============================================================================
// 5. 业务逻辑
// ============================================================================
async fn process_item(conn: Connection, raw_input: String) {
    let (service, path) = parse_service_path(&raw_input);
    
    // 构建 Proxy
    let proxy_builder = StatusNotifierItemProxy::builder(&conn)
        .destination(service.clone())
        .and_then(|b| b.path(path.clone()));

    let item = match proxy_builder {
        Ok(b) => match b.build().await {
            Ok(p) => p,
            Err(e) => {
                eprintln!("   x [Fail] 无法建立连接 {}@{}: {}", service, path, e);
                return;
            }
        },
        Err(e) => {
             eprintln!("   x [Fail] 路径错误: {}", e);
             return;
        }
    };

    // 并行获取属性 (优化点：不需要等一个属性拿完再拿下一个)
    // 使用 join! 宏可以同时发起请求，大幅减少等待时间
    let (id, title, icon) = tokio::join!(
        item.id(),
        item.title(),
        item.icon_name()
    );

    let id = id.unwrap_or_else(|_| "Unknown".into());
    let title = title.unwrap_or_else(|_| "".into());
    let icon = icon.unwrap_or_else(|_| "".into());

    println!("   -> [Item] ID: {:<15} | Icon: {:<15} | Title: {}", id, icon, title);

    // DANGEROUS ZONE: 只有在某些特定条件下才去点击，别瞎点
    // 这里我们只演示，不实际点击，以免你测试时把电脑点炸了
    // 实际使用时，你可以把这里的逻辑绑定到 CLI 参数上
    /*
    if id == "udiskie" {
        println!("   -> [Auto] 自动激活 udiskie...");
        let _ = item.activate(0, 0).await;
    }
    */
}
