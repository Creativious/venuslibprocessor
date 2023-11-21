
fn main() {
    println!("Hello, world!");

}

#[no_mangle]
pub extern "C" fn test() {
    println!("Test received!");
}
