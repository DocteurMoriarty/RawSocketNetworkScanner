```bash
cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --debug_file=./test_send.pcap \
  --debug_format=pcap \
  --dry_run
```

Ouvrez ensuite `test_send.pcap` dans Wireshark.

```bash
sudo cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --debug_file=./test_send.pcap \
  --debug_format=pcap
```


   ```
   ip.src == 192.168.100.10 || ip.dst == 192.168.100.10 || udp.port == 9999
   ```

```bash
sudo cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --timeout_ms=1000 \
  --ip_bitfield=0x00 \
  --debug_file=./test_capture.pcap \
  --debug_format=pcap
```


