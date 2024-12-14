use crate::vectors::Vec2D;

#[must_use]
pub fn get_adjecent_directions() -> [Vec2D; 4] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
    ]
}

#[must_use]
pub fn get_adjecent_directions_including_self() -> [Vec2D; 5] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
        Vec2D::new(0, 0),
    ]
}

#[must_use]
pub fn get_all_surrounding_directions() -> [Vec2D; 8] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
        Vec2D::new(-1, -1),
        Vec2D::new(1, -1),
        Vec2D::new(1, 1),
        Vec2D::new(-1, 1),
    ]
}

#[deprecated(since = "0.1.0", note = "please use `Vec2D.is_in_bounds` instead")]
#[must_use]
pub fn check_in_bounds(point: &Vec2D, width: u32, height: u32) -> bool {
    point.x >= 0
        && point.x < width.try_into().expect("Width not in bounds of i32")
        && point.y >= 0
        && point.y < height.try_into().expect("Height not in bounds of i32")
}
