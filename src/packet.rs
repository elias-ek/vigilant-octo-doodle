use binrw::{BinRead, BinWrite, binrw};
use std::io::{self, Cursor};

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
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid magic bytes",
            ));
        }

        // Sanity check
        let expected_header_size = size_of::<PacketHeader>();
        if packet.header.header_size as usize != expected_header_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Incorrect header size",
            ));
        }

        if packet.header.checksum != crc32fast::hash(&packet.payload) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Incorrect checksum",
            ));
        }

        Ok(packet)
    }

    pub fn serialize(self: Self) -> Vec<u8> {
        let mut cursor = Cursor::new(Vec::new());
        Packet::write(&self, &mut cursor).expect("");

        cursor.into_inner()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn serialize_and_deserialize_packet() {
        let packet = Packet {
            header: PacketHeader {
                magic: MAGIC_BYTES,
                header_size: 20,
                payload_size: 0,
                packet_type: PacketType::A,
                checksum: 0,
            },
            payload: Vec::new(),
        };

        let data = packet.serialize();
        let new_packet = Packet::deserialize(&data).expect("succesfully deserialize packets");
        debug_assert_eq!(new_packet.header.magic, MAGIC_BYTES);
        debug_assert_eq!(new_packet.header.header_size, 20);
    }

    #[test]
    fn invalid_magic_bytes() {
        let packet = Packet {
            header: PacketHeader {
                magic: 0xDEADBEEF,
                header_size: 20,
                payload_size: 0,
                packet_type: PacketType::A,
                checksum: 0,
            },
            payload: Vec::new(),
        };

        let data = packet.serialize();
        let err = Packet::deserialize(&data).unwrap_err();
        debug_assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn invalid_header_size() {
        let packet = Packet {
            header: PacketHeader {
                magic: MAGIC_BYTES,
                header_size: 20 + 20,
                payload_size: 0,
                packet_type: PacketType::A,
                checksum: 0,
            },
            payload: Vec::new(),
        };

        let data = packet.serialize();
        let err = Packet::deserialize(&data).unwrap_err();
        debug_assert_eq!(err.kind(), io::ErrorKind::InvalidData);
    }
}
