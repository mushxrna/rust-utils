pub type Point = crate::vectors::Vec3<f32>;

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

    pub fn to_pod_data(self) -> Vec<crate::wgpu_helpers::pod_types::Vertex> {
        vec![
            self.origin.into_pod_vertex(),
            self.direction.into_pod_vertex(),
            self.intersection.into_pod_vertex(),
        ]
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

    pub fn to_pod_vertices(self) -> Vec<crate::wgpu_helpers::pod_types::Vertex> {
        vec![
            self.p1.into_pod_vertex(),
            self.p2.into_pod_vertex(),
            self.p3.into_pod_vertex(),
        ]
    }
}
