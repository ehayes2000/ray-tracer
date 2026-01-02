use super::types::Triangle;
use encase::{ShaderSize, ShaderType};

use crate::math::Vec3;

#[derive(Clone, Debug)]
enum BvhNode<T> {
    Node(InteriorNode<T>),
    Leaf(PrimitiveInfo<T>),
}

impl<T> BvhNode<T> {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum SplitAxis {
    X,
    Y,
    Z,
}

impl SplitAxis {
    pub fn as_index(&self) -> i32 {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

#[derive(Clone, Debug)]
struct InteriorNode<T> {
    pub left: Box<BvhNode<T>>,
    pub right: Box<BvhNode<T>>,
    pub split_axis: SplitAxis,
    pub bounds: Bounds,
}

#[derive(Clone, Debug)]
struct PrimitiveInfo<T> {
    pub bounds: Bounds,
    pub centroid: Vec3,
    pub primitive: T,
}

#[derive(Clone, Debug, ShaderType)]
pub struct Bounds {
    pub p_min: Vec3,
    pub p_max: Vec3,
}

impl Bounds {
    pub fn union(mut self, other: &Bounds) -> Self {
        self.p_min = Vec3(
            self.p_min.0.min(other.p_min.0),
            self.p_min.1.min(other.p_min.1),
            self.p_min.2.min(other.p_min.2),
        );
        self.p_max = Vec3(
            self.p_max.0.max(other.p_max.0),
            self.p_max.1.max(other.p_max.1),
            self.p_max.2.max(other.p_max.2),
        );
        self
    }

    pub fn union_pt(mut self, pt: Vec3) -> Self {
        self.p_min = Vec3(
            self.p_min.0.min(pt.0),
            self.p_min.1.min(pt.1),
            self.p_min.2.min(pt.2),
        );

        self.p_max = Vec3(
            self.p_max.0.max(pt.0),
            self.p_max.1.max(pt.1),
            self.p_max.2.max(pt.2),
        );
        self
    }

    pub fn small() -> Self {
        Self {
            p_min: Vec3::max(),
            p_max: Vec3::min(),
        }
    }

    pub fn width(&self) -> f32 {
        self.p_max.0 - self.p_min.0
    }

    pub fn height(&self) -> f32 {
        self.p_max.1 - self.p_min.1
    }

    pub fn depth(&self) -> f32 {
        self.p_max.2 - self.p_min.2
    }

    pub fn center(&self) -> Vec3 {
        self.p_min + Vec3(self.width() / 2.0, self.height() / 2.0, self.depth() / 2.0)
    }

    pub fn is_zero_sized(&self) -> bool {
        self.width() == 0. && self.height() == 0. && self.depth() == 0.
    }
}

fn build_bvh_recursive<T: Clone + std::fmt::Debug>(
    primitives: Vec<PrimitiveInfo<T>>,
) -> BvhNode<T> {
    let (centroid_bounds, primitive_bounds) = primitives
        .iter()
        .fold((Bounds::small(), Bounds::small()), |(bc, bp), primitive| {
            (bc.union_pt(primitive.centroid), bp.union(&primitive.bounds))
        });
    if primitives.len() == 1 {
        BvhNode::Leaf(primitives[0].to_owned())
    } else if centroid_bounds.is_zero_sized() {
        // TODO this is trash for > 2 primitives
        BvhNode::Node(InteriorNode {
            left: Box::new(BvhNode::Leaf(primitives[0].to_owned())),
            right: Box::new(BvhNode::Leaf(primitives[1].to_owned())),
            split_axis: SplitAxis::X,
            bounds: primitive_bounds,
        })
    } else {
        let (w, h, d) = (
            centroid_bounds.width(),
            centroid_bounds.height(),
            centroid_bounds.depth(),
        );
        let partition = if w >= h && w >= d {
            SplitAxis::X
        } else if h >= w && h >= d {
            SplitAxis::Y
        } else {
            SplitAxis::Z
        };
        let centroid = centroid_bounds.center();
        let left_partition = |e: &PrimitiveInfo<T>| match partition {
            SplitAxis::X => e.centroid.0 < centroid.0,
            SplitAxis::Y => e.centroid.1 < centroid.1,
            SplitAxis::Z => e.centroid.2 < centroid.2,
        };
        let half_len = primitives.len() / 2;
        let (left, right) = primitives.into_iter().fold(
            (Vec::with_capacity(half_len), Vec::with_capacity(half_len)),
            |(mut l, mut r), e| {
                if left_partition(&e) {
                    l.push(e);
                } else {
                    r.push(e);
                }
                (l, r)
            },
        );
        let left = build_bvh_recursive(left).boxed();
        let right = build_bvh_recursive(right).boxed();

        BvhNode::Node(InteriorNode {
            left,
            right,
            split_axis: partition,
            bounds: primitive_bounds,
        })
    }
}

#[derive(ShaderType, Debug)]
pub struct BvhShaderNode<T: ShaderType + ShaderSize> {
    // 0 - interior | 1 - leaf
    pub kind: u32,
    pub axis: i32,
    pub second_child_offset: i32,
    pub bounds: Bounds,
    // TODO cancer
    pub centroid: Vec3,
    pub primitive: T,
}

#[derive(Default, Debug)]
pub struct BvhShaderArray<T: ShaderType + ShaderSize>(pub Vec<BvhShaderNode<T>>);

impl<T> BvhNode<T>
where
    T: ShaderType + ShaderSize + Default + Clone,
{
    pub fn flatten(self, buf: &mut BvhShaderArray<T>) -> usize {
        let linear_node = match self.clone() {
            BvhNode::Leaf(leaf) => BvhShaderNode {
                axis: 0,
                bounds: leaf.bounds,
                centroid: leaf.centroid,
                kind: 1,
                primitive: leaf.primitive,
                second_child_offset: 0,
            },
            BvhNode::Node(node) => BvhShaderNode {
                axis: node.split_axis.as_index(),
                centroid: node.bounds.center(),
                bounds: node.bounds,
                kind: 0,
                primitive: T::default(),
                second_child_offset: 0,
            },
        };
        let offset = buf.0.len();
        buf.0.push(linear_node);

        if let Self::Node(node) = self {
            node.left.flatten(buf);
            buf.0[offset].second_child_offset = node.right.flatten(buf) as i32;
        }
        offset
    }
}

// impl<T> From<BvhNode<T>> for BvhShaderArray<T>
// where
//     T: ShaderType + ShaderSize + Default + Clone,
// {
//     fn from(value: BvhNode<T>) -> Self {
//         fn depth_traverse<T>(v: BvhNode<T>, nodes: &mut Vec<BvhShaderNode<T>>)
//         where
//             T: ShaderType + ShaderSize + Default + Clone,
//         {
//             let lr = if let BvhNode::Node(ref node) = v {
//                 Some((*node.left.to_owned(), *node.right.to_owned()))
//             } else {
//                 None
//             };
//             nodes.push(v.into());
//             if let Some((l, r)) = lr {
//                 depth_traverse(l, nodes);
//                 depth_traverse(r, nodes);
//             }
//         }
//         let mut nodes = Vec::new();
//         depth_traverse(value, &mut nodes);
//         Self(nodes)
//     }
// }

impl From<Triangle> for PrimitiveInfo<Triangle> {
    fn from(value: Triangle) -> Self {
        let bounds = Bounds::small()
            .union_pt(value.a)
            .union_pt(value.b)
            .union_pt(value.c);
        let centroid = bounds.center();
        Self {
            bounds,
            centroid,
            primitive: value,
        }
    }
}

pub fn build_shader_bvh<T>(primitives: Vec<T>) -> BvhShaderArray<T>
where
    T: ShaderSize + ShaderType + Into<PrimitiveInfo<T>> + Clone + Default + std::fmt::Debug,
{
    let primitives: Vec<_> = primitives.into_iter().map(Into::into).collect();
    let bvh = build_bvh_recursive(primitives);
    let mut buf = BvhShaderArray::default();
    bvh.flatten(&mut buf);
    buf
}
