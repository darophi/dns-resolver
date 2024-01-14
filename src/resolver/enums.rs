use std::fmt::Display;

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
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
