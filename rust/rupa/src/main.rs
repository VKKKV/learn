use std::fmt::Debug;

#[derive(Debug)]
struct Package {
    name: String,
    version: String,
    is_aur: bool,
}

impl Package {
    fn new(name: &str, version: &str, is_aur: bool) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            // is_aur: is_aur,
            is_aur,
        }
    }

    fn full_id(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

trait Installer {
    fn install(&self) -> Result<(), String>;
}

impl Installer for Package {
    fn install(&self) -> Result<(), String> {
        if self.is_aur {
            println!(
                "检测到 AUR 包: {}。正在从源码编译... (这会让你的 CPU 变暖)",
                self.name
            );
        } else {
            println!("正在从官方仓库下载 {}...", self.full_id());
        }

        if self.name == "bloatware" {
            return Err("Failed to install package".to_string());
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let pkg1 = Package::new("neofetch", "7.1.0", false);
    let pkg2 = Package::new("yay", "11.3.0", true); // AUR helper，必需品

    // 把它们放入一个 Vector
    let packages = vec![pkg1, pkg2];

    println!("开始系统更新...");

    for pkg in packages {
        // 这里演示多态：我们调用的是 Installer trait 定义的方法
        // 如果 install 返回 Err，? 会立即终止 main 函数并把错误抛出去
        pkg.install()?;
    }

    // 这里的 ? 会自动处理错误，比原来的 match 简洁得多
    check_kernel()?;

    println!("系统更新完毕。I use Arch btw.");
    Ok(())
}

fn check_kernel() -> Result<(), String> {
    // 模拟检查，为了演示 ? 用法
    // 假设这是某个可能失败的操作
    let kernel_ok = true;

    if kernel_ok {
        println!("内核状态正常。");
        Ok(())
    } else {
        Err("Kernel panic!".to_string())
    }
}
