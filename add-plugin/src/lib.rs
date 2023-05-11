extern "C" {
    fn log_line(offset: i32, size: i32);
}

#[no_mangle]
pub fn add(a: f32, b: f32) -> f32 {
    let log_text = format!("add {a} + {b}");
    let log_bytes = log_text.as_bytes();

    unsafe { log_line(log_bytes.as_ptr() as _, log_bytes.len() as _) };
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
