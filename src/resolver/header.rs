use std::fmt::Display;
use crate::resolver::{MessageType, OpCode, ResponseCode, Serializable};

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
