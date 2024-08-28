use cgmath::Vector3;
use rand::Rng;

#[allow(dead_code)]
pub(crate) fn generate_random_colors(n: u32) -> Vec<Vector3<f32>> {
    let mut colors = vec![];
    let mut rng = rand::thread_rng();
    let min = 0.2;
    let max = 1.0;
    for _ in 0..n {
        let c = Vector3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max));
        colors.push(c);
    }
    colors
}

#[allow(dead_code)]
pub fn create_grid_positions(num_cols: u32, num_rows: u32, spacing: f32, variance: Option<f32>) -> Vec<Vector3<f32>> {
    let mut positions = Vec::new();
    let mut rng = rand::thread_rng();
    let var = variance.unwrap_or(0.0);

    // Calculate the total width and height of the grid
    let width = (num_cols as f32 - 1.0) * spacing;
    let height = (num_rows as f32 - 1.0) * spacing;

    // Calculate the starting positions
    let start_x = -width / 2.0;
    let start_y = -height / 2.0;

    // Generate the grid positions
    for row in 0..num_rows {
        for col in 0..num_cols {
            let mut x = start_x + col as f32 * spacing;
            let y = start_y + row as f32 * spacing;
            if var != 0.0 {
                x += rng.gen_range(-var..var);
            }
            positions.push(Vector3::new(x, y, 0.0));
        }
    }
    positions
}

#[allow(dead_code)]
pub(crate) fn generate_random_radii(num_instances: u32, base_radius: f32, variance: f32) -> Vec<f32> {
    if variance == 0.0 {
        return vec![base_radius; num_instances as usize];
    }
    let mut rng = rand::thread_rng();
    let mut radii = Vec::new();
    for _ in 0..num_instances {
        let radius = base_radius + rng.gen_range(-variance.abs()..variance.abs());
        radii.push(radius);
    }
    return radii;
}
