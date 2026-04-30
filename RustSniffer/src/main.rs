mod args;
mod capture;
mod handlers;
mod packet;
mod protocols;

use args::parse_args;
use handlers::OutputFormat;

fn main() {
    println!("Welcome to RustSniffer , this is a CLI packet sniffer");
    println!("This is a learning project to get better at Rust and networking");

    let cli = parse_args();

    if cli.list {
        let interfaces = pnet::datalink::interfaces();
        println!("Available interfaces:");
        for iface in interfaces {
            println!("- {}", iface.name);
        }
        return;
    }

    let interface = match cli.interface {
        Some(i) => i,
        None => {
            eprintln!("Error: no interface provided. Use --list to see available interfaces or --interface <name>");
            std::process::exit(1);
        }
    };

    let output_fmt = OutputFormat::from_str(&cli.output);

    // Setup Ctrl-C handler to exit cleanly
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, std::sync::atomic::Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");
    }

    capture::capture_packets(&interface, output_fmt, cli.count, running, cli.filter);
}
