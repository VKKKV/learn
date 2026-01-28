use nix::sys::sysinfo::sysinfo;
use nix::sys::utsname::uname;

fn main() {
    // bb();
    // greet_world();
    // first();
    // aa();
    // cc();

    let myos = uname();

    match myos {
        Ok(info) => {
            let release = info.release().to_string_lossy();

            let sysname = info.sysname().to_string_lossy();

            if sysname == "Linux" {
                println!("确认是在 Linux 内核上运行。");
                println!("你的 Linux 内核版本是: {}", release);
            }

            // cow type
            match &*sysname {
                "Linux" => println!("你很有眼光！"),
                "Windows" => println!("你认真的吗？"),
                _ => println!("未知的领域。"),
            }

            // match guard
            match release {
                // n 是把 release 绑定到一个临时变量名
                n if n.contains("arch") => {
                    println!("检测到内核构建信息包含 Arch 字样，硬核！");
                }
                n if n.contains("generic") => {
                    println!("这似乎是一个通用内核。");
                }
                _ => println!("这是一个神秘的自定义内核。"),
            }
        }
        Err(e) => eprintln!("获取系统信息失败: {}", e),
    }

    match sysinfo() {
        Ok(info) => {
            let load = info.load_average();
            println!("1分钟平均负载: {:.2}", load.0);
            println!("总内存: {} MB", info.ram_total() / 1024 / 1024);
        }
        Err(e) => eprintln!("获取系统信息失败: {}", e),
    }
}

#[allow(dead_code)]
fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }
}

use std::io;

// input
#[allow(dead_code)]
fn aa() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();
    // 读取控制台的输出
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

//循环
#[allow(dead_code)]
fn bb() {
    loop {
        println!("again!");
    }
}

//break带返回值
#[allow(dead_code)]
fn cc() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

//文字处理
#[allow(dead_code)]
fn first() {
    let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }
}

// cargo run
// cargo build
// ./target/debug/world_hello
// Hello, world!

// cargo run --release
// cargo build --release
// cargo check
