fn main() {
    print_number(add_one(4));

    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);

    for x in 0..10 {
        println!("{}", x); // x: i32
    }
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
