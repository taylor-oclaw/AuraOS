extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub extern "C" fn mesh_cross_border_check_init() {
    // Initialization logic for the module
}

pub extern "C" fn mesh_cross_border_check_exit() {
    // Cleanup logic for the module
}

pub struct MeshCrossBorderCheck {
    border_points: Vec<(i32, i32)>,
    allowed_areas: Vec<Vec<(i32, i32)>>,
}

impl MeshCrossBorderCheck {
    pub fn new(border_points: Vec<(i32, i32)>, allowed_areas: Vec<Vec<(i32, i32)>>) -> Self {
        MeshCrossBorderCheck {
            border_points,
            allowed_areas,
        }
    }

    pub fn add_border_point(&mut self, point: (i32, i32)) {
        self.border_points.push(point);
    }

    pub fn remove_border_point(&mut self, index: usize) -> Option<(i32, i32)> {
        if index < self.border_points.len() {
            Some(self.border_points.remove(index))
        } else {
            None
        }
    }

    pub fn add_allowed_area(&mut self, area: Vec<(i32, i32)>) {
        self.allowed_areas.push(area);
    }

    pub fn is_point_inside_allowed_area(&self, point: (i32, i32)) -> bool {
        for area in &self.allowed_areas {
            if Self::is_point_in_polygon(point, area) {
                return true;
            }
        }
        false
    }

    fn is_point_in_polygon(point: (i32, i32), polygon: &[(i32, i32)]) -> bool {
        let mut inside = false;
        let n = polygon.len();
        for i in 0..n {
            let j = (i + 1) % n;
            if ((polygon[i].1 > point.1) != (polygon[j].1 > point.1)) &&
               (point.0 < (polygon[j].0 - polygon[i].0) * (point.1 - polygon[i].1) / (polygon[j].1 - polygon[i].1) + polygon[i].0) {
                inside = !inside;
            }
        }
        inside
    }

    pub fn check_cross_border(&self, path: Vec<(i32, i32)>) -> bool {
        for point in &path {
            if self.border_points.contains(point) && !self.is_point_inside_allowed_area(*point) {
                return true;
            }
        }
        false
    }
}
