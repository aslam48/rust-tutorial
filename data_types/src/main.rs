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

    
}
