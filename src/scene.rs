use glm::Vector3;

pub struct Sphere {
    pub position: Vector3<f32>,
    pub radius: f32,
    pub albedo: Vector3<f32>,
}

impl Sphere {
    pub fn _new() -> Self {
        Self {
            position: glm::to_vec3(0.0),
            radius: 0.5,
            albedo: glm::to_vec3(1.0),
        }
    }
}

pub struct Scene {
    pub spheres: Vec<Sphere>,
}

impl Scene {
    pub fn _new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }
}

pub fn create_test_scene() -> Scene {
    Scene {
        spheres: vec![
            Sphere {
                position: glm::to_vec3(0.0),
                radius: 0.5,
                albedo: glm::vec3(1.0, 0.0, 1.0),
            },
            Sphere {
                position: glm::vec3(1.0, 0.0, -5.0),
                radius: 1.5,
                albedo: glm::vec3(0.2, 0.3, 1.0),
            },
        ],
    }
}

pub fn create_test_scene_2() -> Scene {
    Scene {
        spheres: vec![
            Sphere {
                position: glm::to_vec3(0.0),
                radius: 0.5,
                albedo: glm::vec3(1.0, 0.0, 1.0),
            },
            Sphere {
                position: glm::vec3(0.0, -9.2, 0.0),
                radius: 8.6,
                albedo: glm::vec3(0.2, 0.3, 1.0),
            },
        ],
    }
}

pub fn create_test_scene_3() -> Scene {
    Scene {
        spheres: vec![
            Sphere {
                position: glm::to_vec3(0.0),
                radius: 1.0,
                albedo: glm::vec3(1.0, 0.0, 1.0),
            },
            Sphere {
                position: glm::vec3(0.0, -101.0, 0.0),
                radius: 100.0,
                albedo: glm::vec3(0.2, 0.3, 1.0),
            },
        ],
    }
}
