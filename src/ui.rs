#[macro_export]
macro_rules! message {
    () => {
        std::eprintln!()
    };
    ($($arg:tt)*) => {{
        std::eprintln!("\x1b[35;1m{}\x1b[0m", std::format_args!($($arg)*))
    }};
}

#[macro_export]
macro_rules! error {
    () => {
        std::eprintln!()
    };
    ($($arg:tt)*) => {{
        std::eprintln!("\x1b[31;1mError!\x1b[0m \x1b[1m{}\x1b[0m", std::format_args!($($arg)*))
    }};
}
