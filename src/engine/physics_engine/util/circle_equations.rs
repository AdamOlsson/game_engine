/// Returns the left-, right-, top- and bottom-most points of a rotated rectangle
pub fn cardinals(center: [f32;3], radius:f32) -> [[f32;3];4]{
    let west = [center[0] - radius, center[1], 0.0];
    let east = [center[0] + radius, center[1], 0.0];
    let north = [center[0], center[1] + radius, 0.0];
    let south = [center[0], center[1] - radius, 0.0];

    return [west, east, north, south];
}
