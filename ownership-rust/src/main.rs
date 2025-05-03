// fn main() {
// let mut word = String::from("hello");
// word.push_str(" world");
// println!("{}", word);
// }


fn main(){
    let  s1 = String::from("hello");

    let len =  calulate_length(&s1);

   println!("the length of {s1} is {len}");
}

fn calulate_length(s: &String) -> usize {
    s.len()
}


