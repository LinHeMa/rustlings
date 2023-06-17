// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);

    {
        // shadowing: 透過 shadowing 可以重複使用變數
        let mut number = 0;
        number = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}
