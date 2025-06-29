#[cfg(test)]
#[macro_export]
macro_rules! fail {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        return false;
    }};
}
