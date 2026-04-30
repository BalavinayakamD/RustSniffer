# RustSniffer

This is a packet sniffer written in Rust using the pnet crate.

this is a learning project to get better at Rust and networking.

# future plans

Integrate this with an AI to analyze the packets and block malicious activity.

# how to run

```bash
sudo cargo run <Interface>
```
note : the <Interface> is the interface where you want to capture the traffic
Usage examples
-----------

List available interfaces:

```bash
cargo run -- --list
```

Capture on interface `eth0` and print text output (default):

```bash
cargo run -- --interface eth0
```

Capture 10 packets and print JSON output:

```bash
cargo run -- --interface eth0 --count 10 --output json
```

Capture TCP packets only (BPF filter):

```bash
cargo run -- --interface eth0 --filter "tcp"
```

Capture packets on port 80 (BPF filter):

```bash
cargo run -- --interface eth0 --filter "port 80"
```

Capture HTTPS traffic (port 443):

```bash
cargo run -- --interface eth0 --filter "tcp port 443"
```

Capture traffic from a specific IP:

```bash
cargo run -- --interface eth0 --filter "src 192.168.1.1"
```

Complex BPF filter example (TCP from 192.168.1.1 to port 443):

```bash
cargo run -- --interface eth0 --filter "tcp && src 192.168.1.1 && dst port 443"
```

Notes
- The `--filter` flag uses native **Berkeley Packet Filter (BPF)** syntax, compiled and attached at the kernel level for efficient filtering.
- Common BPF filter expressions: `tcp`, `udp`, `icmp`, `port N`, `src HOST`, `dst HOST`, `net CIDR`, etc.
- See [tcpdump man page](https://www.tcpdump.org/papers/sniffing-faq.html#q6) for BPF filter syntax reference.
- Press Ctrl-C to stop capturing — a summary of seen/matched packets and basic counts will be printed.
