use crate::vec3::Vec3;

struct Face {
    vertices: Vec<Vec3>,
}

impl Face {
    pub fn triangles(&self) -> Vec<(Vec3, Vec3, Vec3)> {
        self.vertices
            .windows(2)
            .skip(1)
            .map(|w| (self.vertices[0], w[0], w[1]))
            .collect()
    }
}

pub struct Model {
    _vertices: Vec<Vec3>,
    faces: Vec<Face>,
}

impl Model {
    pub fn parse(data: &str) -> Self {
        let mut vertices: Vec<Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();
        for line in data.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts.as_slice() {
                ["v", x, y, z, ..] => {
                    let x: f32 = x.parse().unwrap();
                    let y: f32 = y.parse().unwrap();
                    let z: f32 = z.parse().unwrap();
                    vertices.push(Vec3 { x, y, z });
                }
                ["f", rest @ ..] => {
                    let indices: Vec<usize> = rest
                        .iter()
                        .map(|s| s.split('/').next().unwrap().parse::<usize>().unwrap())
                        .collect();

                    let face = Face {
                        vertices: indices.iter().map(|i| vertices[i - 1]).collect(),
                    };
                    faces.push(face);
                }
                _ => {}
            }
        }

        Self {
            _vertices: vertices,
            faces,
        }
    }

    pub fn triangles(&self) -> Vec<(Vec3, Vec3, Vec3)> {
        self.faces.iter().flat_map(|f| f.triangles()).collect()
    }
}
