use crate::resolver::header::Header;
use crate::resolver::{MessageType, OpCode, Question, ResourceRecord, ResponseCode, Serializable};
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum DnsValidation {
    InvalidTotalLength,
    InvalidLabelLength,
    Valid,
}
#[derive(Clone, Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
}

const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;

impl Serializable for Message {
    fn serialize(&self) -> Vec<u8> {
        let mut query = self.header.serialize();
        for question in &self.questions {
            query.extend(question.serialize());
        }
        query
    }
}

impl From<Vec<u8>> for Message {
    fn from(data: Vec<u8>) -> Self {
        let id = data[0] as u16;
        let second_row = data[1] as u16;
        let qr = second_row >> 15;
        let opcode = (second_row >> 11) & 0b1111;
        let aa = (second_row >> 10) & 0b1;
        let tc = (second_row >> 9) & 0b1;
        let rd = (second_row >> 8) & 0b1;
        let ra = (second_row >> 7) & 0b1;
        let z = (second_row >> 6) & 0b1;
        let rcode = second_row & 0b1111;

        let questions = u16::from_be_bytes(data[4..6].try_into().unwrap());
        let answers = u16::from_be_bytes(data[6..8].try_into().unwrap());
        let authorities = u16::from_be_bytes(data[8..10].try_into().unwrap());
        let additionals = u16::from_be_bytes(data[10..12].try_into().unwrap());

        Self {
            header: Header {
                id: id,
                message_type: qr.into(),
                op_code: opcode.into(),
                authoritative_answer: aa == 1,
                truncated: tc == 1,
                recursion_desired: rd == 1,
                recursion_available: ra == 1,
                response_code: rcode.into(),
                z: z == 1,
                questions,
                answers,
                authorities,
                additionals,
            },
            questions: vec![],
            answers: vec![],
        }
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

fn encode_dns_name(name: &str) -> String {
    let mut result = String::new();
    for part in name.split('.') {
        result.push(part.len() as u8 as char);
        result.push_str(part);
    }
    result.push('\0');
    result
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_dns_name() {
        let name = "google.com";
        let encoded = encode_dns_name(name);
        assert_eq!(encoded, "\x06google\x03com\x00");
    }

    #[test]
    fn test_validate_dns_name_should_succeed() {
        let name = "google.com";
        let validation = validate_dns_name(&name.to_string());

        assert_eq!(validation, DnsValidation::Valid);
    }

    #[test]
    fn test_validate_dns_name_should_fail_on_dns_with_length_greater_than_253() {
        let name = "google.com".repeat(100);
        let validation = validate_dns_name(&name.to_string());

        assert_eq!(validation, DnsValidation::InvalidTotalLength);
    }
}
