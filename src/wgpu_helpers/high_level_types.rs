use crate::generics::Byteable;
use crate::generics::VectorType;

pub type Point = crate::vectors::Vec3<f32>;
pub type Size = crate::vectors::Vec2<u32>;

#[derive(Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
    intersection: Point,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Point::new(0.0, 0.0, 0.0),
            intersection: Point::new(0.0, 0.0, 0.0),
        }
    }

    pub fn to_raw_bytes(self) -> Vec<u8> {
        vec![
            self.origin.to_collection().to_raw_bytes(),
            self.direction.to_collection().to_raw_bytes(),
            self.intersection.to_collection().to_raw_bytes(),
        ]
        .concat()
    }
}

#[derive(Clone, Copy)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    centroid: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        Self {
            p1,
            p2,
            p3,
            centroid: (p1 + p2 + p3) * Point::new(0.333, 0.333, 0.333),
        }
    }

    pub fn to_raw_bytes(self) -> Vec<u8> {
        vec![
            self.p1.to_collection().to_raw_bytes(),
            self.p2.to_collection().to_raw_bytes(),
            self.p3.to_collection().to_raw_bytes(),
        ]
        .concat()
    }
}
