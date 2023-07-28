// #[test]
// fn parse_header() {
//     let parser = DnsService {};

//     let msg = Header {
//         id: 1,
//         op_code: OpCode::Query,
//         authoritative_answer: false,
//         truncated: false,
//         recursion_desired: false,
//         recursion_available: false,
//         z: false,
//         response_code: ResponseCode::NoError,
//         questions: 1,
//         additionals: 0,
//         answers: 0,
//         authorities: 0,
//         message_type: MessageType::Query,
//     };

//     let bytes = msg.serialize();

//     let res = parser.parse(&bytes);

//     assert!(res.id == 1);
//     assert!(res.questions == 1);
//     assert!(res.answers == 0);
//     assert!(res.authorities == 0);
//     assert!(res.additionals == 0);
//     assert!(res.message_type == MessageType::Query);
// }
