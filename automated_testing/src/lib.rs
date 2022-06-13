pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// unit tests exists inside the module to be tested
// integration test exist in its own directory
#[cfg(test)]
mod tests {
    // bring all parent's items into scope so they can be tested
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_1() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from(
                "two plus two does not equal four, quick maffs",
            ))
        }
    }

    #[test]
    fn testing_private_function() {
        // its possible to test private functions
        // i.e. function that are not pub
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "yaya? 1")]
    fn should_panic() {
        println!("????????????????????");
        let i = 1;
        panic!("yaya? {}", i);
    }

    #[ignore]
    #[test]
    fn ignored() {
        unimplemented!();
    }
}
