mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;
use std::collections::HashMap;

use std::fmt;

use std::io;
use std::fmt::Error;


fn function1() -> fmt::Result {
    Result::Ok(())
}

fn function2() -> fmt::Result {
    Err(Error)
}


pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}