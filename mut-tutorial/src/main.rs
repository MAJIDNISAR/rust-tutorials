fn main() {
    // by default every variable in rust is immutable
    // let x=4 cannot be changed in the program
    let mut x = 4;

    println!("x:{}", x);
    x = 5; // can change since mutability
    println!("x:{}", x);
    // let us define our own scop with {}
    {
        let x = 2;
        println!("Internal Scope x:{}", x);
    }
    let x = x + 1; //5+1= 6  and not 3
    println!("x:{}", x);

    {
        let x = x + 3;
        println!("From external scope to intenal scope x:{}", x);
    }

    let x = 7; // we can redefine variables in rust
    println!("x:{}", x);
    let x = "hello";
    println!("changing type using let x:{}", x);
    // x = "hello";
    // println!("changing type using let x:{}", x);
    // this will through an error
    // cannot assign twice to immutable variable, without using let
}
