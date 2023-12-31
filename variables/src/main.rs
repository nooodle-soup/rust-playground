fn main() {
    // Mutability
    println!("--Mutability--");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    // Shadowing
    println!("--Shadowing--");
    let y = 1;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

}
