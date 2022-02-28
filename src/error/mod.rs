use {
    std::error::Error,
    std::fmt::{ Display, Error as FmtErr, Formatter, },
};
#[derive(Debug)]
pub struct DecibelDbErr {
    message: String,
}
pub trait DecibelDbError<T> {
    fn quick(self) -> Result<T, DecibelDbErr>;
}
impl<T, U> DecibelDbError<T> for Result<T, U>
where
    T: Sized,
    U: Error,
{
    fn quick(self) -> Result<T, DecibelDbErr> {
        match self {
            Ok(t) => Ok(t),
            Err(u) => Err(DecibelDbErr { message: format!("{}", u), }),
        }
    }
}
impl Display for DecibelDbErr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtErr> {
        fmt.write_str(&self.message)
    }
}
impl Error for DecibelDbErr {}
