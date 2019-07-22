use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_cbor::de::from_mut_slice;
use serde_cbor::ser::SliceWrite;
use serde_cbor::Serializer;

#[derive(Debug, PartialEq)]
pub struct Message1 {
    pub r#type: i32,
    pub suite: i32,
    pub x_u: Vec<u8>,
    pub c_u: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawMessage1(
    i32,
    i32,
    #[serde(with = "serde_bytes")] Vec<u8>,
    #[serde(with = "serde_bytes")] Vec<u8>,
);

pub fn serialize_message_1(msg: Message1) -> Result<Vec<u8>, &'static str> {
    // Pack the data into a structure that nicely serializes almost into
    // what we want to have as the actual bytes for the EDHOC message
    let raw_msg = RawMessage1(msg.r#type, msg.suite, msg.x_u, msg.c_u);

    // Initialize a buffer, as well as a writer and serializer relying on it
    let mut buf = [0u8; 128];
    let writer = SliceWrite::new(&mut buf);
    let mut serializer = Serializer::new(writer);
    // Attempt serialization and determine the length
    if let Err(_) = raw_msg.serialize(&mut serializer) {
        return Err("Unable to serialize");
    }
    let writer = serializer.into_inner();
    let size = writer.bytes_written();

    // What we have now is a fixed-length CBOR array with 4 items.
    // What we want is just the sequence of items, so we can simply omit the
    // first byte (indicating array type and length), and get the items.
    Ok(buf[1..size].to_vec())
}

pub fn deserialize_message_1(
    msg: &mut [u8],
) -> Result<Message1, &'static str> {
    // We receive a sequence of 4 CBOR items. For parsing we need an array, so
    // start a CBOR array of length 4.
    let mut cbor_arr = vec![0x84];
    // After the start byte, insert the message (sequence of CBOR items)
    cbor_arr.extend(&msg[..]);

    // Now we can try to deserialize that into our raw message format
    let raw_msg: RawMessage1 = match from_mut_slice(&mut cbor_arr) {
        Ok(msg) => msg,
        _ => return Err("Unable to deserialize"),
    };

    // On success, just move the items into the "nice" message structure
    Ok(Message1 {
        r#type: raw_msg.0,
        suite: raw_msg.1,
        x_u: raw_msg.2,
        c_u: raw_msg.3,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let original = Message1 {
            r#type: 1,
            suite: 0,
            x_u: vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            c_u: vec![195],
        };
        let bytes = vec![
            0x01, 0x00, 0x58, 0x20, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
            0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x41, 0xC3,
        ];

        assert_eq!(serialize_message_1(original).unwrap(), bytes);
    }

    #[test]
    fn deserialize() {
        let original = Message1 {
            r#type: 1,
            suite: 0,
            x_u: vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ],
            c_u: vec![195],
        };
        let mut bytes = vec![
            0x01, 0x00, 0x58, 0x20, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
            0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11,
            0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C,
            0x1D, 0x1E, 0x1F, 0x41, 0xC3,
        ];

        assert_eq!(deserialize_message_1(&mut bytes).unwrap(), original);
    }
}
