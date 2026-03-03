use core::fmt;

pub struct Green(pub &'static str);

impl fmt::Display for Green {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[32m{}\x1B[0m", self.0)?;
        Ok(())
    }
}

pub struct Red(pub &'static str);

impl fmt::Display for Red {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[31m{}\x1B[0m", self.0)?;
        Ok(())
    }
}

pub struct Yellow(pub &'static str);

impl fmt::Display for Yellow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[93m{}\x1B[0m", self.0)?;
        Ok(())
    }
}
