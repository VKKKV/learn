// struct Point {
//     x: f64,
//     y: f64,
// }
//
// // `Point` 的关联函数都放在下面的 `impl` 语句块中
// impl Point {
//     // 关联函数的使用方法跟构造器非常类似
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }
//
//     // 另外一个关联函数，有两个参数
//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y }
//     }
// }
//
// struct Rectangle {
//     p1: Point,
//     p2: Point,
// }
//
// impl Rectangle {
//     // 这是一个方法
//     // `&self` 是 `self: &Self` 的语法糖
//     // `Self` 是当前调用对象的类型，对于本例来说 `Self` = `Rectangle`
//     fn area(&self) -> f64 {
//         // 使用点操作符可以访问 `self` 中的结构体字段
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;
//
//
//         // `abs` 是一个 `f64` 类型的方法，会返回调用者的绝对值
//         ((x1 - x2) * (y1 - y2)).abs()
//     }
//
//     fn perimeter(&self) -> f64 {
//         let Point { x: x1, y: y1 } = self.p1;
//         let Point { x: x2, y: y2 } = self.p2;
//
//         2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
//     }
//
//     // 该方法要求调用者是可变的，`&mut self` 是 `self: &mut Self` 的语法糖
//     fn translate(&mut self, x: f64, y: f64) {
//         self.p1.x += x;
//         self.p2.x += x;
//
//         self.p1.y += y;
//         self.p2.y += y;
//     }
// }
//
// // `Pair` 持有两个分配在堆上的整数
// struct Pair(Box<i32>, Box<i32>);
//
// impl Pair {
//     // 该方法会拿走调用者的所有权
//     // `self` 是 `self: Self` 的语法糖
//     fn destroy(self) {
//         let Pair(first, second) = self;
//
//         println!("Destroying Pair({}, {})", first, second);
//
//         // `first` 和 `second` 在这里超出作用域并被释放
//     }
// }
//
// fn main() {
//     let rectangle = Rectangle {
//         // 关联函数的调用不是通过点操作符，而是使用 `::`
//         p1: Point::origin(),
//         p2: Point::new(3.0, 4.0),
//     };
//
//     // 方法才是通过点操作符调用
//     // 注意，这里的方法需要的是 `&self` 但是我们并没有使用 `(&rectangle).perimeter()` 来调用，原因在于：
//     // 编译器会帮我们自动取引用
//     //  `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
//     println!("Rectangle perimeter: {}", rectangle.perimeter());
//     println!("Rectangle area: {}", rectangle.area());
//
//     let mut square = Rectangle {
//         p1: Point::origin(),
//         p2: Point::new(1.0, 1.0),
//     };
//
//
//     // 错误！`rectangle` 是不可变的，但是这个方法要求一个可变的对象
//     //rectangle.translate(1.0, 0.0);
//     // TODO ^ 试着反注释此行，看看会发生什么
//
//     // 可以！可变对象可以调用可变的方法
//     square.translate(1.0, 1.0);
//
//     let pair = Pair(Box::new(1), Box::new(2));
//
//     pair.destroy();
//
//     // Error! 上一个 `destroy` 调用拿走了 `pair` 的所有权
//     //pair.destroy();
//     // TODO ^ 试着反注释此行
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     // 完成 area 方法，返回矩形 Rectangle 的面积
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     assert_eq!(rect1.area(), 1500);
// }

// 只填空，不要删除任何代码行!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     pub fn show_state(&self) {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: "red".to_owned(),
//     };
//     // 不要拿走 `light` 的所有权
//     light.show_state();
//     // 否则下面代码会报错
//     println!("{:?}", light);
// }

// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 使用 `Self` 填空
//     pub fn show_state(self: &Self) {
//         println!("the current state is {}", self.color);
//     }
//
//     // 填空，不要使用 `Self` 或其变体
//     // pub fn change_state(&mut self) {
//     //     self.color = "green".to_string()
//     // }
//     // pub fn change_state(self: &mut TrafficLight) {
//     //     self.color = "green".to_string()
//     // }
// }
// fn main() {}

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }
//
// impl TrafficLight {
//     // 1. 实现下面的关联函数 `new`,
//     // 2. 该函数返回一个 TrafficLight 实例，包含 `color` "red"
//     // 3. 该函数必须使用 `Self` 作为类型，不能在签名或者函数体中使用 `TrafficLight`
//     pub fn new() -> Self {
//         Self { color: "red".to_string() }
//     }
//
//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }
//
// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// // 使用多个 `impl` 语句块重写下面的代码
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// fn main() {}

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }
//
// // 为 TrafficLightColor 实现所需的方法
// impl TrafficLightColor {
//     fn color(&self) -> &str {
//         match self {
//             TrafficLightColor::Red => "red",
//             TrafficLightColor::Yellow => "yellow",
//             TrafficLightColor::Green => "green",
//         }
//     }
// }
//
// fn main() {
//     let c = TrafficLightColor::Yellow;
//
//     assert_eq!(c.color(), "yellow");
//
//     println!("{:?}",c);
// }

//泛型

// 填空
// struct A;          // 具体的类型 `A`.
// struct S(A);       // 具体的类型 `S`.
// struct SGen<T>(T); // 泛型 `SGen`.
//
// fn reg_fn(_s: S) {}
//
// fn gen_spec_t(_s: SGen<A>) {}
//
// fn gen_spec_i32(_s: SGen<i32>) {}
//
// fn generic<T>(_s: SGen<T>) {}
//
// fn main() {
//     // 使用非泛型函数
//     reg_fn(S(A));          // 具体的类型
//     gen_spec_t(SGen(A));   // 隐式地指定类型参数  `A`.
//     gen_spec_i32(SGen(1)); // 隐式地指定类型参数`i32`.
//
//     // 显式地指定类型参数 `char`
//     generic::<char>(SGen('a'));
//
//     // 隐式地指定类型参数 `char`.
//     generic(SGen('a'));
// }

// 实现下面的泛型函数 sum
// use std::ops::Add;
// fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
//
// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));
// }

// 实现一个结构体 Point 让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// 修改以下结构体让代码工作
// struct Point<T,U> {
//     x: T,
//     y: U,
// }
// struct Point<T> {
//     x: T,
//     y: String,
// }
//
// fn main() {
//     // 不要修改这行代码！
//     let p = Point{x: 5, y : "hello".to_string()};
// }

// 为 Val 增加泛型参数，不要修改 `main` 中的代码
// struct Val<T> {
//     val: T,
// }
//
// impl<T> Val<T> {
//     fn value(&self) -> &T {
//         &self.val
//     }
// }
//
// // impl Val<f64> {
// //     fn value(&self) -> &f64 {
// //         &self.val
// //     }
// // }
// //
// // impl Val<String> {
// //     fn value(&self) -> &str {
// //         &self.val
// //     }
// // }
//
// fn main() {
//     let x = Val { val: 3.0 };
//     let y = Val {
//         val: "hello".to_string(),
//     };
//     println!("{}, {}", x.value(), y.value());
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }
//
// impl<T, U> Point<T, U> {
//     // 实现 mixup，不要修改其它代码！
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};
//
//     let p3 = p1.mixup(p2);
//
//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');
// }

// 修复错误，让代码工作
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
//
// fn main() {
//     let p = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin())
// }

//Const 泛型

// struct ArrayPair<T, const N: usize> {
//     left: [T; N],
//     right: [T; N],
// }
//
// impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
//     // ...
// }

//
// 一个单独的 const 泛型参数
// 一个字面量 (i.e. 整数, 布尔值或字符).
// 一个具体的 const 表达式( 表达式中不能包含任何 泛型参数)
//
// fn foo<const N: usize>() {}
//
// fn bar<T, const M: usize>() {
//     foo::<M>(); // ok: 符合第一种
//     foo::<2021>(); // ok: 符合第二种
//     foo::<{ 20 * 100 + 20 * 10 + 1 }>(); // ok: 符合第三种
//
//     foo::<{ M + 1 }>(); // error: 违背第三种，const 表达式中不能有泛型参数 M
//     foo::<{ std::mem::size_of::<T>() }>(); // error: 泛型表达式包含了泛型参数 T
//
//     let _: [u8; M]; // ok: 符合第一种
//     let _: [u8; std::mem::size_of::<T>()]; // error: 泛型表达式包含了泛型参数 T
// }
//
// fn main() {}

// pub struct MinSlice<T, const N: usize> {
//     pub head: [T; N],
//     pub tail: [T],
// }
//
// fn main() {
//     let slice: &[u8] = b"Hello, world";
//     let reference: Option<&u8> = slice.get(6);
//     // 我们知道 `.get` 返回的是 `Some(b' ')`
//     // 但编译器不知道
//     assert!(reference.is_some());
//
//     let slice: &[u8] = b"Hello, world";
//
//     // 当编译构建 MinSlice 时会进行长度检查，也就是在编译期我们就知道它的长度是 12
//     // 在运行期，一旦 `unwrap` 成功，在 `MinSlice` 的作用域内，就再无需任何检查
//     let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
//     let value: u8 = minslice.head[6];
//     assert_eq!(value, b' ')
// }

// 修复错误
// struct Array<T, const N: usize> {
//     data: [T; N],
// }
//
// fn main() {
//     let arrays = (
//         Array { data: [1, 2, 3] },
//         Array {
//             data: [1.0, 2.0, 3.0],
//         },
//         Array { data: [1, 2] },
//     );
// }

// use std::fmt::Debug;

// // 填空
// fn print_array<T: Debug, const N: usize>(arr: [T; N]) {
//     println!("{:?}", arr);
// }
//
// fn main() {
//     let arr = [1, 2, 3];
//     print_array(arr);
//
//     let arr = ["hello", "world"];
//     print_array(arr);
// }

// #![allow(incomplete_features)]
// // rustup toolchain install nightly
// // rustup override set nightly
// #![feature(generic_const_exprs)]
//
// fn check_size<T>(_val: T)
// where
//     Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
// {
//     //...
// }
//
// // fix the errors in main
// fn main() {
//     check_size([0u8; 767]);
//     check_size([0i32; 191]);
//
//     // check_size(["hello你好"; 1]); // &str is a string reference, containing a pointer and string length in it, so it takes two word long, in x86-64, 1 word = 8 bytes
//     // check_size([(); 1].map(|_| "hello你好".to_string())); // String is a smart pointer struct, it has three fields: pointer, length and capacity, each takes 8 bytes
//     // check_size(['中'; 1]); // A char takes 4 bytes in Rust
//     //
//     // &str 是 16 字节 (ptr + len)
//     // 16 * 47 = 752 < 768
//     check_size(["hello你好"; 47]);
//
//     // String 是 24 字节 (ptr + cap + len)
//     // 24 * 31 = 744 < 768
//     check_size([(); 31].map(|_| "hello你好".to_string()));
//
//     // char 是 4 字节
//     // 4 * 191 = 764 < 768
//     check_size(['中'; 191]);
// }
//
// pub enum Assert<const CHECK: bool> {}
//
// pub trait IsTrue {}
//
// impl IsTrue for Assert<true> {}

//Traits

// struct Sheep { naked: bool, name: String }
//
// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }
//
//     fn shear(&mut self) {
//         if self.is_naked() {
//             // `Sheep` 结构体上定义的方法可以调用 `Sheep` 所实现的特征的方法
//             println!("{} is already naked...", self.name());
//         } else {
//             println!("{} gets a haircut!", self.name);
//
//             self.naked = true;
//         }
//     }
// }
//
//
// trait Animal {
//     // 关联函数签名；`Self` 指代实现者的类型
//     // 例如我们在为 Pig 类型实现特征时，那 `new` 函数就会返回一个 `Pig` 类型的实例，这里的 `Self` 指代的就是 `Pig` 类型
//     fn new(name: String) -> Self;
//
//     // 方法签名
//     fn name(&self) -> String;
//
//     fn noise(&self) -> String;
//
//     // 方法还能提供默认的定义实现
//     fn talk(&self) {
//         println!("{} says {}", self.name(), self.noise());
//     }
// }
//
// impl Animal for Sheep {
//     // `Self` 被替换成具体的实现者类型： `Sheep`
//     fn new(name: String) -> Sheep {
//         Sheep { name: name, naked: false }
//     }
//
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//
//     fn noise(&self) -> String {
//         if self.is_naked() {
//             "baaaaah?".to_string()
//         } else {
//             "baaaaah!".to_string()
//         }
//     }
//
//     // 默认的特征方法可以被重写
//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }
//
// fn main() {
//     // 这里的类型注释时必须的
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());
//     // TODO ^ 尝试去除类型注释，看看会发生什么
//
//     dolly.talk();
//     dolly.shear();
//     dolly.talk();
// }

// 完成两个 `impl` 语句块
// 不要修改 `main` 中的代码
// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//
//     fn say_something(&self) -> String;
// }
//
// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }
//
//     fn say_something(&self) -> String {
//         "I'm a good student".to_string()
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I'm your new teacher")
//     }
//
//     fn say_something(&self) -> String {
//         "I'm not a bad teacher".to_string()
//     }
// }
//
// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I'm a good student");
//
//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");
//
//     println!("Success!")
// }

// `Centimeters`, 一个元组结构体，可以被比较大小
// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);
//
// // `Inches`, 一个元组结构体可以被打印
// #[derive(Debug)]
// struct Inches(i32);
//
// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;
//
//         Centimeters(inches as f64 * 2.54)
//     }
// }
//
// // 添加一些属性让代码工作
// // 不要修改其它代码！
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);
//
// fn main() {
//     let _one_second = Seconds(1);
//
//     println!("One second looks like: {:?}", _one_second);
//     let _this_is_true = _one_second == _one_second;
//     let _this_is_false = _one_second > _one_second;
//
//     let foot = Inches(12);
//
//     println!("One foot equals {:?}", foot);
//
//     let meter = Centimeters(100.0);
//
//     let cmp =
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };
//
//     println!("One foot is {} than one meter.", cmp);
// }

// use std::ops;
//
// // 实现 fn multiply 方法
// // 如上所述，`+` 需要 `T` 类型实现 `std::ops::Add` 特征
// // 那么, `*` 运算符需要实现什么特征呢? 你可以在这里找到答案: https://doc.rust-lang.org/core/ops/
// fn multiply<T: ops::Mul<Output = T>>(a: T, b: T) -> T {
//     a * b
// }
//
// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));
//
//     println!("Success!")
// }

// 修复错误，不要修改 `main` 中的代码!
// use std::ops;
//
// struct Foo;
// struct Bar;
//
// #[derive(Debug, PartialEq)]
// struct FooBar;
//
// #[derive(Debug, PartialEq)]
// struct BarFoo;
//
// // 下面的代码实现了自定义类型的相加： Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
//
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }
//
// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;
//
//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }
//
// fn main() {
//     // 不要修改下面代码
//     // 你需要为 FooBar 派生一些特征来让代码工作
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);
//
//     println!("Success!")
// }

// 实现 `fn summary`
// 修复错误且不要移除任何代码行
// trait Summary {
//     fn summarize(&self) -> String;
// }
//
// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }
//
// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }
//
// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }
//
// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }
//
// fn main() {
//     let post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };
//
//     summary(&post);
//     summary(&weibo);
//
//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }
//
// // 在下面实现 `fn summary` 函数
// // fn summary<T: Summary>(item: &T) {
// //     println!("{}", item.summarize());
// // }
//
// fn summary(t: &impl Summary) {
//     println!("{}", t.summarize());
// }

// struct Sheep {}
// struct Cow {}
//
// trait Animal {
//     fn noise(&self) -> String;
// }
//
// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }
//
// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }
//
// // 返回一个类型，该类型实现了 Animal 特征，但是我们并不能在编译期获知具体返回了哪个类型
// // 修复这里的错误，你可以使用虚假的随机，也可以使用特征对象
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }
//
// fn main() {
//     let random_number = 0.234;
//     let animal = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }

// fn main() {
//     assert_eq!(sum(1, 2), 3);
// }
//
// // 通过两种方法使用特征约束来实现 `fn sum`
// // fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
// fn sum<T>(x: T, y: T) -> T
// where T: std::ops::Add<Output = T>
// {
//     x + y
// }

// 修复代码中的错误
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }
//
// impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
//
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);
//
// fn main() {
//     let pair = Pair{
//         x: Unit(1),
//         y: Unit(3)
//     };
//
//     pair.cmp_display();
// }

// 填空
// fn example1() {
//     // `T: Trait` 是最常使用的方式
//     // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
//     struct Cacher<T: Fn(u32) -> u32> {
//         calculation: T,
//         value: Option<u32>,
//     }
//
//     impl<T: Fn(u32) -> u32> Cacher<T> {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 // much more concise
//                 // Some(v) if arg == v => v,
//                 // _ => {
//                 //     let v = (self.calculation)(arg);
//                 //     self.value = Some(v);
//                 //     v
//                 // },
//             }
//         }
//     }
//
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(10), 11);
//     assert_eq!(cacher.value(15), 16);
//     println!("cacher: {:#?}", cacher.value(15));
// }
//
//
// fn example2() {
//     // 还可以使用 `where` 来约束 T
//     struct Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         calculation: T,
//         value: Option<u32>,
//     }
//
//     impl<T> Cacher<T>
//         where T: Fn(u32) -> u32,
//     {
//         fn new(calculation: T) -> Cacher<T> {
//             Cacher {
//                 calculation,
//                 value: None,
//             }
//         }
//
//         fn value(&mut self, arg: u32) -> u32 {
//             match self.value {
//                 Some(v) => v,
//                 None => {
//                     let v = (self.calculation)(arg);
//                     self.value = Some(v);
//                     v
//                 },
//             }
//         }
//     }
//
//     let mut cacher = Cacher::new(|x| x+1);
//     assert_eq!(cacher.value(20), 21);
//     assert_eq!(cacher.value(25), 21);
// }
//
//
//
// fn main() {
//     example1();
//     example2();
//
//     println!("Success!")
// }

//特征对象

// trait Bird {
//     fn quack(&self) -> String;
// }
//
// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }
//
// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }
//
// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }
//
// fn main() {
//     // 填空
//     let duck = Duck;
//     duck.swim();
//
//     let bird = hatch_a_bird(2);
//     // 变成鸟儿后，它忘记了如何游，因此以下代码会报错
//     // bird.swim();
//     // 但它依然可以叫唤
//     assert_eq!(bird.quack(), "duck duck");
//
//     let bird = hatch_a_bird(1);
//     // 这只鸟儿忘了如何飞翔，因此以下代码会报错
//     // bird.fly();
//     // 但它也可以叫唤
//     assert_eq!(bird.quack(), "swan swan");
//
//     println!("Success!")
// }
//
// // 实现以下函数
// fn hatch_a_bird(kind: i32) -> Box<dyn Bird> {
//     if kind == 1 {
//         Box::new(Swan)
//     } else {
//         Box::new(Duck)
//     }
// }

// trait Bird {
//     fn quack(&self);
// }
//
// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }
//
// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }
//
// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }
//
// fn main() {
//     // 填空
//     let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];
// let birds = vec![Box::new(Duck) as Box<dyn Bird>, Box::new(Swan)];
//
//     for bird in birds {
//         bird.quack();
//         // 当 duck 和 swan 变成 bird 后，它们都忘了如何翱翔于天际，只记得该怎么叫唤了。。
//         // 因此，以下代码会报错
//         // bird.fly();
//     }
// }

// 填空
// trait Draw {
//     fn draw(&self) -> String;
// }
//
// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }
//
// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }
//
// fn main() {
//     let x = 1.1f64;
//     let y = 8u8;
//
//     // draw x
//     draw_with_box(Box::new(x));
//
//     // draw y
//     draw_with_ref(&y);
//
//     println!("Success!")
// }
//
// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }
//
// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }

// trait Foo {
//     fn method(&self) -> String;
// }
//
// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }
//
// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }
//
// // 通过泛型实现以下函数
// // fn static_dispatch<T: Foo>(x: T) {
// fn static_dispatch(x: impl Foo) {
//     x.method();
// }
//
// // 通过特征对象实现以下函数
// fn dynamic_dispatch(x: &dyn Foo) {
//     x.method();
// }
//
// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();
//
//     static_dispatch(x);
//     static_dispatch(y.clone());
//     dynamic_dispatch(&x);
//     dynamic_dispatch(&y);
//
//     println!("Success!")
// }

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
// trait MyTrait {
//     fn f(&self) -> Self;
// }
//
// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }
//
// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }
//
// // fn my_function(x: Box<dyn MyTrait>)  {
// // fn my_function<T: MyTrait>(x: Box<T>)  {
// fn my_function(x: Box<impl MyTrait>)  {
//     x.f();
// }
//
// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));
//
//     println!("Success!")
// }

// struct Container(i32, i32);
//
// // 使用关联类型实现重新实现以下特征
// // trait Contains {
// //    type A;
// //    type B;
//
// trait Contains {
//     type A;
//     type B;
//
//     fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }
//
//
// impl Contains for Container {
//     type A = i32;
//     type B = i32;
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2)
//     }
//     // Grab the first number.
//     fn first(&self) -> i32 { self.0 }
//
//     // Grab the last number.
//     fn last(&self) -> i32 { self.1 }
// }
//
// fn difference<C: Contains>(container: &C) -> i32 {
//     container.last() - container.first()
// }
//
// fn main() {
//     let number_1 = 3;
//     let number_2 = 10;
//
//     let container = Container(number_1, number_2);
//
//     println!("Does container contain {} and {}: {}",
//         &number_1, &number_2,
//         container.contains(&number_1, &number_2));
//     println!("First number: {}", container.first());
//     println!("Last number: {}", container.last());
//
//     println!("The difference is: {}", difference(&container));
// }

// use std::ops::Sub;
//
// #[derive(Debug, PartialEq)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// // 用三种方法填空: 其中两种使用默认的泛型参数，另外一种不使用
// // 填空：
// // impl<T: Sub<Output = T>> Sub for Point<T>{
// // impl<T: Sub<Output = T>> Sub<Self> for Point<T>{
// impl<T: Sub<Output = T>> Sub<Point<T>> for Point<T>{
//     type Output = Self;
//
//     fn sub(self, other: Self) -> Self::Output {
//         Point {
//             x: self.x - other.x,
//             y: self.y - other.y,
//         }
//     }
// }
//
// fn main() {
//     assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
//         Point { x: 1, y: 3 });
//
//     println!("Success!")
// }

// trait UsernameWidget {
//     fn get(&self) -> String;
// }
//
// trait AgeWidget {
//     fn get(&self) -> u8;
// }
//
// struct Form {
//     username: String,
//     age: u8,
// }
//
// impl UsernameWidget for Form {
//     fn get(&self) -> String {
//         self.username.clone()
//     }
// }
//
// impl AgeWidget for Form {
//     fn get(&self) -> u8 {
//         self.age
//     }
// }
//
// fn main() {
//     let form = Form{
//         username: "rustacean".to_owned(),
//         age: 28,
//     };
//
//     // 如果你反注释下面一行代码，将看到一个错误: Fully Qualified Syntax
//     // 毕竟，这里有好几个同名的 `get` 方法
//     //
//     // println!("{}", form.get());
//
//     let username = UsernameWidget::get(&form);
//     assert_eq!("rustacean".to_owned(), username);
//     let age = AgeWidget::get(&form); // 你还可以使用以下语法 `<Form as AgeWidget>::get`
//     assert_eq!(28, age);
//
//     println!("Success!")
// }

// trait Pilot {
//     fn fly(&self) -> String;
// }
//
// trait Wizard {
//     fn fly(&self) -> String;
// }
//
// struct Human;
//
// impl Pilot for Human {
//     fn fly(&self) -> String {
//         String::from("This is your captain speaking.")
//     }
// }
//
// impl Wizard for Human {
//     fn fly(&self) -> String {
//         String::from("Up!")
//     }
// }
//
// impl Human {
//     fn fly(&self) -> String {
//         String::from("*waving arms furiously*")
//     }
// }
//
// fn main() {
//     let person = Human;
//
//     assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
//     assert_eq!(Wizard::fly(&person), "Up!");
//
//     assert_eq!(person.fly(), "*waving arms furiously*");
//
//     println!("Success!")
// }

// trait Person {
//     fn name(&self) -> String;
// }
//
// // Person 是 Student 的 supertrait .
// // 实现 Student 需要同时实现 Person.
// trait Student: Person {
//     fn university(&self) -> String;
// }
//
// trait Programmer {
//     fn fav_language(&self) -> String;
// }
//
// // CompSciStudent (computer science student) 是 Programmer
// // 和 Student 的 subtrait. 实现 CompSciStudent 需要先实现这两个 supertraits.
// trait CompSciStudent: Programmer + Student {
//     fn git_username(&self) -> String;
// }
//
// fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
//     format!(
//         "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
//         student.name(),
//         student.university(),
//         student.fav_language(),
//         student.git_username()
//     )
// }
//
// struct CSStudent {
//     name: String,
//     university: String,
//     fav_language: String,
//     git_username: String
// }
//
// // 为 CSStudent 实现所需的特征
// // 1. 实现最底层的 Person
// impl Person for CSStudent {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
// }
//
// // 2. 实现 Student (依赖 Person)
// impl Student for CSStudent {
//     fn university(&self) -> String {
//         self.university.clone()
//     }
// }
//
// // 3. 实现 Programmer (独立依赖)
// impl Programmer for CSStudent {
//     fn fav_language(&self) -> String {
//         self.fav_language.clone()
//     }
// }
//
// // 4. 最后实现 CompSciStudent (依赖 Student + Programmer)
// // 只有上面三个都实现了，编译器才允许你实现这个
// impl CompSciStudent for CSStudent {
//     fn git_username(&self) -> String {
//         self.git_username.clone()
//     }
// }
//
// fn main() {
//     let student = CSStudent {
//         name: "Sunfei".to_string(),
//         university: "XXX".to_string(),
//         fav_language: "Rust".to_string(),
//         git_username: "sunface".to_string()
//     };
//
//     // 填空
//     println!("{}", comp_sci_student_greeting(&student));
// }

// use std::fmt;
//
// // 定义一个 newtype `Pretty`
//
// struct Pretty(String);
//
// impl fmt::Display for Pretty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\"{}\"", self.0.clone() + ", world")
//     }
// }
//
// fn main() {
//     let w = Pretty("hello".to_string());
//     println!("w = {}", w);
// }

// 集合类型

// 填空并修复错误
// 1. 不要使用 `to_string()`
// 2. 不要添加/删除任何代码行
// fn main() {
//     let mut s: String = String::from("hello, ");
//     s.push_str("world");
//     s.push('!');
//
//     move_ownership(&s);
//     assert_eq!(s, "hello, world!");
//
//     println!("Success!")
// }
//
// fn move_ownership(s: &str) {
//     println!("ownership of \"{}\" is moved here!", s)
// }

// 填空
// fn main() {
//     let mut s = String::from("hello, world");
//
//     // let slice1: &str = &s; // 使用两种方法
//     let slice1: &str = s.as_str(); // 使用两种方法
//     let slice1: &str = s[..]; // 使用两种方法
//     assert_eq!(slice1, "hello, world");
//
//     let slice2 = &s[0..5];
//     assert_eq!(slice2, "hello");
//
//     let slice3: &mut String = &mut s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
//
//     println!("Success!")
// }

// 问题:  我们的代码中发生了多少次堆内存分配？
// 你的回答:
// fn main() {
//     // 基于 `&str` 类型创建一个 String,
//     // 字符串字面量的类型是 `&str`
//     let s: String = String::from("hello, world!");
//
//     // 创建一个切片引用指向 String `s`
//     let slice: &str = &s;
//
//     // 基于刚创建的切片来创建一个 String
//     let s: String = slice.to_string();
//
//     assert_eq!(s, "hello, world!");
//
//     println!("Success!")
// }

// 填空并修复错误
// fn main() {
//     let s = String::from("hello, 世界");
//     let slice1 = &s[0..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
//     assert_eq!(slice1, "h");
//
//     let slice2 = &s[7..=9]; // 提示: `世` 在 UTF-8 编码中占用 3 个字节
//     assert_eq!(slice2, "世");
//
//     // 迭代 s 中的所有字符
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }
//
//     println!("Success!")
// }

// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";
//
//     let rocket = utf8_slice::slice(s, 4, 5);
//     // Will equal "🚀"
//     assert_eq!(rocket, "🚀");
//     println!("Success!")
// }

// 填空
// fn main() {
//     let mut s = String::new();
//     s.push_str("hello");
//
//     let v = vec![104, 101, 108, 108, 111];
//
//     // 将字节数组转换成 String
//     let s1 = String::from_utf8(v).unwrap();
//
//     assert_eq!(s, s1);
//
//     println!("Success!")
// }

// 修改下面的代码以打印如下内容:
// 25
// 25
// 25
// 循环中不会发生任何内存分配
// fn main() {
//     //String::new() 初始容量通常为 0
//     // let mut s = String::new();
//     //Memory Pre-allocation
//     let mut s = String::with_capacity(25);
//
//     println!("{}", s.capacity());
//
//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }
//
//     println!("Success!")
// }

// 填空
// use std::mem;
//
// fn main() {
//     let story = String::from("Rust By Practice");
//
//     // 阻止 String 的数据被自动 drop
//     let mut story = mem::ManuallyDrop::new(story);
//
//     let ptr = story.as_mut_ptr();
//     let len = story.len();
//     let capacity = story.capacity();
//
//     assert_eq!(16, len);
//
//     // 我们可以基于 ptr 指针、长度和容量来重新构建 String.
//     // 这种操作必须标记为 unsafe，因为我们需要自己来确保这里的操作是安全的
//     let s = unsafe { String::from_raw_parts(ptr, len, capacity) };
//
//     assert_eq!(*story, s);
//
//     println!("Success!")
// }

// Vector

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
//
//     let v = Vec::from(arr);
//     is_vec(v);
//
//     let v = vec![1, 2, 3];
//     is_vec(v);
//
//     // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此... wtf?
//     let v = vec![1, 2, 3];
//     is_vec(v.clone());
//
//     // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
//     // 使用 Vec::new 和 `for` 来重写下面这段代码
//     // let v1 = vec![arr];
//     let mut v1 = Vec::new();
//     for i in arr {
//        v1.push(i);
//     }
//
//     is_vec(v1.clone());
//
//     assert_eq!(v, v1);
//
//     println!("Success!")
// }
//
// fn is_vec(v: Vec<u8>) {}

// 填空
// fn main() {
//     let mut v1 = Vec::from([1, 2, 4]);
//     v1.pop();
//     v1.push(3);
//
//     let mut v2 = Vec::new();
//     v2.extend_from_slice(&v1);
//
//     assert_eq!(v1, v2);
//
//     println!("Success!")
// }

// 填空
// fn main() {
//     // array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
//
//     assert_eq!(v1, v2);
//
//     // String -> Vec
//     // impl From<String> for Vec
//     let s = "hello".to_string();
//     let v1: Vec<u8> = s.into_bytes();
//
//     let s = "hello".to_string();
//     let v2 = s.into_bytes();
//     assert_eq!(v1, v2);
//
//     // impl<'_> From<&'_ str> for Vec
//     let s = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);
//
//     // 迭代器 Iterators 可以通过 collect 变成 Vec
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);
//
//     println!("Success!")
//  }

// 修复错误并实现缺失的代码
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..v.len() {
//         println!("{:?}", v[i])
//     }
//
//     for i in 0..5 {
//         match v.get_mut(i) {
//             Some(val) => *val += 1,
//             None => v.push((i + 2) as i32),
//         }
//     }
//
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);
//
//     println!("Success!")
// }

// 修复错误
// fn main() {
//     let mut v = vec![1, 2, 3];
//
//     let slice1 = &v[..];
//     // 越界访问将导致 panic.
//     // 修改时必须使用 `v.len`
//     let slice2 = &v[0..v.len()];
//
//     assert_eq!(slice1, slice2);
//
//     // 切片是只读的
//     // 注意：切片和 `&Vec` 是不同的类型，后者仅仅是 `Vec` 的引用，并可以通过解引用直接获取 `Vec`
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3 = &mut v[0..4];
//
//     assert_eq!(slice3, &[1, 2, 3, 4]);
//
//     println!("Success!")
// }

// 修复错误
// fn main() {
//     let mut vec = Vec::with_capacity(10);
//
//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);
//
//     // 由于提前设置了足够的容量，这里的循环不会造成任何内存分配...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);
//
//     // ...但是下面的代码会造成新的内存分配
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert_eq!(vec.capacity(), 20);
//
//     // 填写一个合适的值，在 `for` 循环运行的过程中，不会造成任何内存分配
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }
//
//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
//
//     println!("Success!")
// }

// fn main() {
//    let v = vec![1, 2.0, 3];
// }

// #[derive(Debug, PartialEq, Clone)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // 填空
//     // let v: Vec<IpAddr> = [IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())].to_vec();
//     let v: Vec<IpAddr> = vec![IpAddr::V4("127.0.0.1".to_string()), IpAddr::V6("::1".to_string())];
// Option 1 ([...].to_vec()) 的过程：
//
//     栈分配：先在栈 (Stack) 上创建一个包含两个 IpAddr 的临时数组。
//
//     堆分配：调用 .to_vec()，在堆 (Heap) 上申请内存。
//
//     深拷贝：遍历栈上的数组，对每个 IpAddr 调用 .clone()（这涉及复制内部的 String 堆内存），把克隆体放入 Vector。
//
//     清理：销毁栈上的临时数组（调用析构函数）。
//
//     代价：双倍内存消耗 + 深拷贝开销。这是绝对的 Bloat。
//
// Option 2 (vec![...]) 的过程：
//
//     堆分配：直接在堆上申请正好能装下两个 IpAddr 的内存。
//
//     移动写入：把生成的 IpAddr 直接写入（Move）这块内存。
//
//     代价：零拷贝，一步到位。这才是 Arch Way。
//     // 枚举的比较需要派生 PartialEq 特征
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));
//
//     println!("Success!")
// }

// trait IpAddr {
//     fn display(&self);
// }
//
// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }
//
// fn main() {
//     // 填空
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];
//
//     for ip in v {
//         ip.display();
//     }
// }

//HashMap

// 填空并修复错误
// use std::collections::HashMap;
// fn main() {
//     let mut scores = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);
//
//     // get 返回一个 Option<&V> 枚举值
//     let score = scores.get("Sunface");
//     assert_eq!(score, Some(&98));
//
//     if scores.contains_key("Daniel") {
//         // 索引返回一个值 V
//         let score = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }
//
//     assert_eq!(scores.len(), 3);
//
//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score)
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let teams = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];
//
//     let mut teams_map1 = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }
//
//     // 使用两种方法实现 team_map2
//     // 提示:其中一种方法是使用 `collect` 方法
//     // let teams_map2 = teams.into_iter().collect();
//     // let teams_map2 = HashMap::from(teams);
//     // let mut teams_map2 = HashMap::new();
//     // for team in &teams {
//     //     teams_map2.insert(team.0, team.1);
//     // }
//
//     assert_eq!(teams_map1, teams_map2);
//
//     println!("Success!")
// }

// 填空
// use std::collections::HashMap;
// fn main() {
//     // 编译器可以根据后续的使用情况帮我自动推断出 HashMap 的类型，当然你也可以显式地标注类型：HashMap<&str, u8>
//     let mut player_stats = HashMap::new();
//
//     // 查询指定的 key, 若不存在时，则插入新的 kv 值
//     player_stats.entry("health").or_insert(100);
//
//     assert_eq!(player_stats["health"], 100);
//
//     // 通过函数来返回新的值
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);
//
//     let health = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);
//
//     println!("Success!")
// }
//
// fn random_stat_buff() -> u8 {
//     // 为了简单，我们没有使用随机，而是返回一个固定的值
//     42
// }

// 修复错误
// 提示: `derive` 是实现一些常用特征的好办法
// use std::collections::HashMap;
//
// #[derive(Hash, Eq, PartialEq, Debug)]
// struct Viking {
//     name: String,
//     country: String,
// }
//
// impl Viking {
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }
//
// fn main() {
//     // 使用 HashMap 来存储 viking 的生命值
//     let vikings = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);
//
//     // 使用 derive 的方式来打印 viking 的当前状态
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
//     map.insert(1, 2);
//     map.insert(3, 4);
//     // 事实上，虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
//     assert!(map.capacity() >= 100);
//
//     // 对容量进行收缩，你提供的值仅仅是一个允许的最小值，实际上，Rust 会根据当前存储的数据量进行自动设置，当然，这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间
//
//     map.shrink_to(50);
//     assert!(map.capacity() >= 50);
//
//     // 让 Rust  自行调整到一个合适的值，剩余策略同上
//     map.shrink_to_fit();
//     assert!(map.capacity() >= 2);
//     println!("Success!")
// }

// 修复错误，尽可能少的去修改代码
// 不要移除任何代码行！
// use std::collections::HashMap;
// fn main() {
//   let v1 = 10;
//   let mut m1 = HashMap::new();
//   m1.insert(v1, v1);
//   println!("v1 is still usable after inserting to hashmap : {}", v1);
//
//   let v2 = "hello".to_string();
//   let mut m2 = HashMap::new();
//   // 所有权在这里发生了转移
//   m2.insert(v2.clone(), v1);
//
//   assert_eq!(v2, "hello");
//
//    println!("Success!")
// }

//三方库 Hash 库
// use std::hash::BuildHasherDefault;
// use std::collections::HashMap;
// // 引入第三方的哈希函数
// use twox_hash::XxHash64;
//
// let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
// hash.insert(42, "the answer");
// assert_eq!(hash.get(&42), Some(&"the answer"));

//Type conversions
//============================================================================
// 修复错误，填空
// 不要移除任何代码
// fn main() {
//     let decimal = 97.123_f32;
//
//     let integer: u8 = decimal.ceil() as u8;
//
//     let c1: char = integer as char;
//     let c2 = integer as char;
//
//     assert_eq!(integer, 'b' as u8);
//
//     println!("Success!")
// }

// fn main() {
//     assert_eq!(u8::MAX, 255);
//     // 如上所示，u8 类型允许的最大值是 255.
//     // 因此以下代码会报溢出的错误： literal out of range for `u8`.
//     // **请仔细查看相应的编译错误，从中寻找到解决的办法**
//     // **不要修改 main 中的任何代码**
//     #[allow(overflowing_literals, unused_variables)]
//     //Truncation
//     let v = 1000 as u8;
//
//     println!("Success!")
// }

// #[allow(overflowing_literals)]
// fn main() {
//     assert_eq!(1000 as u16, 1000);
//
//     assert_eq!(1000 as u8, 232);
//
//     // 事实上，之前说的规则对于正整数而言，就是如下的取模
//     println!("1000 mod 256 is : {}", 1000 % 256);
//
//     assert_eq!(-1_i8 as u8, 255);
//
//     // 从 Rust 1.45 开始，当浮点数超出目标整数的范围时，转化会直接取正整数取值范围的最大或最小值
//     assert_eq!(300.1_f32 as u8, 255);
//     assert_eq!(-100.1_f32 as u8, 0);
//
//     // 上面的浮点数转换有一点性能损耗，如果大家对于某段代码有极致的性能要求，
//     // 可以考虑下面的方法，但是这些方法的结果可能会溢出并且返回一些无意义的值
//     // 总之，请小心使用
//     unsafe {
//         // 300.0 is 44
//         println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
//         // -100.0 as u8 is 156
//         println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
//         // nan as u8 is 0
//         println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
//     }
// }

// 填空
// fn main() {
//     let mut values: [i32; 2] = [1, 2];
//     let p1: *mut i32 = values.as_mut_ptr();
//     let first_address: usize = p1 as usize;
//     let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
//     let p2: *mut i32 = second_address as *mut i32; // p2 指向 values 数组中的第二个元素
//     unsafe {
//         // 将第二个元素加 1
//         *p2 += 1;
//     }
//
//     assert_eq!(values[1], 3);
//
//     println!("Success!")
// }

// fn main() {
//     let arr :[u64; 13] = [0; 13];
//     assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
//     let a: *const [u64] = &arr;
//     let b = a as *const [u8];
//     unsafe {
//         assert_eq!(std::mem::size_of_val(&*b), 13)
//     }
// }

// From/Into

// fn main() {
//     let my_str = "hello";
//
//     // 以下三个转换都依赖于一个事实：String 实现了 From<&str> 特征
//     let string1 = String::from(my_str);
//     let string2 = my_str.to_string();
//     // 这里需要显式地类型标注
//     let string3: String = my_str.into();
// }

// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);
//
//     // 使用两种方式修复错误
//     // 1. 哪个类型实现 From 特征 : impl From<char> for ? , 你可以查看一下之前提到的文档，来找到合适的类型
//     // 2. 上一章节中介绍过的某个关键字
//     // let i3: i32 = 'a'.into();
//     // let i3: i32 = 'a' as i32;
//     let i3: u32 = 'a'.into();
//
//     // 使用两种方法来解决错误
//     // let s: String = 'a' as String;
//     let s: String = 'a'.to_string();
//     let s: String = String::from('a');
//     let s: String = 'a'.into();
//
//     println!("Success!")
// }

// From 被包含在 `std::prelude` 中，因此我们没必要手动将其引入到当前作用域来
// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }
//
// impl From<i32> for Number {
//     // 实现 `from` 方法
//     fn from(num: i32) -> Number {
//         Number { value: num }
//     }
// }
//
// // 填空
// fn main() {
//     let num = Number::from(30);
//     assert_eq!(num.value, 30);
//
//     let num: Number = 30.into();
//     assert_eq!(num.value, 30);
//
//     println!("Success!")
// }

// use std::fs;
// use std::io;
// use std::num;
//
// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }
//
// impl From<io::Error> for CliError {
//     fn from(err: io::Error) -> CliError {
//         CliError::IoError(err)
//     }
// }
//
// impl From<num::ParseIntError> for CliError {
//     fn from(err: num::ParseIntError) -> CliError {
//         CliError::ParseError(err)
//     }
// }
//
// #[allow(dead_code)]
// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? 自动将 io::Error 转换成 CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }
//
// fn main() {
//     println!("Success!")
// }

// TryFrom 和 TryInto 也被包含在 `std::prelude` 中, 因此以下引入是没必要的
// use std::convert::TryInto;
// fn main() {
//     let n: i16 = 256;
//
//     // Into 特征拥有一个方法`into`,
//     // 因此 TryInto 有一个方法是 ?
//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };
//
//     assert_eq!(n, 0);
//
//     println!("Success!")
// }

// #[derive(Debug, PartialEq)]
// struct EvenNum(i32);
//
// impl TryFrom<i32> for EvenNum {
//     type Error = ();
//
//     // 实现 `try_from`
//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNum(value))
//         } else {
//             Err(())
//         }
//     }
// }
//
// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));
//
//     // 填空
//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNum(8)));
//     let result: Result<EvenNum, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));
//
//     println!("Success!")
// }

//其它转换

// use std::fmt::{self, Display};
//
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "The point is ({}, {})", self.x, self.y)
//     }
// }
//
// fn main() {
//     let origin = Point { x: 0, y: 0 };
//     // 填空
//     assert_eq!(origin.to_string(), "The point is (0, 0)");
//     assert_eq!(format!("{origin}"), "The point is (0, 0)");
//
//     println!("Success!")
// }

// 为了使用 `from_str` 方法, 你需要引入该特征到当前作用域中
// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".parse().unwrap();
//     let turbo_parsed = "10".parse::<i32>().unwrap();
//     let from_str = i32::from_str("20").unwrap();
//     let sum = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);
//
//     println!("Success!")
// }

// use std::str::FromStr;
// use std::num::ParseIntError;
//
// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32
// }
//
// impl FromStr for Point {
//     type Err = ParseIntError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
//                                  .split(',')
//                                  .map(|x| x.trim())
//                                  .collect();
//
//         let x_fromstr = coords[0].parse::<i32>()?;
//         let y_fromstr = coords[1].parse::<i32>()?;
//
//         Ok(Point { x: x_fromstr, y: y_fromstr })
//     }
// }
// fn main() {
//     // 使用两种方式填空
//     // 不要修改其它地方的代码
//     // let p = Point::from_str("(3, 4)");
//     // let p = "(3, 4)".parse::<Point>();
//     assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );
//
//     println!("Success!")
// }

//result and panic

// use core::panic;
//
//
// // 填空
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // 实现下面的代码
//         panic!("No lemonade please");
//      }
//
//     println!("Exercise Failed if printing out this line!");
// }
//
// fn main() {
//     drink("lemonade");
//
//     println!("Exercise Failed if printing out this line!");
// }

// 修复所有的 panic，让代码工作
// fn main() {
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);
//
//     let v = vec![1, 2, 3];
//     let ele = &v[2];
//     let ele = v.get(2).unwrap();
//
//     // 大部分时候编译器是可以帮我们提前发现溢出错误，并阻止编译通过。但是也有一些时候，这种溢出问题直到运行期才会出现
//     let v = production_rate_per_hour(2);
//
//     divide(15, 3);
//
//     println!("Success!")
// }
//
// fn divide(x: u8, y: u8) {
//     println!("{}", x / y)
// }
//
// fn production_rate_per_hour(speed: u8) -> f64 {
//     let cph: u8 = 221;
//     match speed {
//         1..=4 => (speed as f64 * cph as f64),
//         5..=8 => (speed as f64 * cph as f64) * 0.9,
//         9..=10 => (speed as f64 * cph as f64) * 0.77,
//         _ => 0 as f64,
//     }
// }
//
// pub fn working_items_per_minute(speed: u8) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }

// RUST_BACKTRACE=1 cargo run

//result and ?

// 填空并修复错误
// use std::num::ParseIntError;
//
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 = n1_str.parse::<i32>()?;
//     let n2 = n2_str.parse::<i32>()?;
//     Ok(n1 * n2)
// }
//
// fn main() {
//     let result = multiply("10", "2");
//     assert_eq!(result, Ok(20));
//
//     let result = multiply("t", "2");
//     assert_eq!(result.unwrap_or(8), 8);
//
//     println!("Success!")
// }

// use std::num::ParseIntError;
//
// // 使用 `?` 来实现 multiply
// // 不要使用 unwrap !
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 = n1_str.parse::<i32>()?;
//     let n2 = n2_str.parse::<i32>()?;
//     Ok(n1 * n2)
// }
//
// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!")
// }

// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_file1() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
//
// // 填空
// // 不要修改其它代码
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();
//
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//
//     Ok(s)
// }
//
// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!")
// }

// use std::num::ParseIntError;
//
// // 使用两种方式填空: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
//    // n_str.parse::<i32>().map(|n| n + 2)
//    n_str.parse::<i32>().and_then(|n| Ok(n + 2))
// }
//
// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);
//
//     println!("Success!")
// }

// use std::num::ParseIntError;
//
// // 使用 Result 重写后，我们使用模式匹配的方式来处理，而无需使用 `unwrap`
// // 但是这种写法实在过于啰嗦..
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1) => match n2_str.parse::<i32>() {
//             Ok(n2) => Ok(n1 * n2),
//             Err(e) => Err(e),
//         },
//         Err(e) => Err(e),
//     }
// }
//
// // 重写上面的 `multiply` ，让它尽量简洁
// // 提示：使用 `and_then` 和 `map`
// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     n1_str
//         .parse::<i32>()
//         .and_then(|n1| n2_str.parse::<i32>().map(|n2| n1 * n2))
// }
//
// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n) => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
//
// fn main() {
//     let twenty = multiply1("10", "2");
//     print(twenty);
//
//     // 下面的调用会提供更有帮助的错误信息
//     let tt = multiply("t", "2");
//     print(tt);
//
//     println!("Success!")
// }

// use std::num::ParseIntError;
//
// // 填空
// type Res<T> = Result<T, ParseIntError>;
//
// // 使用上面的别名来引用原来的 `Result` 类型
// fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
//     })
// }
//
// // 同样, 这里也使用了类型别名来简化代码
// fn print(result: Res<i32>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }
//
// fn main() {
//     print(multiply("10", "2"));
//     print(multiply("t", "2"));
//
//     println!("Success!")
// }

// use std::num::ParseIntError;
//
// fn main() -> Result<(), ParseIntError> {
//     // let number_str = "10";
//     let number_str = "t";
//     let number = match number_str.parse::<i32>() {
//         Ok(number)  => number,
//         Err(e) => return Err(e),
//     };
//     println!("{}", number);
//     Ok(())
// }

//Package and Crate
// cargo new hello_cargo
// cargo new hello_cargo --lib

/* 只使用注释让下面代码工作! */
// fn main() {
//     // todo!();
//     // unimplemented!();
//
//     assert_eq!(6, /*5 + */3 + 2 + 1 )
// }

//生命周期

/* 为 `i` 和 `borrow2` 标注合适的生命周期范围 */

// `i` 拥有最长的生命周期，因为它的作用域完整的包含了 `borrow1` 和 `borrow2` 。
// 而 `borrow1` 和 `borrow2` 的生命周期并无关联，因为它们的作用域没有重叠
fn main() {
    let i = 3;
    {
        let borrow1 = &i; // `borrow1` 生命周期开始. ──┐
        //                                                │
        println!("borrow1: {}", borrow1); //              │
    } // `borrow1` 生命周期结束. ──────────────────────────────────┘
    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }
}
