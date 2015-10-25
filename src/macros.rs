// Copyright (c) 2015 Aaron Power
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

macro_rules! unwrap_opt_cont {
    ($option:expr) => {
        match $option {
            Some(result) => result,
            None => continue,
        }
    }
}

macro_rules! unwrap_rs_cont {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(_) => continue,
        }
    }
}
