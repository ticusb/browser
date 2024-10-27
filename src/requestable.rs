use std::io;

pub trait Requestable {
    fn request(&self) -> Result<String, io::Error>;
    fn scheme(&self) -> String;
}
