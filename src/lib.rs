mod front_of_house;
pub use crate::front_of_house::hosting;

fn deliver_order(){}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }

    fn cook_order(){}

    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_1(){
    crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn eat_at_restaurant2(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast)
}
