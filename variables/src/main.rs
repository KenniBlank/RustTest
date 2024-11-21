fn main() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 6;
    let spaces = "    ";
    {
        let x = 10;
        let spaces = spaces.len();
        println!("The value of x is {x} and space length is {spaces}");
    }
    println!("The value of x is {x} and space is {spaces}.");
}
