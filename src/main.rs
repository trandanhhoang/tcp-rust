use tun::{AbstractDevice, Configuration};
use std::{env::consts::UTUN_ADDRESS_FAMILY, io::{self, Read}};

fn main() -> io::Result<()> {
    let mut config = Configuration::default();

    // Trên macOS, tên giao diện TUN phải có dạng utunX (ví dụ: utun0, utun1)
    config
        .address((10, 0, 0, 1)) // Địa chỉ IP của giao diện TUN
        .netmask((255, 255, 255, 0)) // Netmask
        .destination((10, 0, 0, 254)) // Destination (thường là gateway ảo)
        .up(); // Kích hoạt giao diện khi tạo

    let mut tun_device = tun::create(&config)?;

    let tun_name = tun_device.tun_name();
    println!("Đã tạo giao diện TUN thành công: {:?}", tun_name);
    println!("Địa chỉ IP: 10.0.0.1/24");
    println!("Destination: 10.0.0.254");

    // still need to read 1504 bytes here, have 4 bytes for address family
    let mut MTU =  1500; // maximum transmission unit
    let mut UTUN_ADDRESS_FAMILY = 4; // AF_INET (0x02): IPv4 or AF_INET6 (0x1E): IPv6
    let mut CHUNK =  MTU + UTUN_ADDRESS_FAMILY;
    let mut buf = [0u8; CHUNK];
    
    loop {
        // bytes này bao gồm header và payload
        // có thể lấy ra và biết được là type nào, IP, ICMP
        let nbytes = tun_device.read(&mut buf).unwrap();

        println!("read {} bytes {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}
