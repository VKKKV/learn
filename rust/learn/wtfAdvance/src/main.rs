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
fn example1() {
    // `T: Trait` 是最常使用的方式
    // `T: Fn(u32) -> u32` 说明 `T` 只能接收闭包类型的参数
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10), __);
    assert_eq!(cacher.value(15), __);
}


fn example2() {
    // 还可以使用 `where` 来约束 T
    struct Cacher<T>
        where T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(20), __);
    assert_eq!(cacher.value(25), __);
}



fn main() {
    example1();
    example2();

    println!("Success!")
}
