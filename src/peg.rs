// peg.rs
//
// Copyright (c) 2019-2026  Douglas Lau
//
use std::fmt;

/// A Peg identifies a partition (tile) on a map grid at a specific zoom level.
///
/// It uses XYZ addressing, with `X` increasing from west to east and `Y`
/// increasing from north to south.  `Z` represents zoom, with `0` containing
/// one tile for the entire Earth.  Each subsequent zoom level has twice as
/// many tiles in each dimension (the `X` and `Y` values can range from `0` to
/// `2<sup>Z</sup>-1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Peg {
    x: u32, // not public to prevent invalid values being created
    y: u32,
    z: u32,
}

impl Peg {
    /// Get the X value
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Get the Y value
    pub fn y(&self) -> u32 {
        self.y
    }

    /// Get the Z (zoom) value
    pub fn z(&self) -> u32 {
        self.z
    }
}

impl fmt::Display for Peg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.z, self.x, self.y)
    }
}

impl Peg {
    /// Create a new Peg (partition of Earth's geography)
    ///
    /// If invalid, returns `None`
    pub fn new(x: u32, y: u32, z: u32) -> Option<Self> {
        Peg::check_valid(x, y, z).then_some(Peg { x, y, z })
    }

    /// Check whether XYZ is valid
    fn check_valid(x: u32, y: u32, z: u32) -> bool {
        z <= 31 && {
            let s = 1 << z;
            x < s && y < s
        }
    }
}
