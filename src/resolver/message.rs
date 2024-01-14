use rand::prelude::ThreadRng;
use rand::Rng;
use crate::resolver::header::Header;
use crate::resolver::{MessageType, OpCode, Question, ResourceRecord, ResponseCode, Serializable};

#[derive(Debug, PartialEq)]
pub enum DnsValidation {
    InvalidTotalLength,
    InvalidLabelLength,
    Valid,
}
#[derive(Clone)]
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
