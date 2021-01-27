use crate::readable_buf::ReadableBufExtra;
use crate::some_readable_buf::SomeReadableBuf;

fn parse_header(buf: &SomeReadableBuf) {
    let _version = buf.read_u8();
    let _marker = buf.read_bits_as::<u8>(3);
    let _p_type = buf.read_bits_as::<u8>(5);
    let _something_else = buf.read_u16();
    let sub_header = buf.sub_buffer(4).unwrap();

    parse_sub_header(&sub_header);
}

fn parse_sub_header(buf: &SomeReadableBuf) {
    let _sub_type = buf.read_u8().unwrap();
    parse_payload(&buf.sub_buffer(4).unwrap());
}

fn parse_payload(buf: &SomeReadableBuf) {
    let _p = buf.read_u32().unwrap();
}
