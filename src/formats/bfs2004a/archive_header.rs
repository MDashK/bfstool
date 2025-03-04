use binrw::BinRead;

/// Archive Header for archive of formats: Bfs2004a, Bfs2004b
#[derive(Debug, Default, Eq, PartialEq, BinRead)]
#[brw(little)]
pub struct ArchiveHeader {
    /// File identification magic
    ///
    /// `62 66 73 31`, `"bfs1"`
    pub magic: u32,
    /// File version
    ///
    /// `05 05 04 20`, v2004.05.05a and v2004.05.05b
    pub version: u32,
    /// Offset at which the header section ends
    pub header_end: u32,
    /// Number of files in the archive
    pub file_count: u32,
}

/// Bfs2004a-specific tests
#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn parsing_test() {
        // Test data comes from europe.bfs, first 10h bytes
        let test_data = include_bytes!("../../../test_data/bfs2004a/europe.bin");
        let test_data = &test_data[..=0x10];

        let mut test_data_cursor = Cursor::new(test_data);

        let result = ArchiveHeader::read(&mut test_data_cursor);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ArchiveHeader {
                magic: 0x31736662,
                version: 0x20040505,
                header_end: 0xFDB,
                file_count: 1,
            }
        );

        // Test data comes from common1.bfs, first 10h bytes
        let test_data = include_bytes!("../../../test_data/bfs2004a/common1.bin");
        let test_data = &test_data[..=0x10];

        let mut test_data_cursor = Cursor::new(test_data);

        let result = ArchiveHeader::read(&mut test_data_cursor);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            ArchiveHeader {
                magic: 0x31736662,
                version: 0x20040505,
                header_end: 0x101DD,
                file_count: 1116,
            }
        );
    }
}
