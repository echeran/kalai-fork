use crate::kalai::kalai;
use crate::kalai::kalai::*;
pub fn main() {
    let _args: std::vec::Vec<String> = std::env::args().collect();
    {
        let a: kalai::BValue = kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(1i64))
                .push_back(kalai::BValue::from(2i64)),
        );
        let b: kalai::BValue = kalai::BValue::from(
            rpds::Vector::new()
                .push_back(kalai::BValue::from(1i64))
                .push_back(kalai::BValue::from(3i64)),
        );
        let v: kalai::BValue = crate::data_diff::core::diff(a.clone(), b.clone());
        println!("{}", v.clone());
    }
}
