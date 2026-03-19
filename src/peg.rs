// peg.rs
//
// Copyright (c) 2019-2026  Douglas Lau
//
use std::fmt;

/// A Peg identifies a partition (tile) on a map grid at a specific zoom level.
///
/// It uses **ZXY** addressing:
/// - **Z**: zoom level, starting with 0 (one tile for the entire Earth)
/// - **X**: west to east tile number
/// - **Y**: north to south tile number
///
/// Each successive zoom level has twice as many tiles in each dimension (the
/// **X** and **Y** values can range from 0 to 2<sup>Z</sup>-1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Peg {
    z: u32, // not public to prevent invalid values being created
    x: u32,
    y: u32,
}

impl Peg {
    /// Get the Z (zoom) value
    pub fn z(&self) -> u32 {
        self.z
    }

    /// Get the X value
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Get the Y value
    pub fn y(&self) -> u32 {
        self.y
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
    pub fn new(z: u32, x: u32, y: u32) -> Option<Self> {
        Peg::check_valid(z, x, y).then_some(Peg { z, x, y })
    }

    /// Check whether ZXY is valid
    fn check_valid(z: u32, x: u32, y: u32) -> bool {
        z <= 31 && {
            let s = 1 << z;
            x < s && y < s
        }
    }
}
