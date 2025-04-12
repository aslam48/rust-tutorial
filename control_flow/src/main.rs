// fn main() {
//     let number = 3;
//     if number < 5 {
//         println!("this condiction was met");
//     } else {
//         println!("this condiction was not met");
//     }
// }


fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 9 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}