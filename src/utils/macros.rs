#![allow(unused_macros)]

macro_rules! opt_warn {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                warn!($message);
                continue;
            }
        }
    };
}

macro_rules! rs_warn {
    ($result:expr, $message: expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                warn!("{}", error.description());
                continue;
            }
        }
    };
}

macro_rules! opt_error {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                error!($message);
                continue;
            }
        }
    };
}

macro_rules! rs_error {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                error!("{}", error.description());
                continue;
            }
        }
    };
}

macro_rules! opt_ret_warn {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                warn!($message);
                return None;
            }
        }
    };
}

macro_rules! rs_ret_warn {
    ($result:expr, $message: expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                warn!("{}", error.description());
                return None;
            }
        }
    };
}

macro_rules! opt_ret_error {
    ($option:expr, $message:expr) => {
        match $option {
            Some(result) => result,
            None => {
                error!($message);
                return None;
            }
        }
    };
}

macro_rules! rs_ret_error {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(error) => {
                use std::error::Error;
                error!("{}", error.description());
                return None;
            }
        }
    };
}

macro_rules! debug {
    ($fmt:expr) => (if cfg!(debug_assertions) {println!($fmt)});
    ($fmt:expr, $($arg:tt)*) => (if cfg!(debug_assertions) {println!($fmt, $($arg)*)});
}
