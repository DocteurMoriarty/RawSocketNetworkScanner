#[cfg(test)]
mod tests {
    use projet_rsns_morissetlarresacha::{
        packets::ethernet::pack_ethernet,
        structs::ethernet::EthernetHeader
    };

    #[test]
    fn test_pack_ethernet_basic() {
        let header = EthernetHeader {
            dst_mac: [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF],
            src_mac: [0x11, 0x22, 0x33, 0x44, 0x55, 0x66],
            ethertype: 0x0800,
        };

        let payload = vec![0xDE, 0xAD, 0xBE, 0xEF];

        let packet = pack_ethernet(&header, &payload).unwrap();

        assert_eq!(packet.len(), 14 + payload.len());

        assert_eq!(&packet[0..6], &header.dst_mac);
        assert_eq!(&packet[6..12], &header.src_mac);

        assert_eq!(&packet[12..14], &[0x08, 0x00]);

        assert_eq!(&packet[14..], &payload);
    }

    #[test]
    fn test_pack_ethernet_empty_payload() {
        let header = EthernetHeader {
            dst_mac: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
            src_mac: [0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B],
            ethertype: 0x86DD,
        };

        let payload = vec![];

        let packet = pack_ethernet(&header, &payload).unwrap();

        assert_eq!(packet.len(), 14);
        assert_eq!(&packet[0..6], &header.dst_mac);
        assert_eq!(&packet[6..12], &header.src_mac);
        assert_eq!(&packet[12..14], &[0x86, 0xDD]);
    }
}
