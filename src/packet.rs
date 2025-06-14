use std::{io::{self, Cursor}};
use binrw::{
    binrw,
    BinRead,
};


const MAGIC_BYTES: u32 = 0x13800818;

#[repr(u32)]
#[derive(Debug)]
#[binrw]
#[brw(little, repr=u32)]
enum PacketType {
    Ping = 1,
    Pong = 2,
    A = 3,
    B = 4,
    C = 5,
    D = 6,
    E = 7,
    F = 8,
    G = 9,
}

#[derive(Debug)]
#[repr(C)]
#[binrw]
#[brw(little)]
struct PacketHeader {
    magic: u32,
    header_size: u16,
    payload_size: u32,
    packet_type: PacketType,
    checksum: u32,
}

#[derive(Debug)]
#[binrw]
#[brw(little)]
pub struct Packet {
    header: PacketHeader,

    #[br(count = header.payload_size)]
    payload: Vec<u8>,
}

impl Packet {
    pub fn deserialize(data: &[u8]) -> io::Result<Self> {
        let mut cursor = Cursor::new(data);

        let packet = Packet::read(&mut cursor)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("binrw error: {e}")))?;

        if packet.header.magic != MAGIC_BYTES {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic bytes"));
        }

        // Sanity check
        let expected_header_size = size_of::<PacketHeader>();
        if packet.header.header_size as usize != expected_header_size {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect header size"));
        }

        if packet.header.checksum != crc32fast::hash(&packet.payload)  {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Incorrect checksum"));
        }

        Ok(packet)
    }
}
