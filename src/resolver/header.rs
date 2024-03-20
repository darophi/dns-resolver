use crate::resolver::{MessageType, OpCode, ResponseCode, Serializable};
use std::fmt::Display;

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
        let qr_flag = match self.message_type {
            MessageType::Query => 0,
            MessageType::Response => 1,
        };

        let opcode = match self.op_code {
            OpCode::Query => 0,
            OpCode::IQuery => 1,
            OpCode::Status => 2,
            OpCode::Reserved => 3,
        };

        let recursion_desired = if self.recursion_desired { 1 } else { 0 };
        let recursion_available = if self.recursion_available { 1 } else { 0 };
        let authoritative_answer = if self.authoritative_answer { 1 } else { 0 };
        let truncated = if self.truncated { 1 } else { 0 };
        let z = if self.z { 1 } else { 0 };
        let response_code = match self.response_code {
            ResponseCode::NoError => 0,
            ResponseCode::FormatError => 1,
            ResponseCode::ServerFailure => 2,
            ResponseCode::NameError => 3,
            ResponseCode::NotImplemented => 4,
            ResponseCode::Refused => 5,
            ResponseCode::YxDomain => 6,
            ResponseCode::XrRSet => 7,
            ResponseCode::NotAuth => 8,
            ResponseCode::NotZone => 9,
        };

        let header_second_row: u16 = qr_flag << 15
            | opcode << 11
            | recursion_desired << 8
            | recursion_available << 7
            | authoritative_answer << 10
            | truncated << 9
            | z << 6
            | response_code;

        [
            self.id.to_be_bytes(),
            header_second_row.to_be_bytes(),
            self.questions.to_be_bytes(),
            self.answers.to_be_bytes(),
            self.authorities.to_be_bytes(),
            self.additionals.to_be_bytes(),
        ]
        .concat()
    }
}
