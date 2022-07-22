use add_one;

fn main() {
    let num = 10;
    println!(
        "Hello moggers {} plus one is {}, quick maffs!",
        num,
        add_one::add_one(num)
    );
}
