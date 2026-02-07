fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}