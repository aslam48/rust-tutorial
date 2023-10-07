// ** variables, constants and shadowing  **

fn main() {
    let  x = 4 + 3;
    println!("x is: {}", x);

    {
        let  x = x - 3;
    println!("x is: {}", x);
    }

    let x = x + 3;
    println!("x is: {}", x);
}




 // 2 ** constants **

// fn main() {
//     const ASLAM: u32 = 19;
//     println!("{}", ASLAM)
// }