pub fn get_interface() -> String {
    std::env::args()
        .nth(1)
        .expect("Usage: cargo run <interface>")
}