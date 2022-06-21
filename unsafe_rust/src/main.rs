#![allow(dead_code)]
use core::slice;

fn main() {
    // unsafe superpowers
    // dereference a raw pointer
    // call an unsafe function or method
    // access or modify a mutable static variable
    // access fields of unions (mostly for interfacing with C code)
    deref_a_raw_pointer();
    call_unsafe_function_or_method();
    using_extern_functions_to_call_external_code();
    access_and_modify_a_mutable_static_variable();
}

// const have no fixed address in memory, and is inlined to each place which uses them
// which means they are put directly into the binary on the places which uses them
// static have a fixed address in memory, their value is loaded from this fixed address each place
// which uses them
const YAYA: u32 = 0;
static mut COUNTER: u32 = 0;

fn access_and_modify_a_mutable_static_variable() {
    unsafe {
        println!("count {}", COUNTER);
        COUNTER += 1;
        println!("count {}", COUNTER);
    }
}

fn using_extern_functions_to_call_external_code() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("calling extern function {}", abs(-2));
    }
}

fn calling_rust_function_from_other_languages() {
    #[no_mangle]
    extern "C" fn call_from_c() {
        println!("just called a Rust function from C");
    }
}

fn call_unsafe_function_or_method() {
    // unsafe functions has requirements we need to uphold when we call this function, because rust
    // can't guarantee we've met these conditions
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn deref_a_raw_pointer() {
    // not allowed to have both mutable and immutable reference
    let mut num2 = 5;
    let r3 = &num2;
    // below line not allowed
    // let r4 = &mut num2;

    // raw pointer does allow to have both
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);
    unsafe {
        // can only deref in unsafe block
        println!("r1 is: {:?}", *r1);
        println!("r2 is: {:?}", *r2);
    };
}
