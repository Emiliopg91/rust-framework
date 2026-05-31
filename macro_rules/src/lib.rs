#[macro_export]
macro_rules! debug {
    ($name:expr, $($arg:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name)
            .debug(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! info {
    ($name:expr, $($arg:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name)
            .info(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warning {
    ($name:expr, $($arg:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name)
            .warning(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($name:expr, $($arg:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name)
            .error(format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! critical {
    ($name:expr, $($arg:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name)
            .critical(format!($($arg)*));
    }};
}
