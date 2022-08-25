use core::fmt::{self, Formatter, Display};

// pub type SimpleResult<T> = core::result::Result<T, String>;

#[derive(Debug)]
pub struct StringError {
    pub message: String,
}
unsafe impl Send for StringError {}
unsafe impl Sync for StringError {}
impl std::error::Error for StringError {}

impl StringError {
    pub fn new(message: String) -> StringError {
        StringError { message }
    }

    pub fn anyhow (message: String) -> anyhow::Error {
        return anyhow::Error::new(StringError { message })
    }
}

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match write!(f, "{}", self.message.as_str()) {
            Ok(_) => (),
            Err(n) => println!("{}", n),
        }
        return Ok(())
    }
}

