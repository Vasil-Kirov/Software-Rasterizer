
use crate::{vmath::*};

pub struct Model {
    pub verts: Vec<Vertex>,
    pub indices: Vec<u32>,
}

fn parse_vec2(line: &str) -> Vec2 {
    let mut v = Vec2::splat(0.0);
    let mut at = 0;
    line.split_whitespace().for_each(|val| {
        v[at] = val.parse().unwrap_or(0.0f32);
        at += 1;
    });

    v
}

fn parse_vec3(line: &str) -> Vec3 {
    let mut v = Vec3::splat(0.0);
    let mut at = 0;
    line.split_whitespace().for_each(|val| {
        v[at] = val.parse().unwrap_or(0.0f32);
        at += 1;
    });

    v
}

impl Model {
    pub fn load_from_data(wavefront: &str) -> Result<Self, String> {
        let mut r = Self { verts: vec!(), indices: vec!() };
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::<usize>::new();
        for line in wavefront.lines() {
            if let Some(rest) = line.strip_prefix("v ") {
                positions.push(parse_vec3(rest));
            }
            else if let Some(rest) = line.strip_prefix("vn ") {
                normals.push(parse_vec3(rest).normalize());
            }
            else if let Some(rest) = line.strip_prefix("vt ") {
                tex_coords.push(parse_vec2(rest));
            }
            else if let Some(rest) = line.strip_prefix("f ") {
                let mut face_idxs = vec![];
                for group in rest.split_whitespace() {
                    let mut group_values = Vec::<usize>::new();
                    for str_num in group.split('/') {
                        let parsed = str_num.parse();
                        match parsed {
                            Ok(v) => group_values.push(v),
                            Err(_) => return Err("Failed to parse face value".to_string()),
                        }
                    }
                    face_idxs.push(group_values);
                }
                indices.extend_from_slice(&face_idxs[0]);
                indices.extend_from_slice(&face_idxs[1]);
                indices.extend_from_slice(&face_idxs[2]);
                indices.extend_from_slice(&face_idxs[0]);
                indices.extend_from_slice(&face_idxs[2]);
                indices.extend_from_slice(&face_idxs[3]);
            }
        }

        let face_count = indices.len() / 3;

        for fi in 0..face_count {
            let face = fi * 3;
            let p_i = indices[face+0] - 1;
            let t_i = indices[face+1] - 1;
            let n_i = indices[face+2] - 1;

            let vert = Vertex::new(positions[p_i], normals[n_i], tex_coords[t_i]);
            let find_result = r.verts.iter().position(|it| *it == vert);
            let new_idx = match find_result {
                Some(i) => i,
                None => { 
                    r.verts.push(vert);
                    r.verts.len() - 1
                }
            };
            r.indices.push(new_idx as u32);
        }

        Ok(r)
    }
}

