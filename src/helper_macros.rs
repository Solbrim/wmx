#[macro_export]
macro_rules! unwrap_or_return {
    ( $i:expr, $name:expr ) => {
        match $i {
            Ok(x) => x,
            Err(err) => {
                println!("Failed to unwrap {}: {:?}", $name, err);
                return
            },
        }
    }
}

#[macro_export]
macro_rules! unwrap_or_continue {
    ( $i:expr, $name:expr ) => {
        match $i {
            Ok(x) => x,
            Err(_) => {
                println!("Failed to unwrap {}, continue...", $name);
                continue;
            },
        }
    }
}

#[macro_export]
macro_rules! unwrap_or_stfu {
    ( $i:expr, $name:expr ) => {
        match $i {
            Ok(x) => x,
            Err(_) => {
                return Err(StringError::anyhow(format!("Failed to unwrap {} -- it contained an Error", $name)));
            },
        }
    }
}

#[macro_export]
macro_rules! unwrap_win {
    ( $i:expr, $name:expr ) => {
        match $i {
            Ok(x) => x,
            Err(e) => {
                return Err(StringError::anyhow(format!("Failed to unwrap {} -- it contained an Error: {}", $name, e.message().to_string_lossy())));
            },
        }
    }
}

#[macro_export]
macro_rules! unwrap_or_default {
    ( $i:expr, $name:expr, $default:expr ) => {
        match $i {
            Ok(x) => x,
            Err(_) => {
                println!("Failed to unwrap {}, continue...", $name);
                return $default.to_string();
            },
        }
    }
}