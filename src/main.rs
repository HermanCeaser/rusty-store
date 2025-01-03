use rusty_store::add;

fn main() {
    let a: u64 = 4;
    let b: u64 = 12;
    let sum = add(a, b);
    println!("The sum of {a} and {b} is {sum}.")
}