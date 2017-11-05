use bson::Document;

pub fn get_docment_number_field(d: &Document, y: &'static str) -> i64 {
    let mut ret: i64 = 0;
    match d.get_f64(y) {
        Ok(v) => ret = v as i64,
        Err(_) => { },
    }

    match d.get_i32(y) {
        Ok(v) => ret = v as i64,
        Err(_) => { },
    }

    match d.get_i64(y) {
        Ok(v) => ret = v as i64,
        Err(_) => { },
    }

    ret
}