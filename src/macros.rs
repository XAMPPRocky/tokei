// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

macro_rules! opt_or_cont {
    ($option:expr) => {
        match $option {
            Some(result) => result,
            None => continue,
        }
    }
}

macro_rules! rs_or_cont {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(_) => continue,
        }
    }
}


macro_rules! debug {
    ($fmt:expr) => (if cfg!(debug_assertions) {println!($fmt)});
    ($fmt:expr, $($arg:tt)*) => (if cfg!(debug_assertions) {println!($fmt, $($arg)*)});
}
