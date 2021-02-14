use serde_resp::ser;
use serde_resp::RESPType;

#[test]
fn basic_test() {
    let obj = RESPType::SimpleString("magic".to_owned());
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("+magic\r\n".to_owned(), serialized);
}
