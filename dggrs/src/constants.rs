use geo::{Coord, Rect};

pub fn whole_earth_bbox() -> Rect<f64> {
    Rect::new(
        Coord {
            x: -180.0,
            y: -90.0,
        },
        Coord { x: 180.0, y: 90.0 },
    )
}
