# Prerequisites
- MacOS M1 Pro
  
- video: https://www.youtube.com/watch?v=bzja9fQWzdA

# set up from scratch

```bash
cargo new --bin tcp-rust
```

- step 1: add tun-tap dependencies for reading network packet manually

```bash
cargo build
```

| Command                 | Meaning                                      |
| ----------------------- | -------------------------------------------- |
| `cargo build`           | Build in debug mode (fast build, slow run)   |
| `cargo build --release` | Build in release mode (slow build, fast run) |


First error

```rust
thread 'main' panicked at src/main.rs:5:48:
Failed to create a TUN device: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

tun-tap only work in linux (@see `utun-macos.md`)

step 2: Change to [tun](https://docs.rs/tun/latest/tun/), then run bash below

```bash
cargo clean
cargo build --release
cargo run
```

error: name "utun" 
```rust
Custom { kind: Other, error: ParseNum(ParseIntError { kind: Empty }) }
```
dependencies need `utun + number`

step 2: update name = "utun0"

error
```rust
Error: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
```
work with network in macos need run with sudo

step 3: run with sudo

```bash
sudo cargo run 
```

error: 
```rust
Error: Os { code: 16, kind: ResourceBusy, message: "Resource busy" }
```
utun0 have used by vpn, or something like that

step 4: check utun in device

```bash
$ ifconfig | grep utun

utun0: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1500
	inet6 fe80::1ef8:556d:3954:9d5f%utun0 prefixlen 64 scopeid 0xf 
utun1: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1380
	inet6 fe80::9e0f:8f1d:de53:36ca%utun1 prefixlen 64 scopeid 0x10 
utun2: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 2000
	inet6 fe80::7b96:d29e:1f50:4169%utun2 prefixlen 64 scopeid 0x11 
utun3: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1000
	inet6 fe80::ce81:b1c:bd2c:69e%utun3 prefixlen 64 scopeid 0x12 
utun4: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1380
	inet6 fe80::b976:6144:3ae6:48c5%utun4 prefixlen 64 scopeid 0x13 
utun5: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1380
	inet6 fe80::e471:f54a:274f:35bd%utun5 prefixlen 64 scopeid 0x16
```

step 5: remove name in config -> it will create by default

```rust
config
    .name("utun0") // remove this line
    .address((10, 0, 0, 1)) // Địa chỉ IP của giao diện TUN
    .netmask((255, 255, 255, 0)) // Netmask
    .destination((10, 0, 0, 254)) // Destination (thường là gateway ảo)
    .up(); // Kích hoạt giao diện khi tạo
```

step 6: run SUCCESS, then check with cmd

```bash
utun6: flags=8051<UP,POINTOPOINT,RUNNING,MULTICAST> mtu 1500
```

step 7: try to ping

```bash
$ ping 10.0.0.1

ING 10.0.0.1 (10.0.0.1): 56 data bytes
Request timeout for icmp_seq 0
Request timeout for icmp_seq 1
Request timeout for icmp_seq 2
Request timeout for icmp_seq 3
```

step 8: add loop and try to ping again

```rust
read 84 bytes, flag 4500, proto 54), [45, 0, 0, 54, bd, 86, 0, 0, 40, 1, a9, 21, a, 0, 0, 1, a, 0, 0, 1, 8, 0, f2, 2f, 23, 28, 0, 0, 68, 75, 39, 26, 0, 1, 56, 8, 8, 9, a, b, c, d, e, f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]
read 84 bytes, flag 4500, proto 54), [45, 0, 0, 54, b8, 89, 0, 0, 40, 1, ae, 1e, a, 0, 0, 1, a, 0, 0, 1, 8, 0, e6, 87, 23, 28, 0, 1, 68, 75, 39, 27, 0, 1, 61, ae, 8, 9, a, b, c, d, e, f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c, 1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e, 2f, 30, 31, 32, 33, 34, 35, 36, 37]
```

step 9: set up run.sh with visudo -> it quite buggy -> ignore it

step 10: create alias scgr = `sudo cargo build && sudo cargo run`

step 11: For the first time, I still think I need to parse 4 byte but no, library meh/rust-tun strips it

```rust
    // still need to read 1504 bytes here, have 4 bytes for address family
    const MTU: usize = 1500; // maximum transmission unit
    const UTUN_ADDRESS_FAMILY: usize = 4; // AF_INET (0x02): IPv4 or AF_INET6 (0x1E): IPv6
    const CHUNK: usize = MTU + UTUN_ADDRESS_FAMILY;
    let mut buf = [0u8; CHUNK];
```

step 12: Now we know all data is header + payload of IP, let explore http://www.faqs.org/rfcs/rfc791.html (@see ip-protocol.md)

step 13: Try to read header of IP and TCP, got things below when use netcat `nc 10.0.0.1 80`

```txt
Received packet with header:Ipv4Header { dscp: IpDscp(0), ecn: NotEct, total_len: 64,
identification: 0, dont_fragment: true, more_fragments: false, fragment_offset: IpFragOffset(0),
\time_to_live: 64, protocol: 6 (TCP - Transmission Control), header_checksum: 9911,
source: [10, 0, 0, 1], destination: [10, 0, 0, 1], options: [] }, and rest:
[248, 6, 0, 80, 191, 200, 29, 20, 0, 0, 0, 0, 176, 2, 255, 255, 144, 176, 0, 0, 2, 4,
5, 180, 1, 3, 3, 6, 1, 1, 8, 10, 202, 234, 242, 43, 0, 0, 0, 0, 4, 2, 0, 0]

TCP Header: TcpHeaderSlice { slice: [248, 6, 0, 80, 191, 200, 29, 20, 0, 0, 0, 0, 176, 2,
255, 255, 144, 176, 0, 0, 2, 4, 5, 180, 1, 3, 3, 6, 1, 1, 8, 10, 202, 234, 242, 43, 0, 0,
0, 0, 4, 2, 0, 0] }
```



