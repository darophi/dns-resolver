use std::{fmt::Display, net::Ipv4Addr};

pub struct DnsParser {}

impl DnsParser {
    pub fn parse_message(&self, buffer: Vec<u8>) -> Message {
        let header = self.parse(&buffer);
        let mut questions = Vec::new();

        let mut offset = 12;
        for _ in 0..header.questions {
            let question = self.parse_question(&buffer, &mut offset);
            questions.push(question);
        }

        let mut answers = Vec::new();

        for _ in 0..header.answers {
            let answer = self.parse_answer(&buffer, &mut offset);
            answers.push(answer);
        }

        Message {
            header,
            questions,
            answers,
        }
    }

    fn parse_domain(&self, buffer: &Vec<u8>, offset: &mut i32) -> String {
        *offset += 2;

        let mut dns_offset = buffer[*offset as usize] as i32;
        // let w = buffer[*offset as usize] as u16 & 0b11000000_00000000;

        if dns_offset == 0 {
            return String::from("");
        }

        return self.parse_name(buffer, &mut dns_offset);
    }

    fn parse_name(&self, buffer: &Vec<u8>, offset: &mut i32) -> String {
        let mut name = String::new();

        loop {
            let length = buffer[*offset as usize] as u8;
            if length == 0 {
                break;
            }
            let from = *offset as usize + 1;
            let to = *offset as usize + 1 + length as usize;

            name.push_str(std::str::from_utf8(&buffer[from..to]).unwrap());
            name.push('.');
            *offset += length as i32 + 1;
        }

        println!("i: {}", *offset);

        name
    }
    fn read_next_u16(&self, buffer: &Vec<u8>, offset: &mut i32) -> u16 {
        let res = u16::from_be_bytes(
            buffer[*offset as usize + 1..*offset as usize + 3]
                .try_into()
                .unwrap(),
        );
        *offset += 2;
        res
    }

    fn read_next_u32(&self, buffer: &Vec<u8>, offset: &mut i32) -> u32 {
        let res = u32::from_be_bytes(
            buffer[*offset as usize + 1..*offset as usize + 5]
                .try_into()
                .unwrap(),
        );
        *offset += 4;
        res
    }

    fn parse_answer(&self, buffer: &Vec<u8>, offset: &mut i32) -> ResourceRecord {
        let name = self.parse_domain(buffer, offset);

        let rtype = self.read_next_u16(buffer, offset);
        let rclass = self.read_next_u16(buffer, offset);

        let ttl = self.read_next_u32(buffer, offset);

        let rdlength = self.read_next_u16(buffer, offset);

        let rdata_from = *offset as usize + 1;
        let rdata_to = *offset as usize + 1 + rdlength as usize;
        let rdata = buffer[rdata_from..rdata_to].to_vec();

        let ip = Ipv4Addr::new(rdata[0], rdata[1], rdata[2], rdata[3]);
        ResourceRecord {
            name,
            rtype,
            rclass,
            ttl,
            rdlength,
            rdata: RData::A(ip),
        }
    }

    fn parse_question(&self, buffer: &Vec<u8>, offset: &mut i32) -> Question {
        let name = self.parse_name(buffer, offset);

        let qtype = self.read_next_u16(buffer, offset);
        let qclass = self.read_next_u16(buffer, offset);

        Question {
            name,
            qtype,
            qclass,
        }
    }

    fn parse(&self, buffer: &Vec<u8>) -> Header {
        let id: u16 = u16::from_be_bytes(buffer[0..2].try_into().unwrap());
        let flags = u16::from_be_bytes(buffer[2..4].try_into().unwrap());

        let opcode = (flags >> 11) & 0b1111;
        let authoritative_answer = (flags >> 10) & 0b1;
        let truncated = (flags >> 9) & 0b1;
        let recursion_desired = (flags >> 8) & 0b1;
        let recursion_available = (flags >> 7) & 0b1;
        let z = (flags >> 6) & 0b1;
        let response_code = flags & 0b1111;

        let questions = u16::from_be_bytes(buffer[4..6].try_into().unwrap());
        let answers = u16::from_be_bytes(buffer[6..8].try_into().unwrap());
        let authorities = u16::from_be_bytes(buffer[8..10].try_into().unwrap());
        let additionals = u16::from_be_bytes(buffer[10..12].try_into().unwrap());

        let qr_flag = flags >> 15;
        let message_type = match qr_flag {
            0 => MessageType::Query,
            1 => MessageType::Response,
            _ => panic!("Invalid message type: {qr_flag}"),
        };

        Header {
            id,
            message_type,
            op_code: match opcode {
                0 => OpCode::Query,
                1 => OpCode::IQuery,
                2 => OpCode::Status,
                3 => OpCode::Reserved,
                _ => panic!("Invalid opcode: {opcode}"),
            },
            recursion_desired: recursion_desired == 1,
            recursion_available: recursion_available == 1,
            authoritative_answer: authoritative_answer == 1,
            truncated: truncated == 1,
            z: z == 1,
            response_code: match response_code {
                0 => ResponseCode::NoError,
                1 => ResponseCode::FormatError,
                2 => ResponseCode::ServerFailure,
                3 => ResponseCode::NameError,
                4 => ResponseCode::NotImplemented,
                5 => ResponseCode::Refused,
                _ => panic!("Invalid response code: {response_code}"),
            },
            questions,
            answers,
            authorities,
            additionals,
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub message_type: MessageType,
    pub op_code: OpCode,
    pub authoritative_answer: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,
    pub z: bool,
    pub response_code: ResponseCode,
    pub questions: u16,
    pub answers: u16,
    pub authorities: u16,
    pub additionals: u16,
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
}

impl ToBytes for Message {
    fn to_bytes(&self) -> Vec<u8> {
        let mut query = self.header.to_bytes();
        for question in &self.questions {
            query.extend(question.to_bytes());
        }
        query
    }
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Query => write!(f, "Query"),
            MessageType::Response => write!(f, "Response"),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Query => write!(f, "Query"),
            OpCode::IQuery => write!(f, "IQuery"),
            OpCode::Status => write!(f, "Status"),
            OpCode::Reserved => write!(f, "Reserved"),
        }
    }
}

impl Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseCode::NoError => write!(f, "NoError"),
            ResponseCode::FormatError => write!(f, "FormatError"),
            ResponseCode::ServerFailure => write!(f, "ServerFailure"),
            ResponseCode::NameError => write!(f, "NameError"),
            ResponseCode::NotImplemented => write!(f, "NotImplemented"),
            ResponseCode::Refused => write!(f, "Refused"),
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Header: id: {id}, message_type: {message_type}, op_code: {op_code}, authoritative_answer: {authoritative_answer}, truncated: {truncated}, recursion_desired: {recursion_desired}, recursion_available: {recursion_available}, z: {z}, response_code: {response_code}, questions: {questions}, answers: {answers}, authorities: {authorities}, additionals: {additionals}",
            id = self.id,
            message_type = self.message_type,
            op_code = self.op_code,
            authoritative_answer = self.authoritative_answer,
            truncated = self.truncated,
            recursion_desired = self.recursion_desired,
            recursion_available = self.recursion_available,
            z = self.z,
            response_code = self.response_code,
            questions = self.questions,
            answers = self.answers,
            authorities = self.authorities,
            additionals = self.additionals
        )
    }
}

impl ToBytes for Header {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            0u16.to_be_bytes(),
            self.questions.to_be_bytes(),
            self.answers.to_be_bytes(),
            self.authorities.to_be_bytes(),
            self.additionals.to_be_bytes(),
        ]
        .concat()
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageType {
    Query,
    Response,
}

#[derive(Debug)]
pub enum OpCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Reserved = 3,
}

#[derive(Debug)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    NONE = 254,
    ANY = 255,
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

#[derive(Debug)]
pub struct Question {
    name: String,
    qtype: u16,
    qclass: u16,
}

impl Question {
    pub fn new(name: String, qtype: u16, qclass: u16) -> Self {
        Question {
            name,
            qtype,
            qclass,
        }
    }
}

#[derive(Debug)]
pub struct ResourceRecord {
    name: String,
    rtype: u16,
    rclass: u16,
    ttl: u32,
    rdlength: u16,
    pub rdata: RData,
}

#[derive(Debug)]
pub enum RData {
    A(Ipv4Addr),
    CNAME(String),
    NS(String),
}

impl ToBytes for Question {
    fn to_bytes(&self) -> Vec<u8> {
        [
            self.name.clone().into_bytes(),
            self.qtype.to_be_bytes().to_vec(),
            self.qclass.to_be_bytes().to_vec(),
        ]
        .concat()
    }
}

#[test]
fn parse_header() {
    let parser = DnsParser {};

    let msg = Header {
        id: 1,
        op_code: OpCode::Query,
        authoritative_answer: false,
        truncated: false,
        recursion_desired: false,
        recursion_available: false,
        z: false,
        response_code: ResponseCode::NoError,
        questions: 1,
        additionals: 0,
        answers: 0,
        authorities: 0,
        message_type: MessageType::Query,
    };

    let bytes = msg.to_bytes();

    let res = parser.parse(&bytes);

    assert!(res.id == 1);
    assert!(res.questions == 1);
    assert!(res.answers == 0);
    assert!(res.authorities == 0);
    assert!(res.additionals == 0);
    assert!(res.message_type == MessageType::Query);
}
