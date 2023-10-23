fn main() {
    println!("Hello, aslam!");
    test_aslam();
}

fn test_aslam() {
    let x: f32 = 255.0;
    let y: u8 = x as u8 - 5;
    println!("{}", y);

    let goat: bool = true;
    // let goat = false;
    println!("{}", goat);

    let ages: [u16; 6] = [34, 40, 55, 20, 12, 32];
    println!("{:?}", ages);

    let newage: &[u16] = &ages[1..4];
    println!("{:?}", newage);

    let tup: (&str, &str, u8) = ("okey", "hi", 40);
    println!("{:?}", tup.2);

    // condictional statment 
    let number: i32 = 10;
    if number > 11 {
        print!("condiction was true")
    } else {
        print!("condiction was not true")
    }

    //   loop 
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);


    // while loop 
    let mut number = 1000;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}
