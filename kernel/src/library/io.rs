//! This file contains the `IO` trait and its implementations, heavily borrowed
//! by the `std::io` module. What a shame that the OS-independent part of the
//! `std::io` module are not included in the `core` library...

/// A `Read` trait for reading bytes from a stream of bytes.
pub trait Read {
    /// Pull some bytes from this source into the specified buffer, returning
    /// how many bytes were read.
    ///
    /// This function does not provide any guarantees about whether it blocks
    /// waiting for data, but if an object needs to block for a read and cannot,
    /// it will typically signal this via an [`Err`] return value.
    ///
    /// If the return value of this method is [`Ok(n)`], then implementations
    /// must guarantee that `0 <= n <= buf.len()`. A nonzero `n` value indicates
    /// that the buffer `buf` has been filled in with `n` bytes of data from
    /// this source. If `n` is `0`, then it can indicate one of two scenarios:
    ///
    /// 1. This reader has reached its "end of file" and will likely no longer
    ///    be able to produce bytes. Note that this does not mean that the
    ///    reader will *always* no longer be able to produce bytes. As an
    ///    example, on Linux, this method will call the `recv` syscall for a
    ///    [`TcpStream`], where returning zero indicates the connection was shut
    ///    down correctly. While for [`File`], it is possible to reach the end
    ///    of file and get zero as result, but if more data is appended to the
    ///    file, future calls to `read` will return more data.
    /// 2. The buffer specified was 0 bytes in length.
    ///
    /// It is not an error if the returned value `n` is smaller than the buffer
    /// size, even when the reader is not at the end of the stream yet.
    /// This may happen for example because fewer bytes are actually available
    /// right now (e. g. being close to end-of-file) or because read() was
    /// interrupted by a signal.
    ///
    /// As this trait is safe to implement, callers in unsafe code cannot rely
    /// on `n <= buf.len()` for safety.
    /// Extra care needs to be taken when `unsafe` functions are used to access
    /// the read bytes. Callers have to ensure that no unchecked out-of-bounds
    /// accesses are possible even if `n > buf.len()`.
    ///
    /// *Implementations* of this method can make no assumptions about the
    /// contents of `buf` when this function is called. It is recommended that
    /// implementations only write data to `buf` instead of reading its
    /// contents.
    ///
    /// Correspondingly, however, *callers* of this method in unsafe code must
    /// not assume any guarantees about how the implementation uses `buf`. The
    /// trait is safe to implement, so it is possible that the code that's
    /// supposed to write to the buffer might also read from it. It is your
    /// responsibility to make sure that `buf` is initialized before calling
    /// `read`. Calling `read` with an uninitialized `buf` (of the kind one
    /// obtains via [`MaybeUninit<T>`]) is not safe, and can lead to undefined
    /// behavior.
    ///
    /// # Errors
    ///
    /// If this function encounters any form of I/O or other error, an error
    /// variant will be returned. If an error is returned then it must be
    /// guaranteed that no bytes were read.
    ///
    /// An error of the [`ErrorKind::Interrupted`] kind is non-fatal and the
    /// read operation should be retried if there is nothing else to do.
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    /// Read all bytes until EOF in this source, placing them into `buf`.
    ///
    /// All bytes read from this source will be appended to the specified buffer
    /// `buf`. This function will continuously call [`read()`] to append more
    /// data to `buf` until [`read()`] returns either [`Ok(0)`] or an error of
    /// non-[`ErrorKind::Interrupted`] kind.
    ///
    /// If successful, this function will return the total number of bytes read.
    ///
    /// # Errors
    /// If this function encounters an error of the kind
    /// [`ErrorKind::Interrupted`] then the error is ignored and the operation
    /// will continue.
    ///
    /// If any other read error is encountered then this function immediately
    /// returns. Any bytes which have already been read will be appended to
    /// `buf`.
    ///
    /// ## Implementing `read_to_end`
    /// When implementing the `io::Read` trait, it is recommended to allocate
    /// memory using [`Vec::try_reserve`]. However, this behavior is not
    /// guaranteed by all implementations, and `read_to_end` may not handle
    /// out-of-memory situations gracefully.
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error> {
        // Simply read the stream byte by byte until we hit EOF. This is
        // very inefficient, but should work correctly and is simple to
        // implement.
        let mut byte = [0];
        let mut read = 0;

        while self.read(&mut byte)? > 0 {
            buf.push(byte[0]);
            read += 1;
        }
        Ok(read)
    }

    /// Read the exact number of bytes required to fill `buf`.
    ///
    /// This function reads as many bytes as necessary to completely fill the
    /// specified buffer `buf`.
    ///
    /// *Implementations* of this method can make no assumptions about the
    /// contents of `buf` when this function is called. It is recommended that
    /// implementations only write data to `buf` instead of reading its
    /// contents. The documentation on [`read`] has a more detailed explanation
    /// of this subject.
    ///
    /// # Errors
    /// If this function encounters an error of the kind
    /// [`ErrorKind::Interrupted`] then the error is ignored and the operation
    /// will continue.
    ///
    /// If this function encounters an "end of file" before completely filling
    /// the buffer, it returns an error of the kind [`ErrorKind::UnexpectedEof`].
    /// The contents of `buf` are unspecified in this case.
    ///
    /// If any other read error is encountered then this function immediately
    /// returns. The contents of `buf` are unspecified in this case.
    ///
    /// If this function returns an error, it is unspecified how many bytes it
    /// has read, but it will never read more than would be necessary to
    /// completely fill the buffer.
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let mut total = 0;
        while total < buf.len() {
            match self.read(&mut buf[total..]) {
                Ok(0) => return Err(Error::UnexpectedEof),
                Ok(n) => total += n,
                Err(Error::Interrupted) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Transforms this `Read` instance to an [`Iterator`] over its bytes.
    ///
    /// The returned type implements [`Iterator`] where the [`Item`] is
    /// <code>[Result]<[u8], [io::Error]></code>.
    /// The yielded item is [`Ok`] if a byte was successfully read and [`Err`]
    /// otherwise. EOF is mapped to returning [`None`] from this iterator.
    ///
    /// The default implementation calls `read` for each byte,
    /// which can be very inefficient for data that's not in memory,
    /// such as [`File`]. Consider using a [`BufReader`] in such cases.
    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized,
    {
        Bytes { inner: self }
    }

    /// Creates an adapter which will read at most `limit` bytes from it.
    ///
    /// This function returns a new instance of `Read` which will read at most
    /// `limit` bytes, after which it will always return EOF ([`Ok(0)`]). Any
    /// read errors will not count towards the number of bytes read and future
    /// calls to [`read()`] may succeed.
    fn take(self, limit: u64) -> Take<Self>
    where
        Self: Sized,
    {
        Take { inner: self, limit }
    }
}

/// A `Write` trait for writing bytes to a sink.
pub trait Write {
    /// Write a buffer into this writer, returning how many bytes were written.
    /// This function will attempt to write the entire contents of buf, but the
    /// entire write might not succeed, or the write may also generate an error.
    /// Typically, a call to write represents one attempt to write to any
    /// wrapped object.
    ///
    /// Calls to write are not guaranteed to block waiting for data to be
    /// written, and a write which would otherwise block can be indicated
    /// through an Err variant. If this method consumed n > 0 bytes of buf
    /// it must return Ok(n). If the return value is Ok(n) then n must satisfy
    /// n <= buf.len(). A return value of Ok(0) typically means that the
    /// underlying object is no longer able to accept bytes and will likely not
    /// be able to in the future as well, or that the buffer provided is empty.
    ///
    /// # Errors
    /// Each call to write may generate an I/O error indicating that the
    /// operation could not be completed. If an error is returned then no bytes
    /// in the buffer were written to this writer. It is not considered an error
    /// if the entire buffer could not be written to this writer.
    ///
    /// An error of the `ErrorKind::Interrupted` kind is non-fatal and the write
    /// operation should be retried if there is nothing else to do.
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;

    /// Flush the writer. This function is used to make sure that the buffer is
    /// written into the underlying storage.
    fn flush(&mut self) -> Result<(), Error>;

    /// Attempts to write an entire buffer into this writer.
    ///
    /// This method will continuously call [`write`] until there is no more data
    /// to be written or an error of non-[`ErrorKind::Interrupted`] kind is
    /// returned. This method will not return until the entire buffer has been
    /// successfully written or such an error occurs. The first error that is
    /// not of [`ErrorKind::Interrupted`] kind generated from this method will
    /// be returned.
    ///
    /// If the buffer contains no data, this will never call [`write`].
    ///
    /// # Errors
    ///
    /// This function will return the first error of
    /// non-[`ErrorKind::Interrupted`] kind that [`write`] returns.
    ///
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error> {
        let mut total = 0;
        while total < buf.len() {
            match self.write(&buf[total..]) {
                Ok(0) => return Err(Error::UnexpectedEof),
                Ok(n) => total += n,
                Err(Error::Interrupted) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

pub trait Seek {
    /// Seek to an offset, in bytes, in a stream.
    ///
    /// A seek beyond the end of a stream is allowed, but behavior is defined
    /// by the implementation.
    ///
    /// If the seek operation completed successfully, this method returns the
    /// new position from the start of the stream. That position can be used
    /// later with SeekFrom::Start.
    ///
    /// # Errors
    /// Seeking can fail, for example because it might involve flushing a
    /// buffer. Seeking to a negative offset is considered an error.
    fn seek(&mut self, pos: Whence) -> Result<u64, Error>;

    /// Rewind to the beginning of a stream.
    ///
    /// This is a convenience method, equivalent to `seek(SeekFrom::Start(0))`.
    ///
    /// # Errors
    /// Rewinding can fail, for example because it might involve flushing
    /// a buffer.
    fn rewind(&mut self) -> Result<(), Error> {
        self.seek(Whence::Start(0)).map(|_| ())
    }

    /// Returns the length of this stream (in bytes).
    ///
    /// This method is implemented using up to three seek operations. If this
    /// method returns successfully, the seek position is unchanged (i.e. the
    /// position before calling this method is the same as afterwards).
    /// However, if this method returns an error, the seek position is
    /// unspecified.
    ///
    /// If you need to obtain the length of *many* streams and you don't care
    /// about the seek position afterwards, you can reduce the number of seek
    /// operations by simply calling `seek(SeekFrom::End(0))` and using its
    /// return value (it is also the stream length).
    ///
    /// Note that length of a stream can change over time (for example, when
    /// data is appended to a file). So calling this method multiple times does
    /// not necessarily return the same length each time.
    fn stream_len(&mut self) -> Result<u64, Error> {
        let pos = self.stream_position()?;
        let len = self.seek(Whence::End(0))?;
        self.seek(Whence::Start(pos))?;
        Ok(len)
    }

    /// Returns the current seek position from the start of the stream.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(0))`.
    fn stream_position(&mut self) -> Result<u64, Error> {
        self.seek(Whence::Current(0))
    }

    /// Seeks relative to the current position.
    ///
    /// This is equivalent to `self.seek(SeekFrom::Current(offset))` but
    /// doesn't return the new position which can allow some implementations
    /// such as [`BufReader`] to perform more efficient seeks.
    fn seek_relative(&mut self, offset: i64) -> Result<u64, Error> {
        self.seek(Whence::Current(offset))
    }
}

/// A trait for objects that are readable and writable.
pub trait IO: Read + Write + Seek {}

/// An iterator over u8 values of a reader. This struct is generally created
/// by calling bytes on a reader. Please see the documentation of bytes for
/// more details.
pub struct Bytes<R> {
    inner: R,
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Result<u8, Error>> {
        inlined_slow_read_byte(&mut self.inner)
    }
}

pub struct Take<R> {
    inner: R,
    limit: u64,
}

impl<T> Take<T> {
    /// Returns the number of bytes that can be read before this instance will
    /// return EOF.
    ///
    /// # Note
    /// This instance may reach `EOF` after reading fewer bytes than indicated
    /// by this method if the underlying [`Read`] instance reaches EOF.
    #[must_use]
    pub fn limit(&self) -> u64 {
        self.limit
    }

    /// Sets the number of bytes that can be read before this instance will
    /// return EOF. This is the same as constructing a new `Take` instance, so
    /// the amount of bytes read and the previous limit value don't matter when
    /// calling this method.
    pub fn set_limit(&mut self, limit: u64) {
        self.limit = limit;
    }

    /// Consumes the `Take`, returning the wrapped reader.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /// Gets a reference to the underlying reader.
    #[must_use]
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    ///
    /// Care should be taken to avoid modifying the internal I/O state of the
    /// underlying reader as doing so may corrupt the internal limit of this
    /// `Take`.
    #[must_use]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<R: Read> Read for Take<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        // If we've already hit our limit, return EOF
        if self.limit == 0 {
            return Ok(0);
        }

        let max = core::cmp::min(buf.len(), self.limit as usize);
        let n = self.inner.read(&mut buf[..max])?;
        self.limit -= n as u64;
        Ok(n)
    }
}

/// An enumeration of possible ways to seek within an I/O object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Whence {
    /// Set the offset to the size of the file plus the provided
    Current(i64),

    /// Set the offset to the provided number of bytes.
    Start(u64),

    /// Set the offset to the size of the file plus the provided
    /// number of bytes.
    End(i64),
}

/// Errors that can occur when performing I/O operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    /// The operation was interrupted
    Interrupted,

    /// The operation hit the "end of file" but expected more data
    UnexpectedEof,
}

/// Read a single byte in a slow, generic way. This is sufficient for
/// non-performance-critical code.
#[inline]
fn inlined_slow_read_byte<R: Read>(
    reader: &mut R,
) -> Option<Result<u8, Error>> {
    let mut byte = 0;
    loop {
        return match reader.read(core::slice::from_mut(&mut byte)) {
            Ok(0) => None,
            Ok(..) => Some(Ok(byte)),
            Err(Error::Interrupted) => continue,
            Err(e) => Some(Err(e)),
        };
    }
}
