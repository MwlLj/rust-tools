#[derive(Eq, Debug)]
pub struct CLibInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub no: &'a u32,
    pub root: String
}

impl<'a> std::cmp::PartialEq for CLibInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.no == other.no
    }
}

impl<'a> std::cmp::PartialOrd for CLibInfo<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> std::cmp::Ord for CLibInfo<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.no.cmp(&other.no)
    }
}
