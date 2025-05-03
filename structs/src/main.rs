// struct User {
//     userName: String,
//     email: String,
//     sign_in_count: usize, 
//     active: bool, 
// }

// fn main() {
//     let mut user1 = User {
//         userName: String::from("aslam48"),
//         email: String::from("aslam@gmail.com"),
//         sign_in_count: 1,
//         active: true,
//     };

//     let name = user1.userName;
//     println!("User name: {}", name);
//      user1.userName = String::from("aslam47");
//     println!("User name: {}", user1.userName);


//     let user2 = build_user(String::from("iliyas@gmail.com"), String::from("aslam50"));
//     println!("User2 email and name: {} {}", user2.email, user2.userName);
// }

// fn build_user(email: String, userName: String) -> User {
//     User {
//         email,
//         userName,
//         active: true,
//         sign_in_count: 1,
//     }
// }




// tupple struct 
// fn main() {
//     struct color(i32, i32, i32);
//     struct point(i32, i32, i32);
// }






// normal afrument type 
// fn main() {
//     let width1 = 50;
//     let height1 = 2;
//     println!("the caculation: {}", area(width1, height1))
// }

// fn area(width: i32, height: i32) -> i32 {
//     width * height
// }





// tuple type  argument decleration 
// fn main() {
//     let rect1 = (30, 50);
//     println!("the caculation: {}", area(rect1))
// }

// fn area(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }





// Refactoring with Structs: Adding More Meaning
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//  fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("this is our output: {}", area(&rect1));
//  }

//  fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
//  }



// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }



// Defining Methods impl

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// } 






// Methods with More Parameters

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     // fn area(&self) -> u32 {
//     //     self.width * self.height
//     // }

//     fn can_hold(&self, other:&Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }


// fn main() {
//     let rect = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 20,
//         height: 40,
//     };

//     let rect3 = Rectangle {
//         width: 40,
//         height: 50,
//     };

//     println!("rect can hold rect2 is {}", rect.can_hold(&rect2));
//     println!("rect can hold rect3 is {}", rect.can_hold(&rect3));

//     // println!(
//     //     "The area of the rectangle is {} square pixels.",
//     //     rect1.area()
//     // );
// } 






// Associated Functions
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // fn area(&self) -> u32 {
    //     self.width * self.height
    // }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(Size: u32) -> Rectangle {
        Rectangle {
            width: Size,
            height: Size,
        }

    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 50,
    };
    let square = Rectangle::square(40);
    println!("square is {square:#?}");
    
    println!("rect can hold rect2 is {}", rect.can_hold(&rect2));
    println!("rect can hold rect3 is {}", rect.can_hold(&rect3));

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
} 