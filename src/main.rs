fn main() {
    // Variables en Rust
    // let;             Variable como constante
    // let mut;         Variable normal 
    // --
    let name = "Carlos";
    let lastname = "some..";
    println!("Hello, {} {}!", name, lastname);
    // lastname = "Leon";
    // name = "some"

    // Condiciones
    let age = 20;

    if age < 18 {
        println!("eres menor de edad");
    } else {
        println!("eres mayor de edad");
    }
}
