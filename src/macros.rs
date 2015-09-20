#[inline(always)]
macro_rules! unwrap_opt_cont {
    ($option:expr) => {
        match $option {
            Some(result) => result,
            None => continue,
        }
    }
}

#[inline(always)]
macro_rules! unwrap_rs_cont {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(_) => continue,
        }
    }
}
