use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex};
use zbus::{interface, connection};
use zbus::object_server::SignalEmitter;

struct Watcher {
    items: Arc<Mutex<HashSet<String>>>,
    hosts: Arc<Mutex<HashSet<String>>>,
}

#[interface(name = "org.freedesktop.StatusNotifierWatcher")]
impl Watcher {
    // === 方法 ===

    // Item 调用此方法注册自己
    async fn register_status_notifier_item(
        &mut self,
        service: &str,
        #[zbus(signal_context)] ctxt: SignalEmitter<'_>,
    ) -> zbus::fdo::Result<()> {
        println!("[Watcher] 收到 Item 注册请求: {}", service);

        let service_name = service.to_string();

        {
            let mut items = self.items.lock().unwrap();
            if !items.contains(&service_name) {
                items.insert(service_name.clone());
            }
        }

        // 广播信号
        Self::status_notifier_item_registered(&ctxt, &service_name).await?;
        Ok(())
    }

    // Host 调用此方法注册自己
    fn register_status_notifier_host(&mut self, service: &str) -> zbus::fdo::Result<()> {
        println!("[Watcher] 收到 Host 注册请求: {}", service);
        self.hosts.lock().unwrap().insert(service.to_string());
        Ok(())
    }

    // === 属性 ===

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> {
        self.items.lock().unwrap().iter().cloned().collect()
    }

    #[zbus(property)]
    fn is_status_notifier_host_registered(&self) -> bool {
        !self.hosts.lock().unwrap().is_empty()
    }

    #[zbus(property)]
    fn protocol_version(&self) -> i32 {
        0
    }

    // === 信号 ===

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        ctxt: &SignalEmitter<'_>,
        service: &str,
    ) -> zbus::Result<()>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let watcher = Watcher {
        items: Arc::new(Mutex::new(HashSet::new())),
        hosts: Arc::new(Mutex::new(HashSet::new())),
    };

    println!(":: 启动 Watcher Daemon...");

    // [修复 2] 使用 Connection::builder() 构建连接
    // 这样才能在连接建立前就配置好 Name 和 Object Server
    let _conn = connection::Builder::session()? // 建立连接
        .name("org.freedesktop.StatusNotifierWatcher")? // 抢注名字
        .serve_at("/StatusNotifierWatcher", watcher)? // 挂载对象
        .build()
        .await?;

    println!(":: Watcher 已就绪，正在监听 (Ctrl+C 退出)...");

    // 保持进程运行，否则连接会断开
    std::future::pending().await
}
