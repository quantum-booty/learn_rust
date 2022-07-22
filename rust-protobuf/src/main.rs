use rust_protobuf::{create_large_shirt, serialize_shirt, deserialize_shirt};
fn main() {
    let shirt = create_large_shirt("Blue".to_string());
    println!("{:?}", shirt);
    let bytes = serialize_shirt(&shirt);
    println!("{:?}", bytes);
    let shirt = deserialize_shirt(&bytes).unwrap();
    println!("{:?}", shirt);
}
