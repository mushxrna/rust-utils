use crate::voxels::Voxel;

#[derive(Clone)]
pub struct SparseVoxelOctreeNode {
    data: Option<Voxel>,
    children: Option<Box<[SparseVoxelOctreeNode; 8]>>,
}

#[derive(Clone)]
pub struct SparseVoxelOctree {
    root: SparseVoxelOctreeNode,
    pub max_depth: u32,
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
    pub fn new(max_depth: u32) -> Self {
        Self {
            root: SparseVoxelOctreeNode::new(),
            max_depth,
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

    pub fn remove_voxel(&mut self, x: u32, y: u32, z: u32) {
        let size = 1u32 << self.max_depth;
        assert!(
            x < size && y < size && z < size,
            "Coordinates out of bounds"
        );

        Self::remove_voxel_recursive(&mut self.root, x, y, z, 0, 0, 0, size);
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
            let old_data = node.data;
            node.populate_children();

            if let Some(voxel) = old_data {
                if let Some(ref mut children) = node.children {
                    for child in children.iter_mut() {
                        child.data = Some(voxel);
                    }
                }
            }

            node.data = None;
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

            Self::try_collapse(node);
        }
    }

    fn remove_voxel_recursive(
        node: &mut SparseVoxelOctreeNode,
        x: u32,
        y: u32,
        z: u32,
        node_x: u32,
        node_y: u32,
        node_z: u32,
        node_size: u32,
    ) {
        if node_size == 1 {
            node.data = None;
            return;
        }

        if node.is_leaf() && node.data.is_none() {
            return;
        }

        if node.is_leaf() {
            let old_data = node.data;
            node.populate_children();

            if let Some(voxel) = old_data {
                if let Some(ref mut children) = node.children {
                    for child in children.iter_mut() {
                        child.data = Some(voxel);
                    }
                }
            }

            node.data = None;
        }

        let half_size = node_size / 2;
        let octant = Self::get_octant(x, y, z, node_x, node_y, node_z, half_size);

        let child_x = node_x + if (octant & 1) != 0 { half_size } else { 0 };
        let child_y = node_y + if (octant & 2) != 0 { half_size } else { 0 };
        let child_z = node_z + if (octant & 4) != 0 { half_size } else { 0 };

        if let Some(ref mut children) = node.children {
            Self::remove_voxel_recursive(
                &mut children[octant],
                x,
                y,
                z,
                child_x,
                child_y,
                child_z,
                half_size,
            );

            Self::try_collapse(node);
        }
    }

    fn try_collapse(node: &mut SparseVoxelOctreeNode) {
        if let Some(ref children) = node.children {
            let all_leaves = children.iter().all(|child| child.is_leaf());

            if !all_leaves {
                return;
            }

            let first_data = children[0].data;
            let all_identical = children.iter().all(|child| child.data == first_data);

            if all_identical {
                node.data = first_data;
                node.children = None;
            }
        }
    }

    pub fn get_octant(
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

    pub fn to_gpu_buffer(&self) -> Vec<SerialOctreeNode> {
        let mut buffer = Vec::new();
        self.serialize_node(&self.root, &mut buffer);
        buffer
    }

    fn serialize_node(
        &self,
        node: &SparseVoxelOctreeNode,
        buffer: &mut Vec<SerialOctreeNode>,
    ) -> u32 {
        let current_index = buffer.len() as u32;

        if node.is_leaf() {
            if let Some(data) = node.data {
                let r = (data.color.x.clamp(0.0, 1.0) * 255.0) as u32;
                let g = (data.color.y.clamp(0.0, 1.0) * 255.0) as u32;
                let b = (data.color.z.clamp(0.0, 1.0) * 255.0) as u32;
                let a = (data.color.w.clamp(0.0, 1.0) * 255.0) as u32;

                buffer.push(SerialOctreeNode {
                    color: (a << 24) | (b << 16) | (g << 8) | r,
                    child_ptr: 0, // Leaf nodes have no children
                });
            } else {
                buffer.push(SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                });
            }
            return current_index;
        }

        if let Some(ref children) = node.children {
            buffer.push(SerialOctreeNode {
                color: 0,
                child_ptr: 0,
            });

            let mut child_mask = 0u8;
            let mut non_empty_children = Vec::new();
            for i in 0..8 {
                if Self::has_data(&children[i]) {
                    child_mask |= 1u8 << i;
                    non_empty_children.push(&children[i]);
                }
            }

            if child_mask == 0 {
                buffer[current_index as usize] = SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                };
                return current_index;
            }

            let child_start_index = buffer.len() as u32;

            let num_children = non_empty_children.len();
            for _ in 0..num_children {
                buffer.push(SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                });
            }

            for (i, child) in non_empty_children.iter().enumerate() {
                let child_index = child_start_index + i as u32;

                if child.is_leaf() {
                    if let Some(data) = child.data {
                        let r = (data.color.x.clamp(0.0, 1.0) * 255.0) as u32;
                        let g = (data.color.y.clamp(0.0, 1.0) * 255.0) as u32;
                        let b = (data.color.z.clamp(0.0, 1.0) * 255.0) as u32;
                        let a = (data.color.w.clamp(0.0, 1.0) * 255.0) as u32;

                        buffer[child_index as usize] = SerialOctreeNode {
                            color: (a << 24) | (b << 16) | (g << 8) | r,
                            child_ptr: 0,
                        };
                    } else {
                        buffer[child_index as usize] = SerialOctreeNode {
                            color: 0,
                            child_ptr: 0,
                        };
                    }
                } else {
                    if let Some(ref grandchildren) = child.children {
                        let mut gc_mask = 0u8;
                        let mut gc_non_empty = Vec::new();
                        for j in 0..8 {
                            if Self::has_data(&grandchildren[j]) {
                                gc_mask |= 1u8 << j;
                                gc_non_empty.push(&grandchildren[j]);
                            }
                        }

                        let gc_start_index = buffer.len() as u32;

                        for _ in 0..gc_non_empty.len() {
                            buffer.push(SerialOctreeNode {
                                color: 0,
                                child_ptr: 0,
                            });
                        }

                        buffer[child_index as usize] = SerialOctreeNode {
                            color: gc_mask as u32,
                            child_ptr: if gc_mask != 0 { gc_start_index } else { 0 },
                        };

                        for (j, grandchild) in gc_non_empty.iter().enumerate() {
                            self.serialize_node_at(grandchild, buffer, gc_start_index + j as u32);
                        }
                    }
                }
            }

            buffer[current_index as usize] = SerialOctreeNode {
                color: child_mask as u32,
                child_ptr: child_start_index,
            };

            return current_index;
        }

        current_index
    }

    fn serialize_node_at(
        &self,
        node: &SparseVoxelOctreeNode,
        buffer: &mut Vec<SerialOctreeNode>,
        target_index: u32,
    ) {
        if node.is_leaf() {
            if let Some(data) = node.data {
                let r = (data.color.x.clamp(0.0, 1.0) * 255.0) as u32;
                let g = (data.color.y.clamp(0.0, 1.0) * 255.0) as u32;
                let b = (data.color.z.clamp(0.0, 1.0) * 255.0) as u32;
                let a = (data.color.w.clamp(0.0, 1.0) * 255.0) as u32;

                buffer[target_index as usize] = SerialOctreeNode {
                    color: (a << 24) | (b << 16) | (g << 8) | r,
                    child_ptr: 0,
                };
            } else {
                buffer[target_index as usize] = SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                };
            }
            return;
        }

        if let Some(ref children) = node.children {
            let mut child_mask = 0u8;
            let mut non_empty = Vec::new();
            for i in 0..8 {
                if Self::has_data(&children[i]) {
                    child_mask |= 1u8 << i;
                    non_empty.push(&children[i]);
                }
            }

            if child_mask == 0 {
                buffer[target_index as usize] = SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                };
                return;
            }

            let child_start = buffer.len() as u32;

            for _ in 0..non_empty.len() {
                buffer.push(SerialOctreeNode {
                    color: 0,
                    child_ptr: 0,
                });
            }

            buffer[target_index as usize] = SerialOctreeNode {
                color: child_mask as u32,
                child_ptr: child_start,
            };

            for (i, child) in non_empty.iter().enumerate() {
                self.serialize_node_at(child, buffer, child_start + i as u32);
            }
        }
    }

    fn has_data(node: &SparseVoxelOctreeNode) -> bool {
        if node.data.is_some() {
            return true;
        }

        if let Some(ref children) = node.children {
            return children.iter().any(|child| Self::has_data(child));
        }

        false
    }
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SerialOctreeNode {
    pub color: u32,     // RGBA8 for leaves, child mask (bits 0-7) for internal nodes
    pub child_ptr: u32, // 0 for leaves, buffer index of first child for internal nodes
}
