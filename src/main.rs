mod ip;

use futures::StreamExt;
use packet::{icmp, ip as p_ip, Packet};

#[tokio::main]
async fn main() {
    println!("Starting");
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 1))
        .destination((10, 0, 0, 2))
        .netmask((255, 255, 255, 0))
        .up();

    #[cfg(target_os = "linux")]
    config.platform(|config| {
        config.packet_information(true);
    });

    let dev = tun::create_as_async(&config).unwrap();

    let mut stream = dev.into_framed();

    while let Some(packet) = stream.next().await {
        let Ok(pkt) = packet else { continue };
        let Ok(p_ip::Packet::V4(pkt_v4)) = p_ip::Packet::new(pkt.get_bytes()) else {
            continue;
        };

        // icmp
        if let Ok(icmp) = icmp::Packet::new(pkt_v4.payload()) {
            let _ = ip::v4::icmp::Icmpv4Packet::build(icmp, pkt_v4, &mut stream)
                .await
                .is_ok();
        }
    }
}
