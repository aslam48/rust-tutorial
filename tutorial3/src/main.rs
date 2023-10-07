// datatypes 
// 3 types of data types premitive, scaler and compound

fn main() {
    let floating_point: f32 = 10.92;
    let true_or_false: bool = false;
    let string: &str = "aslam";
    let letter: char = 'a';
    println!("{}", floating_point);
    println!("{}", true_or_false);
    println!("{}", string);
    println!("{}", letter);


  // ** compound types 

  let mut tup: (i32, bool, char) = (1, true, 's');
   tup.0 = 10;
  println!("{}", tup.0);


  // ** Arrays

  let arr: [i32; 6] = [1,2,3,4,5,6];
  arr[4];
  println!("arr is {}", arr[2]);
}
