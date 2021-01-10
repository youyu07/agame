use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::PrimitiveTopology,
    },
};

pub struct Cube {
    extend: Vec3,
    segment: Vec3,
}

impl Default for Cube
{
    fn default() -> Self {
        Self{extend:Vec3::one(), segment:Vec3::one()}
    }
}

#[allow(unused_assignments)]
impl From<Cube> for Mesh {
    fn from(cube: Cube) -> Self {
        let mut vertice = vec![];
        let mut normals = vec![];
        let mut uvs = vec![];
        let mut indices = vec![];

        let mut index_offset = 0u16;
        macro_rules! plane {
            ($u:tt, $v:tt, $w:tt,$udir:expr, $vdir:expr, $width:expr, $height:expr, $depth:expr, $gridx:expr, $gridy:expr) => {{
                let segment_width = $width / $gridx as f32;
                let segment_height = $height / $gridy as f32;

                let mut vector = Vec3::zero();

                let mut num_vertex = 0u16;
                for iy in 0..($gridy + 1) {
                    let y = iy as f32 * segment_height - $height * 0.5;
                    for ix in 0..($gridx + 1) {
                        let x = ix as f32 * segment_width - $width * 0.5;

                        vector.$u = x * $udir;
                        vector.$v = y * $vdir;
                        vector.$w = $depth * 0.5;

                        vertice.push([vector.x, vector.y, vector.z]);

                        vector.$u = 0.0;
                        vector.$v = 0.0;
                        vector.$w = if $depth > 0.0 { 1.0 } else { -1.0 };

                        normals.push([vector.x, vector.y, vector.z]);
                        uvs.push([ix as f32 / $gridx as f32, 1.0 - (iy as f32 / $gridy as f32)]);

                        num_vertex += 1;
                    }
                }

                for iy in 0..$gridy {
                    for ix in 0..$gridx {
                        let a = index_offset + (ix + ($gridx + 1) * iy) as u16;
                        let b = index_offset + (ix + ($gridx + 1) * (iy + 1)) as u16;
                        let c = index_offset + ((ix + 1) + ($gridx + 1) * (iy + 1)) as u16;
                        let d = index_offset + ((ix + 1) + ($gridx + 1) * iy) as u16;

                        let mut arr = vec![a, b, d, b, c, d];
                        indices.append(&mut arr);
                    }
                }

                index_offset += num_vertex;
            }};
        }

        let extend = cube.extend;

        let segment_x = (cube.segment.x.floor() as u32).max(1);
        let segment_y = (cube.segment.y.floor() as u32).max(1);
        let segment_z = (cube.segment.z.floor() as u32).max(1);

        plane! {z,y,x,-1.0,-1.0,extend.z,extend.y, extend.x,segment_z,segment_y};
        plane! {z,y,x, 1.0,-1.0,extend.z,extend.y,-extend.x,segment_z,segment_y};
        plane! {x,z,y, 1.0, 1.0,extend.x,extend.z, extend.y,segment_x,segment_z};
        plane! {x,z,y, 1.0,-1.0,extend.x,extend.z,-extend.y,segment_x,segment_z};
        plane! {x,y,z, 1.0,-1.0,extend.x,extend.y, extend.z,segment_x,segment_y};
        plane! {x,y,z,-1.0,-1.0,extend.x,extend.y,-extend.z,segment_x,segment_y};

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float3(vertice),
        );
        mesh.set_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float3(normals),
        );
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float2(uvs));
        mesh.set_indices(Some(Indices::U16(indices)));

        mesh
    }
}
