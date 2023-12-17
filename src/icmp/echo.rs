use futures::SinkExt;
use packet::{ip as p_ip, Builder, Packet};
use tokio_util::codec::Framed;
use tun::{AsyncDevice, TunPacket, TunPacketCodec};

pub async fn echo(
    icmp: packet::icmp::echo::Packet<&&[u8]>,
    pkt: p_ip::v4::Packet<&[u8]>,
    stream: &mut Framed<AsyncDevice, TunPacketCodec>,
) -> Result<bool, anyhow::Error> {
    let reply = p_ip::v4::Builder::default()
        .id(0x42)?
        .ttl(64)?
        .source(pkt.destination())?
        .destination(pkt.source())?
        .icmp()?
        .echo()?
        .reply()?
        .identifier(icmp.identifier())?
        .sequence(icmp.sequence())?
        .payload(icmp.payload())?
        .build()?;
    stream.send(TunPacket::new(reply)).await?;
    Ok(true)
}
