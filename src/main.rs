use std::net::{Ipv4Addr, UdpSocket};

use dns_resolver::{
    DnsParser, Header, Message, MessageType, OpCode, Question, ResponseCode, ToBytes,
};

const TYPE_A: u16 = 1;
const TYPE_CNAME: u16 = 5;
const TYPE_NS: u16 = 2;
const UDP_BYTE_SIZE_RESTRICTION: usize = 512;
const CLASS_IN: u16 = 1;

fn main() -> std::io::Result<()> {
    // Udp client
    let google = "8.8.8.8:53";
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).expect("Could not bind client");

    socket.connect(google).expect("Could not connect to google");
    let message = Message {
        header: Header {
            id: 0,
            message_type: MessageType::Query,
            op_code: OpCode::Query,
            authoritative_answer: false,
            truncated: false,
            recursion_desired: true,
            recursion_available: false,
            response_code: ResponseCode::NoError,
            z: false,
            questions: 1,
            answers: 0,
            authorities: 0,
            additionals: 0,
        },
        questions: vec![Question::new(
            encode_dns_name("google.com"),
            CLASS_IN,
            TYPE_A,
        )],
        answers: vec![],
    };

    let query = message.to_bytes();
    let result = socket.send(query.as_slice())?;
    println!("ResultSize: {}", result);

    let mut buf = [0u8; UDP_BYTE_SIZE_RESTRICTION];
    socket.recv_from(&mut buf).unwrap();

    let parser = DnsParser {};

    let msg = parser.parse_message(buf.to_vec());

    println!("Message: {:?}", msg.answers[0]);

    Ok(())
}
fn encode_dns_name(name: &str) -> String {
    let mut result = String::new();
    for part in name.split('.') {
        result.push(part.len() as u8 as char);
        result.push_str(part);
    }
    result.push('\0');
    result
}
