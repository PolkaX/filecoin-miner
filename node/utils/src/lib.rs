use std::error;
use std::io;

pub fn other_io_err<E>(e: E) -> io::Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    io::Error::new(io::ErrorKind::Other, e)
}

pub fn base32_decode(input: impl AsRef<[u8]>) -> Result<Vec<u8>, data_encoding::DecodeError> {
    data_encoding::BASE32_NOPAD.decode(&input.as_ref().to_ascii_uppercase())
}
