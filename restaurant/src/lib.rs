// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::serving;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();
    serving::take_payment();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries")
}