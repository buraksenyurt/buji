use std::io::Write;

/// Mock instance object of Log struct to use on tests.
pub struct MockLogger;

impl Write for MockLogger {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
