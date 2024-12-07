use std::{collections::HashMap, f64::consts::PI};

/// Calculates the difference between two vectors with periodic boundary conditions
///
/// # Arguments
///
/// * `vector1` - First vector
/// * `vector2` - Second vector
/// * `dimensions` - Dimensions of the system xhi, yhi, (zhi if 3D)
///
/// # Panics
///
/// Panics if the sizes of the vectors do not match the dimensions
///
/// # Returns
///
/// The difference vector
pub fn pbc_vector(vector1: &[f64], vector2: &[f64], dimensions: &[f64]) -> Vec<f64> {
    if vector1.len() != vector2.len() || vector1.len() != dimensions.len() {
        panic!(
            "Vector sizes do not match in PBC function: {} {} {}",
            vector1.len(),
            vector2.len(),
            dimensions.len()
        );
    }
    vector1
        .iter()
        .zip(vector2.iter())
        .zip(dimensions.iter())
        .map(|((&v1, &v2), &dim)| {
            let mut difference = v2 - v1;
            let half_dimension = dim / 2.0;
            if difference > half_dimension {
                difference -= dim;
            } else if difference < -half_dimension {
                difference += dim;
            }
            difference
        })
        .collect()
}

/// Normalises the values of a map in place
///
/// # Arguments
///
/// * `map` - The map to normalise
pub fn normalise_map(map: &mut HashMap<i32, f64>) {
    let sum: f64 = map.values().sum();
    for value in map.values_mut() {
        *value /= sum;
    }
}

/// Prints a nested map to the console
///
/// # Arguments
///
/// * `map` - The map to print
pub fn show_nested_map(map: &HashMap<i32, HashMap<i32, f64>>) {
    for (key, inner_map) in map {
        print!("{}: {{", key);
        for (inner_key, value) in inner_map {
            print!("{}: {}, ", inner_key, value);
        }
        println!("}}");
    }
}

/// Calculates angle between two vectors
///
/// # Arguments
///
/// * `vector1` - First vector
/// * `vector2` - Second vector
///
/// # Returns
///
/// Angle between the two vectors in radians
pub fn get_clockwise_angle_between_vectors(vector1: &[f64], vector2: &[f64]) -> f64 {
    let dot_product = vector1[0] * vector2[0] + vector1[1] * vector2[1];
    let magnitude_product = (vector1[0].powi(2) + vector1[1].powi(2)).sqrt()
        * (vector2[0].powi(2) + vector2[1].powi(2)).sqrt();
    let mut angle = (dot_product / magnitude_product).acos();

    // If the cross product is positive, subtract the angle from 2π to get the angle in the range [π, 2π]
    if vector1[0] * vector2[1] - vector1[1] * vector2[0] > 0.0 {
        angle = 2.0 * PI - angle;
    }

    angle
}

/// Get the clockwise angle of the vector between two nodes relative to the x axis taking into account periodic boundary conditions
///
/// # Arguments
///
/// * `coord1` - Coordinates of the first node
/// * `coord2` - Coordinates of the second node
/// * `dimensions` - Dimensions of the periodic box
///
/// # Returns
///
/// Angle of the vector between the two nodes relative to the x axis in radians
pub fn get_clockwise_angle(coord1: &[f64], coord2: &[f64], dimensions: &[f64]) -> f64 {
    let periodic_vector = pbc_vector(coord1, coord2, dimensions);
    let mut angle = periodic_vector[1].atan2(periodic_vector[0]);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    2.0 * PI - angle
}

/// Get the clockwise angle of a point relative to the x axis
///
/// # Arguments
///
/// * `point` - Coordinates of the point
///
/// # Returns
///
/// Clockwise angle of the point relative to the x axis in radians
pub fn get_clockwise_angle_point(point: &[f64]) -> f64 {
    let mut angle = point[1].atan2(point[0]);
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    2.0 * PI - angle
}

/// Sorts a vector of coordinates in clockwise order relative to [0, 0]
///
/// # Arguments
///
/// * `coords` - Vector of coordinates to sort
pub fn sort_coordinates_clockwise(coords: &mut Vec<Vec<f64>>) {
    coords.sort_by(|a, b| {
        let angle_a = get_clockwise_angle_point(a);
        let angle_b = get_clockwise_angle_point(b);
        angle_a.partial_cmp(&angle_b).unwrap()
    });
}

/// Calculates the area of a polygon given its vertices
///
/// # Arguments
///
/// * `vertices` - Vertices of the polygon
///
/// # Returns
///
/// Area of the polygon
pub fn calculate_polygon_area(vertices: &mut Vec<Vec<f64>>) -> f64 {
    if vertices.is_empty() {
        return 0.0;
    }
    sort_coordinates_clockwise(vertices);
    let mut area = 0.0;
    for i in 0..vertices.len() - 1 {
        area += vertices[i][0] * vertices[i + 1][1] - vertices[i + 1][0] * vertices[i][1];
    }
    area += vertices.last().unwrap()[0] * vertices.first().unwrap()[1]
        - vertices.first().unwrap()[0] * vertices.last().unwrap()[1];
    area /= 2.0;
    area.abs()
}
