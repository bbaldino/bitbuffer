# bitbuffer
A bit-addressable buffer in Rust.

Say you have some data formatted like so:
```
0                   1                   2                   3
0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|V  |P|    RC   |   PT          |             length            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```
And you receive it in a `Vec<u8>`.  You can parse it like so:

```
fn main() {
    let data: Vec<u8> = ...;
    let mut buf = BitBuffer::new(data);
    let parsed = parse_data(&mut buf);
    ...
}

fn parse_data(buf: &mut dyn ReadableBuf) -> ParsedData {
    // Read the first 2 bits as a u8
    let version = buf.read_bits_as<u8>(2);
    // Read the next bit as a bool
    let has_padding = buf.read_bit_as_bool();
    // Read the next 5 bits as a u8
    let report_count = buf.read_bits_as<u8>(5);
    // Read the next byte
    let payload_type = buf.read_u8();
    // Read the next 2 bytes
    let length = buf.read_u16();
    
    ...
}
```
