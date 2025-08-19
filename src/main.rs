use std::collections::HashMap;

#[derive(Debug)]
enum PacketType {
    TCP { scr_port: u16, dst_port: u16 },
    UDP { scr_port: u16, dst_port: u16 },
    Unknown,
}

#[derive(Debug)]
struct Packet {
    src_ip: String,
    dst_ip: String,
    packet_type: PacketType,
    payload: Vec<u8>,
}

impl Packet {
    fn summary(&self) {
        println!("source ip : {}", self.src_ip);
        println!("destination ip : {}", self.dst_ip);
        println!("packet type : {:?}", self.packet_type);
        println!("payload : {:#?}", self.payload);
    }

    fn size(&self) {
        println!("size of payload : {}", self.payload.len());
    }

    fn is_suspicious(&self) -> bool {
        if self.payload.is_empty() {
            return true;
        } else if self.payload.len() > 10000 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
struct Connection {
    src_ip: String,
    dst_ip: String,
    src_port: u16,
    dst_port: u16,
    protocol: String,
    packet_count: u64,
    bytes_transferred: u64,
}

impl Connection {
    fn new(packet: &Packet) -> Self {
        let result = match packet.packet_type {
            PacketType::TCP { scr_port, dst_port } => ("TCP".to_string(), (scr_port, dst_port)),
            PacketType::UDP { scr_port, dst_port } => ("UDP".to_string(), (scr_port, dst_port)),
            PacketType::Unknown => ("Unknown".to_string(), (0, 0)),
        };

        Connection {
            src_ip: packet.src_ip.clone(),
            dst_ip: packet.dst_ip.clone(),
            src_port: result.1 .0,
            dst_port: result.1 .1,
            protocol: result.0,
            packet_count: 0,
            bytes_transferred: 0,
        }
    }

    // modify inplace instaed of rebuilding
    fn update(&self, packet: &Packet) -> Self {
        Connection {
            src_ip: self.src_ip.clone(),
            dst_ip: self.dst_ip.clone(),
            src_port: self.src_port,
            dst_port: self.dst_port,
            protocol: self.protocol.clone(),
            packet_count: self.packet_count + 1,
            bytes_transferred: self.bytes_transferred + packet.payload.len() as u64,
        }
    }
}

#[derive(Debug)]
struct AnalyzeState {
    connections: HashMap<String, Connection>,
    events: Vec<String>,
}

impl AnalyzeState {
    fn add_or_update_connection(&mut self, packet: &Packet) {
        let the_key = format!(
            "{}{}-{:?}",
            packet.src_ip, packet.dst_ip, packet.packet_type
        );

        if !packet.is_suspicious() == true {
            if let Some(got_connection) = &self.connections.get(&the_key) {
                self.connections
                    .insert(the_key, Connection::update(&got_connection, packet));
                println!("updated connection is : {:#?}", self.connections);
            } else {
                self.connections.insert(the_key, Connection::new(packet));
                println!("new connection is : {:#?}", self.connections);
            }
        } else {
            self.events.push(the_key);
        }
    }
}

fn main() {
    let packet = Packet {
        src_ip: "127.0.0.1".to_string(),
        dst_ip: "0.0.0.0".to_string(),
        packet_type: PacketType::TCP {
            scr_port: 80,
            dst_port: 3000,
        },
        payload: vec![10, 20, 20],
    };

    let connections = Connection {
        src_ip: "127.0.0.1".to_string(),
        dst_ip: "192.168.0.10".to_string(),
        src_port: 50000,
        dst_port: 80,
        protocol: "TCP".to_string(),
        packet_count: 12,
        bytes_transferred: 8421,
    };

    packet.summary();
    let suspicion = packet.is_suspicious();
    println!("Is Suspecied : {}", suspicion);

    let mut analyzestate = AnalyzeState {
        connections: HashMap::new(),
        events: vec![],
    };
    analyzestate.add_or_update_connection(&packet);
    analyzestate.add_or_update_connection(&packet);
}
