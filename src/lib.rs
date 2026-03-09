// lib.rs      squarepeg crate.
//
// Copyright (c) 2026  Douglas Lau
#![forbid(unsafe_code)]

mod geo;
mod mapgrid;
mod peg;

pub use crate::geo::{WebMercatorPos, Wgs84Pos};
pub use crate::mapgrid::MapGrid;
pub use crate::peg::Peg;
