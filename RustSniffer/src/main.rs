mod args;
mod capture;
// mod handlers;
// mod packet;
mod protocols;

fn main() {
    println!("Welcome to RustSniffer , this is a CLI packet sniffer");
    println!("This is a learning project to get better at Rust and networking");

    let interface = args::get_interface();
    capture::capture_packets(&interface);
}
