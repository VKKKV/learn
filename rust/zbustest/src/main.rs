use futures_util::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use zbus::{
    dbus_proxy,
    zvariant::{OwnedObjectPath, OwnedValue, Type, Value},
    Connection,
};

// ============================================================================
// Interfaces
// ============================================================================

mod fdo {
    use zbus::dbus_proxy;
    #[dbus_proxy(
        interface = "org.freedesktop.StatusNotifierWatcher",
        default_service = "org.freedesktop.StatusNotifierWatcher",
        default_path = "/StatusNotifierWatcher"
    )]
    pub trait StatusNotifierWatcher {
        #[dbus_proxy(property, name = "RegisteredStatusNotifierItems")]
        fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>>;

        #[dbus_proxy(signal, name = "StatusNotifierItemRegistered")]
        fn status_notifier_item_registered(&self, service: &str) -> zbus::Result<()>;
    }
}

mod kde {
    use zbus::dbus_proxy;
    #[dbus_proxy(
        interface = "org.kde.StatusNotifierWatcher",
        default_service = "org.kde.StatusNotifierWatcher",
        default_path = "/StatusNotifierWatcher"
    )]
    pub trait StatusNotifierWatcher {
        #[dbus_proxy(property, name = "RegisteredStatusNotifierItems")]
        fn registered_status_notifier_items(&self) -> zbus::Result<Vec<String>>;

        #[dbus_proxy(signal, name = "StatusNotifierItemRegistered")]
        fn status_notifier_item_registered(&self, service: &str) -> zbus::Result<()>;
    }
}

#[dbus_proxy(
    interface = "org.freedesktop.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
trait StatusNotifierItem {
    #[dbus_proxy(property)]
    fn id(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn title(&self) -> zbus::Result<String>;

    #[dbus_proxy(property)]
    fn menu(&self) -> zbus::Result<OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "com.canonical.dbusmenu",
    assume_defaults = true
)]
trait DBusMenu {
    fn get_layout(
        &self,
        parent_id: i32,
        recursion_depth: i32,
        property_names: &[&str],
    ) -> zbus::Result<(u32, LayoutNode)>;
}

// ============================================================================
// Data Structures
// ============================================================================

#[derive(Debug, Deserialize, Serialize, Type, Clone)]
pub struct LayoutNode(i32, HashMap<String, OwnedValue>, Vec<OwnedValue>);

impl LayoutNode {
    fn from_value(value: &OwnedValue) -> Option<Self> {
        let structure = match &**value {
            Value::Structure(s) => s,
            _ => return None,
        };

        let fields = structure.fields();
        if fields.len() != 3 {
            return None;
        }

        let id = match &fields[0] {
            Value::I32(i) => *i,
            _ => return None,
        };

        let props = match &fields[1] {
            Value::Dict(d) => {
                let mut map = HashMap::new();
                // In zvariant 3.x, use iterator for Dict
                for (k, v) in d.clone() {
                     if let (Value::Str(k_str), v_val) = (k, v) {
                         map.insert(k_str.to_string(), OwnedValue::from(v_val.clone()));
                     }
                }
                map
            }
            _ => HashMap::new(),
        };

        let children = match &fields[2] {
            Value::Array(a) => a
                .iter()
                .map(|v| OwnedValue::from(v.clone()))
                .collect(),
            _ => Vec::new(),
        };

        Some(LayoutNode(id, props, children))
    }
}

// ============================================================================
// Logic
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!(":: Connecting to Session Bus...");
    let conn = Connection::session().await?;

    let fdo_watcher = fdo::StatusNotifierWatcherProxy::new(&conn).await?;
    let kde_watcher = kde::StatusNotifierWatcherProxy::new(&conn).await?;

    let mut known_services = HashSet::new();

    println!(":: Initial scan...");
    if let Ok(items) = fdo_watcher.registered_status_notifier_items().await {
        for service in items {
            if known_services.insert(service.clone()) {
                inspect_item(&conn, &service).await;
            }
        }
    }

    if let Ok(items) = kde_watcher.registered_status_notifier_items().await {
        for service in items {
            if known_services.insert(service.clone()) {
                inspect_item(&conn, &service).await;
            }
        }
    }

    let mut fdo_signals = fdo_watcher.receive_status_notifier_item_registered().await?;
    let mut kde_signals = kde_watcher.receive_status_notifier_item_registered().await?;

    println!(":: Monitoring for new items...");

    loop {
        tokio::select! {
            Some(signal) = fdo_signals.next() => {
                if let Ok(args) = signal.args() {
                    let service = args.service();
                    if known_services.insert(service.to_string()) {
                        println!("\n[+] New item (FDO): {}", service);
                        inspect_item(&conn, service).await;
                    }
                }
            }
            Some(signal) = kde_signals.next() => {
                if let Ok(args) = signal.args() {
                    let service = args.service();
                    if known_services.insert(service.to_string()) {
                        println!("\n[+] New item (KDE): {}", service);
                        inspect_item(&conn, service).await;
                    }
                }
            }
        }
    }
}

async fn inspect_item(conn: &Connection, service: &str) {
    let item_proxy = match StatusNotifierItemProxy::builder(conn)
        .destination(service)
        .unwrap()
        .build()
        .await
    {
        Ok(proxy) => proxy,
        Err(e) => {
            eprintln!("   x Failed to build item proxy for {}: {}", service, e);
            return;
        }
    };

    let id = item_proxy.id().await.unwrap_or_else(|_| "Unknown".to_string());
    let title = item_proxy.title().await.unwrap_or_else(|_| "No Title".to_string());
    
    println!("   -> ID:    {}", id);
    println!("   -> Title: {}", title);

    if let Ok(menu_path) = item_proxy.menu().await {
        println!("   -> Menu:  {}", menu_path.as_str());
        inspect_menu(conn, service, &menu_path).await;
    }
}

async fn inspect_menu(conn: &Connection, service: &str, path: &OwnedObjectPath) {
    let menu_proxy = match DBusMenuProxy::builder(conn)
        .destination(service)
        .unwrap()
        .path(path)
        .unwrap()
        .build()
        .await
    {
        Ok(proxy) => proxy,
        Err(e) => {
            eprintln!("     x Failed to build menu proxy: {}", e);
            return;
        }
    };

    match menu_proxy.get_layout(0, -1, &[]).await {
        Ok((_rev, root_node)) => {
            println!("      -> Layout Tree:");
            print_tree(&root_node, 0);
        }
        Err(e) => {
            eprintln!("     x Failed to fetch menu layout: {}", e);
        }
    }
}

fn print_tree(node: &LayoutNode, depth: usize) {
    let indent = "        ".repeat(depth + 1);
    let LayoutNode(id, props, children) = node;

    let label = props.get("label").and_then(|v| {
        match &**v {
            Value::Str(s) => Some(s.as_str()),
            _ => None,
        }
    }).unwrap_or("(no label)");

    let is_visible = props.get("visible").map(|v| {
        match &**v {
            Value::Bool(b) => *b,
            _ => true,
        }
    }).unwrap_or(true);

    if !is_visible {
        return;
    }

    let marker = if children.is_empty() { "-" } else { "+" };
    println!("{} {} [ID:{}] {}", indent, marker, id, label);

    for child_val in children {
        if let Some(child_node) = LayoutNode::from_value(child_val) {
            print_tree(&child_node, depth + 1);
        }
    }
}