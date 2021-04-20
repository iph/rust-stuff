fn  not_main(){
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}

        pub fn seat_at_tables(){}
    }

    mod serving{
        fn take_order(){}

        fn serve_order(){}

        fn take_payment(){}
    }
}

fn server_order(){}
mod back_of_house {
    fn fix_incorrect_order() {

    }

    fn cook_order(){}
}