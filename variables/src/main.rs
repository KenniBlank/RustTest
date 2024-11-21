fn main() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 6;
    let spaces = "    ";
    {
        let x = 10;
        let spaces = spaces.len();
        println!("The value of x is {x} and space length is {spaces}");

        {
            let decimal: i32 = 42; //Decimal Integer
            let octal: i32 = 0o_52; // Octal for 42
            let hex: i32 = 0x_2A; // Hexadecimal for 42
            let binary: i32 = 0b_101010; // Binary for 42
            let normal = decimal + 10i32; // Addition?
            println!(
                "
                In decimal: {decimal}
                In Octal: {octal}
                In Hexadecimal: {hex}
                In binary: {binary}
                Normal: {normal}
                "
            );
        }
    }
    println!("The value of x is {x} and space is {spaces}.");
}
