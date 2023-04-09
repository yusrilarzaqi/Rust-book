mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("hi mom")
        }
    } /* hosting */
} /* front_of_house */

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
