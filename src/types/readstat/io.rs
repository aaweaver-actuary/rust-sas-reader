// typedef int (*readstat_open_handler)(const char *path, void *io_ctx);
// typedef int (*readstat_close_handler)(void *io_ctx);
// typedef readstat_off_t (*readstat_seek_handler)(readstat_off_t offset, readstat_io_flags_t whence, void *io_ctx);
// typedef ssize_t (*readstat_read_handler)(void *buf, size_t nbyte, void *io_ctx);
// typedef readstat_error_t (*readstat_update_handler)(long file_size, readstat_progress_handler progress_handler, void *user_ctx, void *io_ctx);
// typedef struct readstat_io_s {
// readstat_open_handler          open;
// readstat_close_handler         close;
// readstat_seek_handler          seek;
// readstat_read_handler          read;
// readstat_update_handler        update;
// void                          *io_ctx;
// int                            io_ctx_needs_free;
// } readstat_io_t;

use crate::types::ReadStatError;
use std::fs::{File, Path};

/// ReadStatIO is a struct that contains the file path and the file I/O handlers.
/// The file I/O handlers are used to open, close, seek, read, and update the file.
/// Since this is implemented in Rust instead of C, the file I/O handlers are implemented as
/// methods, instead of function pointers.
///
/// Rust features that simplify the implementation of the file I/O handlers include:
/// - The `std::fs::File` struct, which provides methods for opening, closing, seeking, and reading
/// files.
/// - The `std::fs::File` struct, which provides a `metadata` method that returns a `Metadata`
/// struct that contains the file size.
/// - The `std::fs::File` struct, which provides a `set_len` method that sets the file size.
pub struct ReadStatIO {
    path: Path,
    file: Option<File>,
}

impl ReadStatIO {
    /// Create a new `ReadStatIO` struct with the given file path and no file.
    pub fn new(path: Path) -> Self {
        Self { path, file: None }
    }

    /// Open the file at the given path.
    pub fn open(&mut self) -> Result<(), ReadStatError> {
        self.file = Some(File::open(&self.path)?);
        Ok(())
    }

    /// Close the file.
    pub fn close(&mut self) -> Result<(), ReadStatError> {
        self.file = None;
        Ok(())
    }

    /// Seek to the given offset in the file.
    pub fn seek(&mut self, offset: i64, whence: std::io::SeekFrom) -> Result<(), ReadStatError> {
        self.file.as_mut().unwrap().seek(whence)?;
        Ok(())
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, ReadStatError> {
        self.file.as_mut().unwrap().read(buf)
    }

    pub fn update(&mut self, file_size: u64) -> Result<(), ReadStatError> {
        self.file.as_mut().unwrap().set_len(file_size)?;
        Ok(())
    }
}
