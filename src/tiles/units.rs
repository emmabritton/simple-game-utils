use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct MapSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}

impl MapSize {
    #[inline]
    pub const fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

impl MapPosition {
    #[inline]
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn from_idx(i: usize, using: MapSize) -> MapPosition {
        MapPosition {
            x: i as u32 % using.w,
            y: i as u32 / using.w,
        }
    }
}

impl MapSize {
    #[inline]
    pub const fn count(&self) -> u32 {
        self.w * self.h
    }
}

impl MapPosition {
    #[inline]
    pub const fn to_idx(&self, using: MapSize) -> usize {
        (self.x + self.y * using.w) as usize
    }
}

macro_rules! from_num {
    ($num:ty) => {
        impl From<($num, $num)> for MapPosition {
            #[inline]
            fn from(value: ($num, $num)) -> Self {
                MapPosition {
                    x: value.0 as u32,
                    y: value.1 as u32,
                }
            }
        }
        impl From<($num, $num)> for MapSize {
            #[inline]
            fn from(value: ($num, $num)) -> Self {
                MapSize {
                    w: value.0 as u32,
                    h: value.1 as u32,
                }
            }
        }
    };
}

from_num!(u64);
from_num!(u32);
from_num!(u16);
from_num!(u8);
from_num!(usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn macros() {
        let _u64s: MapSize = (12_u64, 12_u64).into();
        let _u64p: MapPosition = (12_u64, 12_u64).into();

        assert_eq!(MapSize::new(12, 12), _u64s);
        assert_eq!(MapPosition::new(12, 12), _u64p);
    }

    #[test]
    fn indexing() {
        let size1 = MapSize::new(4, 4);
        let size2 = MapSize::new(8, 4);

        let pos1 = MapPosition::new(1, 1);

        assert_eq!(pos1.to_idx(size1), 5);
        assert_eq!(pos1.to_idx(size2), 9);
        assert_eq!(MapPosition::from_idx(15, size1), MapPosition::new(3, 3));
        assert_eq!(MapPosition::from_idx(15, size2), MapPosition::new(7, 1));
    }
}
