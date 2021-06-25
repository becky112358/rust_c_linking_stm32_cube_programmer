use libstm32_cube_programmer_sys::disconnect;

fn main() {
    println!("Hello, world!");
    unsafe {
        disconnect();
    }
}
