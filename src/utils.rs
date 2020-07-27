#[macro_export]
macro_rules! input {
    ($p:expr, $t:ident) => {{
        println!("{}", $p);
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf)
            .expect("READ ERROR: in macro input");
        let val: $t = buf.trim().parse()
            .expect("PARSE ERROR: in macro input");
        val
    }};
}

//#[cfg(target_os = "linux")]
pub fn clear_screen() {
    let output = std::process::Command::new("clear")
        .output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn check_in_range<T: Copy + PartialOrd>(value: T, low: T, high: T) -> bool {
    low < value && high > value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_in_range() {
        assert_eq!(true, check_in_range(1, 0, 2));
        assert_eq!(true, check_in_range(2, 0, 3));
        assert_eq!(false, check_in_range(3, 0, 2));
    }
}

