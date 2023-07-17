use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let orig = Vec3::new(0.0, 0.0, 0.0);
        let horiz = Vec3::new(viewport_width, 0.0, 0.0);
        let vert = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - horiz / 2.0 - vert / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: orig,
            horizontal: horiz,
            vertical: vert,
            lower_left_corner: llc,
        }
    }
}
