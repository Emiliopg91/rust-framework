#[allow(unused_macros)]
macro_rules! debug {
    ($name:expr, $fmt:expr $(, $args:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name).debug(
            format!($fmt $(, $args)*)
        );
    }};
}

#[allow(unused_macros)]
macro_rules! info {
    ($name:expr, $fmt:expr $(, $args:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name).info(
            format!($fmt $(, $args)*)
        );
    }};
}

#[allow(unused_macros)]
macro_rules! warning {
    ($name:expr, $fmt:expr $(, $args:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name).warning(
            format!($fmt $(, $args)*)
        );
    }};
}

#[allow(unused_macros)]
macro_rules! error {
    ($name:expr, $fmt:expr $(, $args:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name).error(
            format!($fmt $(, $args)*)
        );
    }};
}

#[allow(unused_macros)]
macro_rules! critical {
    ($name:expr, $fmt:expr $(, $args:tt)*) => {{
        fwkarq::logger::provider::Provider::get_logger($name).critical(
            format!($fmt $(, $args)*)
        );
    }};
}
