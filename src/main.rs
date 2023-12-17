mod icmp;

use futures::{StreamExt, SinkExt};
use packet::{icmp as p_icmp, ip as p_ip, Packet};

#[tokio::main]
async fn main() {
    println!("Starting");
    let mut config = tun::Configuration::default();
    config
        .address((10, 0, 0, 1))
        // .destination((10, 0, 0, 2))
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
        let p = pkt.get_bytes();
        // println!("p: {:#?}", p);
        let Ok(p_ip::Packet::V4(pkt_v4)) = p_ip::Packet::new(p) else {
            continue;
        };

        // println!("pkt_v4: {:#?}", pkt_v4);
        println!("ip source: {}", pkt_v4.source());
        println!("ip destination: {}", pkt_v4.destination());
        println!("\n");

        // // icmp
        if let Ok(icmp) = p_icmp::Packet::new(pkt_v4.payload()) {
            match icmp::Icmpv4Packet::build(icmp, pkt_v4, &mut stream)
            .await
            .is_ok() {
                true => continue,
                false => {},
            }
        }
        
        // let src = [10, 0, 0, 2];
        // let dst = [10, 0, 0, 1];

        // let mut np = p.to_vec()[..12].to_vec();
        // np.extend_from_slice(&src);
        // np.extend_from_slice(&dst);
        // np.extend_from_slice(&p[20..]);
        // let _ = stream.send(tun::TunPacket::new(np)).await.unwrap();
    }
}
