# utun

Apple has built-in utun driver in MACOS in very early versions. This driver has several limitations. I will explain it briefly and everyone will understand.

1. The utun driver cannot be set to a specific network card name, which is different from the tun driver integrated in the LINUX version 2.x kernel.
2. The utun driver only supports P2P point-to-point mode, so you cannot set MAC, ARP and other L2 things.
3. The utun driver needs to clearly indicate whether the IP protocol type is AF_INET or AF_INET6, which is indicated by the four bytes before the IP packet header read in and written into utun.

# Structure
Apple is little endian machine, but network is big endian machine
