

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function from C!");
}

#[no_mangle]
pub extern "C" fn call_add(x:i32, y:i32)-> i32 {
    x + y
}