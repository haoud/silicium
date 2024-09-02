use crate::library::io;

#[trait_variant::make(Send)]
pub trait Read {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// This function behaves similarly to [`util::io::Read`], except that it
    /// will yield to the executor if it is unable to read more data instead
    /// of blocking or returning an error code. Refer to the documentation of
    /// [`util::io::Read`] for more information.
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error>;
}

#[trait_variant::make(Send)]
pub trait Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    ///
    /// This function behaves similarly to [`util::io::Write`], except that it
    /// will yield to the executor if it is unable to write more data instead
    /// of blocking or returning an error code. Refer to the documentation of
    /// [`util::io::Write`] for more information.
    async fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error>;

    /// Flush the writer. This function is used to make sure that the buffer is
    /// written into the underlying storage.
    ///
    /// This function behaves similarly to [`util::io::Write::flush`], except
    /// that it will yield to the executor if it is unable to flush the data
    /// instead of blocking or returning an error code. Refer to the
    /// documentation of [`util::io::Write::flush`] for more information.
    async fn flush(&mut self) -> Result<(), io::Error>;
}

#[trait_variant::make(Send)]
pub trait Seek {
    /// Seek to an offset, in bytes, in a stream.
    ///
    /// This function behaves similarly to [`util::io::Seek`], except that it
    /// will yield to the executor if it is unable to seek the data instead
    /// of blocking or returning an error code. Refer to the documentation of
    /// [`util::io::Seek`] for more information.
    async fn seek(&mut self, pos: io::Whence) -> Result<u64, io::Error>;
}
