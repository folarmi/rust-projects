// // // use std::env;
// // // use std::process;

// // // use minigrep::Config;

// // #[derive(Debug, PartialEq, Copy, Clone)]
// // enum ShirtColor {
// //     Red,
// //     Blue
// // }

// // struct Inventory {
// //     shirts: Vec<ShirtColor>,
// // }

// // impl Inventory {
// //     fn giveaway(&self, user_preference:Option<ShirtColor>) -> ShirtColor {
// //         user_preference.unwrap_or_else(|| self.most_stocked())
// //     }

// //         fn most_stocked(&self) -> ShirtColor {
// //             let mut num_red = 0;
// //             let mut num_blue = 0;

// //             for color in &self.shirts{
// //                 match color {
// //                     ShirtColor::Red => num_red += 1,
// //                     ShirtColor::Blue => num_blue += 1,
// //                 }
// //             }

// //             if num_red > num_blue {
// //                 ShirtColor::Red
// //             } else {
// //                 ShirtColor::Blue
// //             }
// //         }
 
// // }
// // fn main() {

// //   let store = Inventory {
// //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
// //   };

// //   let user_pref1 = Some(ShirtColor::Red);
// //   let giveaway1 = store.giveaway(user_pref1);

// //   println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

// //   let user_pref2 = None;
// //   let giveaway2 = store.giveaway(user_pref2);

// //   println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

// //     // let args: Vec<String> = env::args().collect();

// //     // let config = Config::build(&args).unwrap_or_else(|err| {
// //     //     eprint!("Problem parsing arguments: {err}");
// //     //     process::exit(1);
// //     // });

// //     // if let Err(e) = minigrep::run(config) {
// //     //     eprint!("Application error: {e}");
// //     //     process::exit(1);
// //     // }
    
// // }
// use std::thread;
// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);

//     thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap()
// }

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// 
fn main() {
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
    
}