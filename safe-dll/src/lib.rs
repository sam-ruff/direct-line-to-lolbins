#[unsafe(no_mangle)]
pub extern "C" fn greet() {
    println!("hello from a safe binary");
}
