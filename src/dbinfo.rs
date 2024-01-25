use anyhow::Result;
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Read,
};

pub struct DbInfo {
    header: Header,
    btree_header: BTreeHeader,
}

struct Header {
    page_size: u16,
}

struct BTreeHeader {
    number_of_cells: u16,
}

impl Display for DbInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "database page size: {}\nnumber of tables: {}",
            self.header.page_size, self.btree_header.number_of_cells
        )
    }
}

impl TryFrom<&String> for DbInfo {
    type Error = anyhow::Error;
    fn try_from(file: &String) -> Result<DbInfo, Self::Error> {
        let mut file = File::open(file)?;
        let mut header = [0; 100];
        file.read_exact(&mut header)?;
        let header = Header::try_from(header)?;
        let mut btree_header = [0; 12];
        file.read_exact(&mut btree_header)?;
        let btree_header = BTreeHeader::try_from(btree_header)?;
        Ok(DbInfo {
            header,
            btree_header,
        })
    }
}

impl TryFrom<[u8; 100]> for Header {
    type Error = anyhow::Error;
    fn try_from(value: [u8; 100]) -> Result<Header, Self::Error> {
        let page_size = u16::from_be_bytes([value[16], value[17]]);
        return Ok(Header { page_size });
    }
}

impl TryFrom<[u8; 12]> for BTreeHeader {
    type Error = anyhow::Error;
    fn try_from(value: [u8; 12]) -> Result<BTreeHeader, Self::Error> {
        let number_of_cells = u16::from_be_bytes([value[3], value[4]]);
        return Ok(BTreeHeader { number_of_cells });
    }
}
