fn main() {
    let yaya = 'outer_loop: loop {
        loop {
            println!("again!");
            break 'outer_loop 1
        }
    };
    println!("{}", yaya)

}
