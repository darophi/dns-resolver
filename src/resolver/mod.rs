use std::{
    fmt::Display,
    net::{Ipv4Addr, UdpSocket},
};

use rand::{rngs::ThreadRng, Rng};

use crate::dns_cache::DnsCache;

const GOOGLE_DNS: &str = "8.8.8.8";
const DNS_PORT: u16 = 53;
pub const DEFAULT_DNS: (&str, u16) = (GOOGLE_DNS, DNS_PORT);

const TYPE_A: u16 = 1;
const TYPE_CNAME: u16 = 5;
const TYPE_NS: u16 = 2;
const UDP_BYTE_SIZE_RESTRICTION: usize = 512;
const CLASS_IN: u16 = 1;

pub struct DnsService {
    dns_cache: DnsCache,
    udp_socket: UdpSocket,
}

#[derive(Clone)]
pub struct DnsQuery {
    pub dns_name: String,
    pub message: Message,
}

pub enum DnsQueryError {
    FailedToSend,
}

impl Display for DnsQueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsQueryError::FailedToSend => write!(f, "Failed to send query"),
        }
    }
}

impl DnsService {
    pub fn new(udp_socket: UdpSocket, ip_cache: DnsCache) -> DnsService {
        DnsService {
            dns_cache: ip_cache,
            udp_socket,
        }
    }

    pub fn send_query(&mut self, dns_name: String) -> Result<Message, DnsQueryError> {
        match self.dns_cache.get(&dns_name) {
            res => {
                if res.is_some() {
                    return Ok(res.unwrap().message);
                }
            }
        }

        let message = match create_query(&dns_name) {
            Ok(message) => message,
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(DnsQueryError::FailedToSend);
            }
        };

        let query = message.serialize();

        self.udp_socket
            .send(query.as_slice())
            .expect("failed to send message");
        let bytes_to_send = query.len();

        println!("Sending {} bytes", bytes_to_send);

        let mut buf = [0u8; UDP_BYTE_SIZE_RESTRICTION];
        self.udp_socket.recv_from(&mut buf).unwrap();

        let msg = self.parse_message(buf.to_vec());

        if msg.answers.len() > 0 {
            let dns_name = dns_name.clone();

            let cache_key = dns_name.clone();
            self.dns_cache.set(
                cache_key.as_str(),
                &DnsQuery {
                    dns_name: cache_key.to_string(),
                    message: msg.clone(),
                },
            );
            return Ok(msg);
        }

        Ok(msg)
    }

    fn parse_message(&mut self, buffer: Vec<u8>) -> Message {
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

#[derive(Debug, Clone)]
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

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

#[derive(Clone)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
}

impl Serializable for Message {
    fn serialize(&self) -> Vec<u8> {
        let mut query = self.header.serialize();
        for question in &self.questions {
            query.extend(question.serialize());
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

impl Serializable for Header {
    fn serialize(&self) -> Vec<u8> {
        [
            self.id.to_be_bytes(),
            0u16.to_be_bytes(), // Flags will not be set for now
            self.questions.to_be_bytes(),
            self.answers.to_be_bytes(),
            self.authorities.to_be_bytes(),
            self.additionals.to_be_bytes(),
        ]
        .concat()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageType {
    Query,
    Response,
}

#[derive(Debug, Clone)]
pub enum OpCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Reserved = 3,
}

#[derive(Debug, Clone)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
    NONE = 254,
    ANY = 255,
}

#[derive(Debug, Clone)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct ResourceRecord {
    name: String,
    rtype: u16,
    rclass: u16,
    pub ttl: u32,
    rdlength: u16,
    pub rdata: RData,
}

#[derive(Debug, Clone)]
pub enum RData {
    A(Ipv4Addr),
    CNAME(String),
    NS(String),
    HInfo { cpu: String, os: String },
}

impl Serializable for Question {
    fn serialize(&self) -> Vec<u8> {
        [
            self.name.clone().into_bytes(),
            self.qtype.to_be_bytes().to_vec(),
            self.qclass.to_be_bytes().to_vec(),
        ]
        .concat()
    }
}

pub fn create_query(dns_name: &String) -> Result<Message, DnsValidation> {
    let mut rng: ThreadRng = rand::thread_rng();

    let dns_validation = validate_dns_name(dns_name);

    if dns_validation != DnsValidation::Valid {
        return Err(dns_validation);
    }

    let encoded_dns_name = encode_dns_name(&dns_name);
    let message = Message {
        header: Header {
            id: rng.gen(),
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
        questions: vec![Question::new(encoded_dns_name.clone(), CLASS_IN, TYPE_A)],
        answers: vec![],
    };

    Ok(message)
}

fn validate_dns_name(dns_name: &String) -> DnsValidation {
    let validate_total_length = dns_name.len() <= 253;
    if !validate_total_length {
        return DnsValidation::InvalidTotalLength;
    }

    let validate_label_length = dns_name.split('.').all(|part| part.len() <= 63);
    if !validate_label_length {
        return DnsValidation::InvalidLabelLength;
    }

    DnsValidation::Valid
}

#[derive(Debug, PartialEq)]
pub enum DnsValidation {
    InvalidTotalLength,
    InvalidLabelLength,
    Valid,
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
