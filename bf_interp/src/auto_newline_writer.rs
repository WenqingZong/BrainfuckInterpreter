//! A writer which will automatically add a new line when Dropping.

use std::io::{self, Write};

/// The handy new writer, it has a similar API to any other [Write] implementations.
pub struct AutoNewlineWriter<W: Write> {
    writer: W,
    last_written_char_is_newline: bool,
}

impl<W: Write> AutoNewlineWriter<W> {
    /// The new writer requires an existing writer to construct.
    /// # Example
    /// ```rust
    /// # use bf_interp::auto_newline_writer::*;
    /// use std::io::{stdout, Write};
    /// let mut writer = stdout();
    /// let mut auto_newline_writer = AutoNewlineWriter::new(&mut writer);
    ///
    /// let buf = &[b'H'];
    /// auto_newline_writer.write(buf);
    ///
    /// // drop auto_newline_writer here.
    /// // And you will get a new line character in your write destination.
    /// ```
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            last_written_char_is_newline: false,
        }
    }
}

impl<W: Write> Write for AutoNewlineWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.last_written_char_is_newline = buf.last().map_or(false, |&char| char == b'\n');
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Drop for AutoNewlineWriter<W> {
    fn drop(&mut self) {
        if !self.last_written_char_is_newline {
            let _ = self.write(&[b'\n']);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    /// Should add a new line character after drop trait.
    #[test]
    fn should_add_a_newline() {
        let mut writer: Cursor<Vec<u8>> = Cursor::new(vec![]);
        let mut auto_newline_writer = AutoNewlineWriter::new(&mut writer);
        let buf = &[b'a'];
        let _ = auto_newline_writer.write(buf);

        drop(auto_newline_writer);
        assert_eq!(writer.into_inner(), vec![b'a', b'\n']);
    }
}
