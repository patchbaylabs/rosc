#![cfg_attr(not(feature = "std"), no_std)]

extern crate rosc;

use rosc::{decoder, encoder};
use rosc::{OscArray, OscBundle, OscColor, OscMessage, OscMidiMessage, OscPacket, OscType};

extern crate hex;

const SLIP_END: &str = "c0";

/// Reference SLIP (RFC1055) framer, written independently of `rosc::encoder`,
/// so tests can check the crate's SLIP output against something other than
/// itself.
#[cfg(feature = "std")]
fn slip_frame(data: &[u8]) -> Vec<u8> {
    let mut framed = vec![0xC0u8];
    for &b in data {
        match b {
            0xC0 => framed.extend_from_slice(&[0xDB, 0xDC]),
            0xDB => framed.extend_from_slice(&[0xDB, 0xDD]),
            b => framed.push(b),
        }
    }
    framed.push(0xC0);
    framed
}
const GOLDEN_MESSAGE_WO_ARGS: &str = "2f736f6d652f6164647200002c000000";
const GOLDEN_MESSAGE_WITH_ALL_TYPES: &str = "2f616e6f746865722f616464726573732f3100002c69686664737362746346544e496d725b695b64645d735d0000000000000004000000000000002a40490fda400921fb54442eea54686973206973206120737472696e672e00000054686973206973206120737472696e6720746f6f2e00000000000003010203000000007b000001c80000006304292a81ffc02a0d0000002a3ff3ae147ae147ae4009ae147ae147ae59617900";
const GOLDEN_EMPTY_BUNDLE: &str = "2362756e646c65000000000400000002";
const GOLDEN_BUNDLE: &str = "2362756e646c6500000004d2000010e10000000c2f766965772f31002c000000000000202f6d697865722f6368616e6e656c2f312f616d70000000002c6600003f666666000000442362756e646c65000000162e0000223d000000142f6f73632f312f66726571002c690000000001b8000000182f6f73632f312f7068617365000000002c660000becccccd";

#[cfg(feature = "std")]
#[test]
fn test_message_wo_args() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/some/addr".to_string(),
        args: vec![],
    });

    let bytes = encoder::encode(&packet).expect("encode failed");
    assert_eq!(hex::decode(GOLDEN_MESSAGE_WO_ARGS).unwrap(), bytes);

    let (tail, decoded_packet) = decoder::decode_udp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(packet, decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_message_wo_args_tcp() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/some/addr".to_string(),
        args: vec![],
    });

    let bytes = encoder::encode_tcp(&packet).expect("encode failed");
    assert_eq!(
        hex::decode(GOLDEN_MESSAGE_WO_ARGS).unwrap(),
        bytes[4..bytes.len()]
    );
    assert_eq!((bytes.len() as u32 - 4).to_be_bytes(), bytes[0..4]);
    let (tail, decoded_packet) = decoder::decode_tcp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(Some(packet), decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_message_wo_args_tcp_slip() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/some/addr".to_string(),
        args: vec![],
    });

    let bytes = encoder::encode_tcp_slip(&packet).expect("encode failed");
    assert_eq!(
        hex::decode(format!("{}{}{}", SLIP_END, GOLDEN_MESSAGE_WO_ARGS, SLIP_END).as_str())
            .unwrap(),
        bytes
    );
    assert_eq!(hex::decode(SLIP_END).unwrap(), bytes[0..1]);
    assert_eq!(
        hex::decode(SLIP_END).unwrap(),
        bytes[bytes.len() - 1..bytes.len()]
    );
    let (tail, decoded_packet) = decoder::decode_tcp_slip(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(Some(packet), decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_encode_message_with_all_types() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/another/address/1".to_string(),
        args: vec![
            4i32.into(),
            42i64.into(),
            3.1415926f32.into(),
            3.14159265359f64.into(),
            "This is a string.".to_string().into(),
            "This is a string too.".into(),
            vec![1u8, 2u8, 3u8].into(),
            (123, 456).into(),
            'c'.into(),
            false.into(),
            true.into(),
            OscType::Nil,
            OscType::Inf,
            OscMidiMessage {
                port: 4,
                status: 41,
                data1: 42,
                data2: 129,
            }
            .into(),
            OscColor {
                red: 255,
                green: 192,
                blue: 42,
                alpha: 13,
            }
            .into(),
            OscArray {
                content: vec![
                    42i32.into(),
                    OscArray {
                        content: vec![1.23.into(), 3.21.into()],
                    }
                    .into(),
                    "Yay".into(),
                ],
            }
            .into(),
        ],
    });

    let bytes = encoder::encode(&packet).expect("encode failed");
    assert_eq!(hex::decode(GOLDEN_MESSAGE_WITH_ALL_TYPES).unwrap(), bytes);

    let (tail, decoded_packet) = decoder::decode_udp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(packet, decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_encode_message_with_all_types_tcp() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/another/address/1".to_string(),
        args: vec![
            4i32.into(),
            42i64.into(),
            3.1415926f32.into(),
            3.14159265359f64.into(),
            "This is a string.".to_string().into(),
            "This is a string too.".into(),
            vec![1u8, 2u8, 3u8].into(),
            (123, 456).into(),
            'c'.into(),
            false.into(),
            true.into(),
            OscType::Nil,
            OscType::Inf,
            OscMidiMessage {
                port: 4,
                status: 41,
                data1: 42,
                data2: 129,
            }
            .into(),
            OscColor {
                red: 255,
                green: 192,
                blue: 42,
                alpha: 13,
            }
            .into(),
            OscArray {
                content: vec![
                    42i32.into(),
                    OscArray {
                        content: vec![1.23.into(), 3.21.into()],
                    }
                    .into(),
                    "Yay".into(),
                ],
            }
            .into(),
        ],
    });

    let bytes = encoder::encode_tcp(&packet).expect("encode failed");
    assert_eq!(
        hex::decode(GOLDEN_MESSAGE_WITH_ALL_TYPES).unwrap(),
        bytes[4..bytes.len()]
    );
    assert_eq!((bytes.len() as u32 - 4).to_be_bytes(), bytes[0..4]);

    let (tail, decoded_packet) = decoder::decode_tcp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(Some(packet), decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_encode_message_with_all_types_tcp_slip() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/another/address/1".to_string(),
        args: vec![
            4i32.into(),
            42i64.into(),
            3.1415926f32.into(),
            3.14159265359f64.into(),
            "This is a string.".to_string().into(),
            "This is a string too.".into(),
            vec![1u8, 2u8, 3u8].into(),
            (123, 456).into(),
            'c'.into(),
            false.into(),
            true.into(),
            OscType::Nil,
            OscType::Inf,
            OscMidiMessage {
                port: 4,
                status: 41,
                data1: 42,
                data2: 129,
            }
            .into(),
            OscColor {
                red: 255,
                green: 192,
                blue: 42,
                alpha: 13,
            }
            .into(),
            OscArray {
                content: vec![
                    42i32.into(),
                    OscArray {
                        content: vec![1.23.into(), 3.21.into()],
                    }
                    .into(),
                    "Yay".into(),
                ],
            }
            .into(),
        ],
    });

    let bytes = encoder::encode_tcp_slip(&packet).expect("encode failed");

    // GOLDEN_MESSAGE_WITH_ALL_TYPES contains a raw 0xC0 byte (OscColor's
    // green = 192), so the framed output isn't a plain END+payload+END —
    // that byte must come out escaped. Check against an independently
    // written reference SLIP framer rather than just round-tripping, so an
    // encode/decode bug that happens to cancel out wouldn't slip through.
    let golden = hex::decode(GOLDEN_MESSAGE_WITH_ALL_TYPES).unwrap();
    assert_eq!(slip_frame(&golden), bytes);

    assert_eq!(hex::decode(SLIP_END).unwrap(), bytes[0..1]);
    assert_eq!(
        hex::decode(SLIP_END).unwrap(),
        bytes[bytes.len() - 1..bytes.len()]
    );
    let (tail, decoded_packet) = decoder::decode_tcp_slip(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(Some(packet), decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_slip_escaping_esc_byte() {
    // A blob containing raw 0xDB (SLIP ESC) and 0xC0 (SLIP END) bytes forces
    // both escape cases the plain-byte tests above never exercise.
    let packet = OscPacket::Message(OscMessage {
        addr: "/blob".to_string(),
        args: vec![OscType::Blob(vec![0xDB, 0xC0, 0x01, 0x02])],
    });

    let payload = encoder::encode(&packet).expect("encode failed");
    let bytes = encoder::encode_tcp_slip(&packet).expect("encode failed");
    assert_eq!(slip_frame(&payload), bytes);

    let (tail, decoded_packet) = decoder::decode_tcp_slip(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(Some(packet), decoded_packet);
}

#[cfg(feature = "std")]
#[test]
fn test_decode_tcp_slip_multiple_packets_in_one_buffer() {
    let packet1 = OscPacket::Message(OscMessage {
        addr: "/first".to_string(),
        args: vec![1i32.into()],
    });
    let packet2 = OscPacket::Message(OscMessage {
        addr: "/second".to_string(),
        args: vec![2i32.into()],
    });

    let mut bytes = encoder::encode_tcp_slip(&packet1).expect("encode failed");
    bytes.extend(encoder::encode_tcp_slip(&packet2).expect("encode failed"));

    let (remainder, decoded1) = decoder::decode_tcp_slip(&bytes).expect("decode failed");
    assert_eq!(Some(packet1), decoded1);
    assert!(!remainder.is_empty());

    let (remainder, decoded2) = decoder::decode_tcp_slip(remainder).expect("decode failed");
    assert_eq!(Some(packet2), decoded2);
    assert_eq!(0, remainder.len());
}

#[cfg(feature = "std")]
#[test]
fn test_decode_tcp_slip_waits_for_more_data_without_terminating_end() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/some/addr".to_string(),
        args: vec![],
    });
    let bytes = encoder::encode_tcp_slip(&packet).expect("encode failed");

    // Drop the closing END, simulating a TCP read that stopped mid-packet.
    let partial = &bytes[..bytes.len() - 1];
    let (remainder, decoded) = decoder::decode_tcp_slip(partial).expect("decode failed");
    assert_eq!(None, decoded);
    assert_eq!(partial, remainder);
}

#[cfg(feature = "std")]
#[test]
fn test_decode_tcp_slip_waits_for_more_data_mid_escape() {
    let packet = OscPacket::Message(OscMessage {
        addr: "/color".to_string(),
        args: vec![OscColor {
            red: 1,
            green: 192, // 0xC0, forces an ESC/ESC_END pair in the SLIP stream
            blue: 3,
            alpha: 4,
        }
        .into()],
    });
    let bytes = encoder::encode_tcp_slip(&packet).expect("encode failed");

    let esc_idx = bytes
        .iter()
        .position(|&b| b == 0xDB)
        .expect("expected an escaped byte in the stream");

    // Cut the buffer right after the ESC byte, before its completion arrives.
    let partial = &bytes[..=esc_idx];
    let (remainder, decoded) = decoder::decode_tcp_slip(partial).expect("decode failed");
    assert_eq!(None, decoded);
    assert_eq!(partial, remainder);
}

#[cfg(feature = "std")]
#[test]
fn test_decode_tcp_slip_short_buffers_do_not_panic() {
    // Regression test: buffers shorter than 8 bytes used to underflow
    // `msg.len() - 8` inside decode_tcp_slip and panic (or, in release
    // builds, wrap around into bogus logic).
    for len in 0..8 {
        let mut msg = vec![0xC0u8];
        msg.extend(std::iter::repeat(0x01u8).take(len));
        let (remainder, decoded) = decoder::decode_tcp_slip(&msg).expect("decode failed");
        assert_eq!(None, decoded);
        assert_eq!(msg, remainder);
    }
}

#[cfg(feature = "std")]
#[test]
fn test_empty_bundle() {
    let packet = OscPacket::Bundle(OscBundle {
        timetag: (4, 2).into(),
        content: vec![],
    });

    let bytes = encoder::encode(&packet).expect("encode failed");
    assert_eq!(hex::decode(GOLDEN_EMPTY_BUNDLE).unwrap(), bytes);

    let (tail, decoded_packet) = decoder::decode_udp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(packet, decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_bundle() {
    let packet = OscPacket::Bundle(OscBundle {
        timetag: (1234, 4321).into(),
        content: vec![
            OscPacket::Message(OscMessage {
                addr: "/view/1".to_string(),
                args: vec![],
            }),
            OscPacket::Message(OscMessage {
                addr: "/mixer/channel/1/amp".to_string(),
                args: vec![0.9f32.into()],
            }),
            OscPacket::Bundle(OscBundle {
                timetag: (5678, 8765).into(),
                content: vec![
                    OscPacket::Message(OscMessage {
                        addr: "/osc/1/freq".to_string(),
                        args: vec![440i32.into()],
                    }),
                    OscPacket::Message(OscMessage {
                        addr: "/osc/1/phase".to_string(),
                        args: vec![(-0.4f32).into()],
                    }),
                ],
            }),
        ],
    });

    let bytes = encoder::encode(&packet).expect("encode failed");
    assert_eq!(hex::decode(GOLDEN_BUNDLE).unwrap(), bytes);

    let (tail, decoded_packet) = decoder::decode_udp(&bytes).expect("decode failed");
    assert_eq!(0, tail.len());
    assert_eq!(packet, decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_bundle_cursor() {
    let packet = OscPacket::Bundle(OscBundle {
        timetag: (1234, 4321).into(),
        content: vec![
            OscPacket::Message(OscMessage {
                addr: "/view/1".to_string(),
                args: vec![],
            }),
            OscPacket::Message(OscMessage {
                addr: "/mixer/channel/1/amp".to_string(),
                args: vec![0.9f32.into()],
            }),
            OscPacket::Bundle(OscBundle {
                timetag: (5678, 8765).into(),
                content: vec![
                    OscPacket::Message(OscMessage {
                        addr: "/osc/1/freq".to_string(),
                        args: vec![440i32.into()],
                    }),
                    OscPacket::Message(OscMessage {
                        addr: "/osc/1/phase".to_string(),
                        args: vec![(-0.4f32).into()],
                    }),
                ],
            }),
        ],
    });

    let mut bytes = Vec::new();
    let n = encoder::encode_into(
        &packet,
        &mut encoder::WriteOutput(std::io::Cursor::new(&mut bytes)),
    )
    .expect("encode failed");
    assert_eq!(140, n);
    assert_eq!(hex::decode(GOLDEN_BUNDLE).unwrap(), bytes);
}

#[cfg(feature = "std")]
#[test]
fn test_decode_encoded_message_four_byte_aligned_blob() {
    let message = OscPacket::Message(OscMessage {
        addr: "/view/1".to_string(),
        args: vec![OscType::Blob(vec![1, 2, 3, 4])],
    });
    let encoded_message = encoder::encode(&message).unwrap();
    let (_, decoded_packet) = decoder::decode_udp(&encoded_message).unwrap();
    assert_eq!(message, decoded_packet)
}

#[cfg(feature = "std")]
#[test]
fn test_decode_encoded_message_four_byte_aligned_blob_and_string() {
    let message = OscPacket::Message(OscMessage {
        addr: "/view/12".to_string(),
        args: vec![OscType::Blob(vec![1, 2, 3, 4])],
    });
    let encoded_message = encoder::encode(&message).unwrap();
    let (_, decoded_packet) = decoder::decode_udp(&encoded_message).unwrap();
    assert_eq!(message, decoded_packet)
}
