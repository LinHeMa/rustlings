// variables4.rs
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a hint.

fn main() {
    // 沒有加上mut 不可以中途改變變數的數值
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
