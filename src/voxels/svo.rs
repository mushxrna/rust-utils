use crate::voxels::Voxel;

pub struct SparseVoxelOctreeNode {
    data: Option<Voxel>,
    children: Option<Box<[SparseVoxelOctreeNode; 8]>>,
}

pub struct SparseVoxelOctree {
    root: SparseVoxelOctreeNode,
    max_depth: u32,
}

impl SparseVoxelOctreeNode {
    pub fn new() -> Self {
        Self {
            data: None,
            children: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }

    pub fn populate_children(&mut self) {
        self.children = Some(Box::new([
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
            SparseVoxelOctreeNode::new(),
        ]))
    }
}

impl SparseVoxelOctree {
    pub fn new() -> Self {
        Self {
            root: SparseVoxelOctreeNode::new(),
            max_depth: 5,
        }
    }

    pub fn set_voxel(&mut self, x: u32, y: u32, z: u32, data: Voxel) {
        let size = 1u32 << self.max_depth;
        assert!(
            x < size && y < size && z < size,
            "Coordinates out of bounds"
        );

        Self::set_voxel_recursive(&mut self.root, x, y, z, data, 0, 0, 0, size);
    }

    fn set_voxel_recursive(
        node: &mut SparseVoxelOctreeNode,
        x: u32,
        y: u32,
        z: u32,
        data: Voxel,
        node_x: u32,
        node_y: u32,
        node_z: u32,
        node_size: u32,
    ) {
        if node_size == 1 {
            node.data = Some(data);
            node.children = None;
            return;
        }

        if node.is_leaf() {
            node.populate_children();
            match node.data {
                Some(data) => {
                    if let Some(ref mut children) = node.children {
                        for child in children.iter_mut() {
                            child.data = Some(data);
                        }
                    }
                }
                None => {}
            }
        }

        let half_size = node_size / 2;
        let octant = Self::get_octant(x, y, z, node_x, node_y, node_z, half_size);

        let child_x = node_x + if (octant & 1) != 0 { half_size } else { 0 };
        let child_y = node_y + if (octant & 2) != 0 { half_size } else { 0 };
        let child_z = node_z + if (octant & 4) != 0 { half_size } else { 0 };

        if let Some(ref mut children) = node.children {
            Self::set_voxel_recursive(
                &mut children[octant],
                x,
                y,
                z,
                data,
                child_x,
                child_y,
                child_z,
                half_size,
            );
        }
    }

    fn get_octant(
        x: u32,
        y: u32,
        z: u32,
        node_x: u32,
        node_y: u32,
        node_z: u32,
        half_size: u32,
    ) -> usize {
        let mut octant = 0;
        if x >= node_x + half_size {
            octant |= 1;
        }
        if y >= node_y + half_size {
            octant |= 2;
        }
        if z >= node_z + half_size {
            octant |= 4;
        }
        octant
    }

    pub fn get_voxel(&self, x: u32, y: u32, z: u32) -> Option<Voxel> {
        let size = 1u32 << self.max_depth;
        Self::get_voxel_recursive(&self.root, x, y, z, 0, 0, 0, size)
    }

    fn get_voxel_recursive(
        node: &SparseVoxelOctreeNode,
        x: u32,
        y: u32,
        z: u32,
        node_x: u32,
        node_y: u32,
        node_z: u32,
        node_size: u32,
    ) -> Option<Voxel> {
        if node_size == 1 {
            return node.data;
        }

        if node.is_leaf() {
            return node.data; // Uniform region
        }

        let half_size = node_size / 2;
        let octant = Self::get_octant(x, y, z, node_x, node_y, node_z, half_size);

        let child_x = node_x + if (octant & 1) != 0 { half_size } else { 0 };
        let child_y = node_y + if (octant & 2) != 0 { half_size } else { 0 };
        let child_z = node_z + if (octant & 4) != 0 { half_size } else { 0 };

        if let Some(ref children) = node.children {
            Self::get_voxel_recursive(
                &children[octant],
                x,
                y,
                z,
                child_x,
                child_y,
                child_z,
                half_size,
            )
        } else {
            None
        }
    }
}
