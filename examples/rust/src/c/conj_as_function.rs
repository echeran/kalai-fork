use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn test_map() -> i64 {
    let a: kalai::BValue = kalai::BValue::from(rpds::HashTrieMap::new().insert(
        kalai::BValue::from(String::from(":a")),
        kalai::BValue::from(1i64),
    ));
    let b: kalai::BValue = kalai::BValue::from(rpds::HashTrieMap::new().insert(
        kalai::BValue::from(String::from(":b")),
        kalai::BValue::from(2i64),
    ));
    let c: kalai::BValue = conj(a, b);
    return 3i64;
}
pub fn type_conversions() -> i64 {
    let a: rpds::HashTrieMap<kalai::BValue, kalai::BValue> = rpds::HashTrieMap::new().insert(
        kalai::BValue::from(String::from(":a")),
        kalai::BValue::from(1i64),
    );
    let b: rpds::HashTrieMap<kalai::BValue, kalai::BValue> = rpds::HashTrieMap::new().insert(
        kalai::BValue::from(String::from(":b")),
        kalai::BValue::from(1i64),
    );
    {
        conj(kalai::BValue::from(a), kalai::BValue::from(b));
        return 4i64;
    }
}
pub fn main() {
    let _args: std::vec::Vec<String> = std::env::args().collect();
    {
        println!("{}", test_map());
        println!("{}", type_conversions());
    }
}
