# IP protocol

http://www.faqs.org/rfcs/rfc791.html

## Internet header format

A summary of the contents of the internet header follows:

```txt
    0                   1                   2                   3   
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |Version|  IHL  |Type of Service|          Total Length         |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |         Identification        |Flags|      Fragment Offset    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |  Time to Live |    Protocol   |         Header Checksum       |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                       Source Address                          |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Destination Address                        |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                    Options                    |    Padding    |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
                    Example Internet Datagram Header
```

## My code
- when we ping utun, we receive data below

```txt
read 84, bytes [45, 0, 0, 54, 40, 45, 0, 0, 40, 1, 26, 63, a, 0, 0, 1,
a, 0, 0, 1, 8, 0, 46, 61, 89, cc, 0, 0, 68, 78, 53, 45, 0, 9, 81, 8, 8,
9, a, b, c, d, e, f, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1a, 1b, 1c,
1d, 1e, 1f, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 2a, 2b, 2c, 2d, 2e,
2f, 30, 31, 32, 33, 34, 35, 36, 37]
```

- Each word is a hex, represent for 8 bit, 1 byte (e.g: 45 = 0100_0101)
  
## IPv4 Header Analysis (20 bytes)

- Byte 0: Version and IHL (0x45) with Raw Value: 0x45 = 01000101
  - Version: 4 - Indicates IPv4 protocol
  - IHL: 5 - Internet Header Length = 5 × 4 = 20 bytes