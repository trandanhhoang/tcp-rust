use tun::{AbstractDevice, Configuration};
use core::convert::TryInto;
use std::{io::{self, Read}};

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
    println!("create TUN interface success: {:?}", tun_name);
    println!("IP: 10.0.0.1/24");
    println!("Destination: 10.0.0.254");

    const MTU: usize = 1500; // maximum transmission unit
    let mut buf = [0u8; MTU];
    
    loop {
        let nbytes = tun_device.read(&mut buf).unwrap();

        println!("read {}, bytes {:x?}", nbytes, &buf[..nbytes]);
    }
    Ok(())
}
