struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

pub fn drop() {
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let _d = CustomSmartPointer{
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer{
        data: String::from("yaya")
    };

    // can drop early by using std::mem::drop function
    std::mem::drop(e);

    println!("CustomSmartPointers created.");
    // rust automatically calls drop when c and d went out of scope
    // variables ar edropped in reverse order of their creation
    // so d is dropped before c
}

