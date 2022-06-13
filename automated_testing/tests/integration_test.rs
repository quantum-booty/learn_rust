use adder;

mod common;

#[test]
fn it_adds_two() {
    // use a helper function
    common::setup();
    assert_eq!(adder::add_two(2), 4);
}
