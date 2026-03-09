// mapgrid.rs
//
// Copyright (c) 2019-2026  Douglas Lau
//
use crate::geo::WebMercatorPos;
use crate::peg::Peg;
use pointy::{BBox, Pt, Transform};

/// A map grid is used to address [Peg]s on a map.
///
/// The grid should be in projected coördinates.  Use `default()` for
/// [Web Mercator].
///
/// [Peg]: struct.Peg.html
/// [Web Mercator]: https://en.wikipedia.org/wiki/Web_Mercator_projection
#[derive(Clone, Debug)]
pub struct MapGrid {
    /// Spatial reference ID
    srid: i32,

    /// Bounding box
    bbox: BBox<f64>,
}

impl Default for MapGrid {
    fn default() -> Self {
        const WEB_MERCATOR_SRID: i32 = 3857;
        let srid = WEB_MERCATOR_SRID;
        let bbox = WebMercatorPos::bbox();
        Self { srid, bbox }
    }
}

impl MapGrid {
    /// Create a new map grid.
    ///
    /// * `srid` Spatial reference ID.
    /// * `bbox` Bounding box.
    pub fn new(srid: i32, bbox: BBox<f64>) -> Self {
        MapGrid { srid, bbox }
    }

    /// Get the spatial reference ID.
    pub fn srid(&self) -> i32 {
        self.srid
    }

    /// Get the bounding box of the grid.
    pub fn bbox(&self) -> BBox<f64> {
        self.bbox
    }

    /// Get the bounding box of a Peg.
    pub fn bbox_peg(&self, peg: Peg) -> BBox<f64> {
        let px = self.bbox.x_min(); // west edge
        let py = self.bbox.y_max(); // north edge
        let pz = zoom_scale(peg.z());
        let sx = self.bbox.x_span() * pz;
        let sy = self.bbox.y_span() * pz;
        let t = Transform::with_scale(sx, -sy).translate(px, py);
        let pegx = f64::from(peg.x());
        let pegy = f64::from(peg.y());
        let p0 = t * Pt::new(pegx, pegy);
        let p1 = t * Pt::new(pegx + 1.0, pegy + 1.0);
        BBox::from((p0, p1))
    }

    /// Get the transform to coördinates in 0 to 1 range.
    pub fn transform_peg(&self, peg: Peg) -> Transform<f64> {
        let px = self.bbox.x_min(); // west edge
        let py = self.bbox.y_max(); // north edge
        let pz = f64::from(1 << peg.z());
        let sx = pz / self.bbox.x_span();
        let sy = pz / self.bbox.y_span();
        let pegx = f64::from(peg.x());
        let pegy = f64::from(peg.y());
        Transform::with_translate(-px, -py)
            .scale(sx, -sy)
            .translate(-pegx, -pegy)
    }
}

/// Calculate scales at one zoom level.
fn zoom_scale(zoom: u32) -> f64 {
    1.0 / f64::from(1 << zoom)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_peg_bbox() {
        let g = MapGrid::default();
        let peg = Peg::new(0, 0, 0).unwrap();
        let b = g.bbox_peg(peg);
        assert_eq!(b.x_min(), -20037508.3427892480);
        assert_eq!(b.x_max(), 20037508.3427892480);
        assert_eq!(b.y_min(), -20037508.3427892480);
        assert_eq!(b.y_max(), 20037508.3427892480);

        let peg = Peg::new(0, 0, 1).unwrap();
        let b = g.bbox_peg(peg);
        assert_eq!(b.x_min(), -20037508.3427892480);
        assert_eq!(b.x_max(), 0.0);
        assert_eq!(b.y_min(), 0.0);
        assert_eq!(b.y_max(), 20037508.3427892480);

        let peg = Peg::new(1, 1, 1).unwrap();
        let b = g.bbox_peg(peg);
        assert_eq!(b.x_min(), 0.0);
        assert_eq!(b.x_max(), 20037508.3427892480);
        assert_eq!(b.y_min(), -20037508.3427892480);
        assert_eq!(b.y_max(), 0.0);

        let peg = Peg::new(246, 368, 10).unwrap();
        let b = g.bbox_peg(peg);
        assert_eq!(b.x_min(), -10410111.756214727);
        assert_eq!(b.x_max(), -10370975.997732716);
        assert_eq!(b.y_min(), 5596413.462927466);
        assert_eq!(b.y_max(), 5635549.221409475);
    }

    #[test]
    fn test_peg_transform() {
        let g = MapGrid::default();
        let peg = Peg::new(0, 0, 0).unwrap();
        let t = g.transform_peg(peg);
        assert_eq!(
            Pt::new(0.0, 0.0),
            t * Pt::new(-20037508.3427892480, 20037508.3427892480)
        );
        assert_eq!(
            Pt::new(1.0, 1.0),
            t * Pt::new(20037508.3427892480, -20037508.3427892480)
        );

        let peg = Peg::new(0, 0, 1).unwrap();
        let t = g.transform_peg(peg);
        assert_eq!(
            Pt::new(0.0, 0.0),
            t * Pt::new(-20037508.3427892480, 20037508.3427892480)
        );
        assert_eq!(Pt::new(1.0, 1.0), t * Pt::new(0.0, 0.0));

        let peg = Peg::new(1, 1, 1).unwrap();
        let t = g.transform_peg(peg);
        assert_eq!(Pt::new(0.0, 0.0), t * Pt::new(0.0, 0.0));
        assert_eq!(
            Pt::new(1.0, 1.0),
            t * Pt::new(20037508.3427892480, -20037508.3427892480)
        );

        let peg = Peg::new(246, 368, 10).unwrap();
        let t = g.transform_peg(peg);
        assert_eq!(
            Pt::new(0.0, 0.0),
            t * Pt::new(-10410111.756214727, 5635549.221409475)
        );
        assert_eq!(
            Pt::new(1.0, 0.9999999999999716),
            t * Pt::new(-10370975.997732716, 5596413.462927466)
        );
    }
}
