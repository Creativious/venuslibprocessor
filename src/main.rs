use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use json::JsonValue;

struct JavaConnection {
    socket_addr: SocketAddr,
    id: u32,
}

struct Packet {
    packet_type: PacketTypes,
    packet_data: JsonValue,
}

impl Packet {
    fn new(packet_type: PacketTypes, packet_data: JsonValue) -> Packet {
        Packet {
            packet_type,
            packet_data,
        }
    }
}

#[derive(PartialEq)]
enum PacketTypes {
    C2SNewConnection,
    S2CAcceptedNewConnection,
}

impl JavaConnection {
    fn new(socket_addr: SocketAddr, id: u32) -> JavaConnection {
        JavaConnection {
            socket_addr,
            id,
        }
    }

    fn handle_packet(packet: Packet) {
        match packet.packet_type {
            PacketTypes::C2SNewConnection => {
                let packet = Packet::new(PacketTypes::S2CAcceptedNewConnection, json::object!{
                    "id" => 0,
                });
            },
            _ => panic!("Unknown packet type"),
        }
    }
}
fn main() -> std::io::Result<()> {
    println!("Starting socket");

    let socket_addr = "127.0.0.1";
    let socket_port = "3201";
    let socket = UdpSocket::bind(format!("{}:{}", socket_addr, socket_port)).expect("Couldn't bind to address");
    const BUF_MAX_SIZE: usize = 1024;
    let mut buf = [0; BUF_MAX_SIZE];
    let mut last_id = 0;
    let mut connections = HashMap::new();
    loop {
        let result = socket.recv_from(&mut buf);
        if result.is_ok() {
            let (amt, src) = result.unwrap();
            let buf = &mut buf[..amt];
            let packet = convert_packet_bytes_to_packet(buf);
            let connection;
            if packet.packet_type == PacketTypes::C2SNewConnection {
                connections.insert(last_id, JavaConnection::new(src, last_id));
                last_id += 1;
                connection = connections.get(&last_id).unwrap();
            }
            else {
                let id = packet.packet_data["id"].as_u32().unwrap();
                connection = connections.get(&id).unwrap();
            }
        } else {
            println!("Error receiving message");
        }

    }

}

fn convert_packet_bytes_to_packet(packet_bytes: &[u8]) -> Packet {
    let packet_raw_string = String::from_utf8_lossy(packet_bytes);
    let mut message_data = json::parse(&packet_raw_string).unwrap();
    let message = message_data["message_type"].to_string();
    message_data.remove("message_type");
    let message = match message.as_str() {
        "C2SNewConnection" => PacketTypes::C2SNewConnection,
        "S2CAcceptedNewConnection" => PacketTypes::S2CAcceptedNewConnection,
        _ => panic!("Unknown message"),
    };
    Packet::new(message, message_data)
}