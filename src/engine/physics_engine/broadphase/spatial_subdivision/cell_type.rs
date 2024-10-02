#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CellType {
    One,
    Two,
    Three,
    Four,
}

impl CellType {
    pub fn top_neighbor(&self) -> Self {
        match self {
            Self::One => Self::Three,
            Self::Three => Self::One,
            Self::Two => Self::Four,
            Self::Four => Self::Two,
        }
    }

    pub fn bottom_neighbor(&self) -> Self {
        self.top_neighbor()
    }

    pub fn left_neighor(&self) -> Self {
        match self {
            Self::One => Self::Two,
            Self::Two => Self::One,
            Self::Three => Self::Four,
            Self::Four => Self::Three,
        }
    }

    pub fn right_neighbor(&self) -> Self {
        self.left_neighor()
    }

    pub fn top_left_neighor(&self) -> Self {
        match self {
            Self::One => Self::Four,
            Self::Two => Self::Three,
            Self::Three => Self::Two,
            Self::Four => Self::One,
        }
    }

    pub fn bottom_right_neighbor(&self) -> Self {
        self.top_left_neighor()
    }
}
