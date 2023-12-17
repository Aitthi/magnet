mod echo;
use packet::ip;
use tokio_util::codec::Framed;
use tun::{AsyncDevice, TunPacketCodec};

pub struct Icmpv4Packet {}

impl Icmpv4Packet {
    pub async fn build(
        p: packet::icmp::Packet<&[u8]>,
        pkt: ip::v4::Packet<&[u8]>,
        stream: &mut Framed<AsyncDevice, TunPacketCodec>,
    ) -> Result<bool, anyhow::Error> {
        match p.echo() {
            Ok(icmp) => Ok(echo::echo(icmp, pkt, stream).await?),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}
