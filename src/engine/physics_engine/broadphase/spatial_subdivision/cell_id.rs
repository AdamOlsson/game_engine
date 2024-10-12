
#[derive(PartialEq, Debug)]
pub struct CellId {
    pub cell_id: (u32,u32,u32),
    pub cell_object_type: CellIdType,
    pub object_id: usize,
}

impl CellId {
    pub fn new(cell_id: (u32,u32,u32), cell_type:CellIdType, object_id: usize) -> Self {
        Self {cell_id, cell_object_type: cell_type, object_id }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum CellIdType {
    Home,
    Phantom,
}

impl std::fmt::Display for CellIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Home => write!(f, "CellIdType::Home"),
            Self::Phantom => write!(f, "CellIdType::Phantom"),
        }
    }
}

impl std::fmt::Display for CellId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object ID: {}, Cell ID: {:?}, Type: {}",
            self.object_id, self.cell_id, self.cell_object_type)
    }
}
