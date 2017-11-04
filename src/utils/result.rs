use std::fmt::Display;

pub fn un_warp_result<T, E: Display>(x: Result<T, E>) -> T {
    match x {
        Ok(x) => {
            return x
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}