use restaurant::front_of_house;

fn main() {
    front_of_house::hosting::add_to_wait_list();
    front_of_house::serving::serve_order();
    restaurant::back_of_house::take_care_trash();
}