use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum MessageType {
    Query,
    Response,
}

impl From<u16> for MessageType {
    fn from(value: u16) -> Self {
        match value {
            0 => MessageType::Query,
            1 => MessageType::Response,
            _ => panic!("Invalid value for MessageType"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OpCode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Reserved = 3,
}

impl From<u16> for OpCode {
    fn from(value: u16) -> Self {
        match value {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            3 => OpCode::Reserved,
            _ => panic!("Invalid value for OpCode"),
        }
    }
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
    YxDomain = 6,
    XrRSet = 7,
    NotAuth = 8,
    NotZone = 9,
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

impl From<u16> for ResponseCode {
    fn from(value: u16) -> Self {
        match value {
            0 => ResponseCode::NoError,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => panic!("Invalid value for ResponseCode: {value}"),
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
            ResponseCode::YxDomain => write!(f, "YxDomain"),
            ResponseCode::XrRSet => write!(f, "XrRSet"),
            ResponseCode::NotAuth => write!(f, "NotAuth"),
            ResponseCode::NotZone => write!(f, "NotZone"),
        }
    }
}
