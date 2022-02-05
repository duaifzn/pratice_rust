mod house;
fn main() {
    let item1 = house::bath_room::get_item();
    let item2 = house::living_room::get_item();
    dbg!(item1);
    dbg!(item2);
}
