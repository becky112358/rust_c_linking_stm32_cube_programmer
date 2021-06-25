#[link(name = "CubeProgrammer_API")]
extern {
    fn disconnect();
}

fn main() {
    unsafe {
        disconnect();
    }
}
