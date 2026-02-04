fn main() {

}

// fn move() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("s1: {}", s1);
// }

fn borrow() {
    let s1 = String::from("Pacman");
    let r1 = &s1; // 借给 r1 看
    let r2 = &s1; // 借给 r2 看
    println!("{}, {}", r1, r2);
}

// fn mutborrow() {
//     let mut s1 = String::from("Pacman");
//     let r1 = &mut s1; // 借给 r1 看
//     let r2 = &mut s1; // 借给 r2 看
//     println!("{}, {}", r1, r2);
// }

fn write() {
    let mut s = String::from("Hello");
    let r1 = &s; // 不可变借用
    let r2 = &mut s;
}

// fn pointer() {
//     let r;                // ---------+-- r 的生命周期开始
//     {                     //          |
//         let x = 5;        // -+-- x 的生命周期开始
//         r = &x;           //  | 试图让 r 指向 x
//     }                     // -+-- x 在这里挂了 (Drop)！内存被释放
//                           //          |
//     println!("r: {}", r); //          | ⚠️ 错误！r 还在引用 x，但 x 已经不在了！
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

