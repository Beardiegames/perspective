use std::ops::{Add, Sub};
use super::*;


#[derive(Debug, Clone)]
pub struct MxPos {
	pub hor: i16,
	pub ver: i16,
}

impl From<TilePos> for MxPos {
	fn from(other: TilePos) -> Self {
		MxPos {
			hor: other.hor.round() as i16,
			ver: other.ver.round() as i16,
		}
	}
}

impl From<&TilePos> for MxPos {
	fn from(other: &TilePos) -> Self {
		MxPos {
			hor: other.hor.round() as i16,
			ver: other.ver.round() as i16,
		}
	}
}

impl Add<&MxPos> for &MxPos {
	type Output = MxPos;
	
    fn add(self, other: &MxPos) -> MxPos {
        MxPos {
            hor: self.hor + other.hor,
            ver: self.ver + other.ver,
        }
    }
}

impl Sub<&MxPos> for &MxPos {
	type Output = MxPos;
	
    fn sub(self, other: &MxPos) -> MxPos {
        MxPos {
            hor: self.hor - other.hor,
            ver: self.ver - other.ver,
        }
    }
}

impl Add<MxPos> for MxPos {
	type Output = MxPos;
	
    fn add(self, other: MxPos) -> MxPos {
        MxPos {
            hor: self.hor + other.hor,
            ver: self.ver + other.ver,
        }
    }
}

impl Sub<MxPos> for MxPos {
	type Output = MxPos;
	
    fn sub(self, other: MxPos) -> MxPos {
        MxPos {
            hor: self.hor - other.hor,
            ver: self.ver - other.ver,
        }
    }
}

