# Prerequisites
- MacOS M1 Pro

# set up from scratch

```bash
cargo new --bin tcp-rust
```

- add tun-tap in dependencies -> but it's don't work on MacOs

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

# Time to run
step 1: Change to [tun](https://docs.rs/tun/latest/tun/), then run bash below

```bash
cargo clean
cargo build --release
cargo run
```

First error: name "utun"
```rust
Custom { kind: Other, error: ParseNum(ParseIntError { kind: Empty }) }
```

step 2: update name = "utun0"

Second error: 
```rust
Error: Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }
```

step 3: run with sudo

```bash
sudo cargo run 
```

Third error: 
```rust
Error: Os { code: 16, kind: ResourceBusy, message: "Resource busy" }
```

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

step 5: remove name in config

```rust
config
    .name("utun0") // remove this line
    .address((10, 0, 0, 1)) // Địa chỉ IP của giao diện TUN
    .netmask((255, 255, 255, 0)) // Netmask
    .destination((10, 0, 0, 254)) // Destination (thường là gateway ảo)
    .up(); // Kích hoạt giao diện khi tạo
```

step 6: run SUCCESS and check

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

step 9: set up run.sh with visudo



