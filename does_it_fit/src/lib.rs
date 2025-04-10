pub mod areas_volumes;
pub use areas_volumes::*;

pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let max_size = x * y;
    let size;
    match objects {
        GeometricalShapes::Square => size = square_area(a) as f64,
        GeometricalShapes::Circle => size = circle_area(a),
        GeometricalShapes::Rectangle => size = rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => size = triangle_area(a, b),
    }
    times as f64 * size <= max_size as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let max_size = x * y * z;
    let size;
    match objects {
        GeometricalVolumes::Cube => size = cube_volume(a) as f64,
        GeometricalVolumes::Sphere => size = sphere_volume(a),
        GeometricalVolumes::Parallelepiped => size = parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Pyramid => size = triangular_pyramid_volume(triangle_area(a, b), c),
        GeometricalVolumes::Cone => size = cone_volume(a, b),
    }
    times as f64 * size <= max_size as f64
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_volumes_shapes() {
        assert_eq!(true, area_fit(2, 5, GeometricalShapes::Circle, 0, 2, 1));
        assert_eq!(true, area_fit(2, 2, GeometricalShapes::Rectangle, 0, 6, 10));
        assert_eq!(
            true,
            volume_fit(2, 5, 3, GeometricalVolumes::Cone, 0, 1, 1, 1)
        );
        assert_eq!(
            true,
            volume_fit(3, 5, 7, GeometricalVolumes::Parallelepiped, 0, 2, 6, 3)
        );
    }

    #[test]
    fn equal_size() {
        assert_eq!(true, area_fit(2, 5, GeometricalShapes::Square, 1, 2, 5));
        assert_eq!(
            true,
            volume_fit(3, 1, 4, GeometricalVolumes::Cube, 1, 1, 3, 4)
        );
    }

    #[test]
    fn all_shapes() {
        assert_eq!(false, area_fit(3, 5, GeometricalShapes::Circle, 2, 2, 0));
        assert_eq!(true, area_fit(8, 6, GeometricalShapes::Triangle, 5, 5, 2));
        assert_eq!(true, area_fit(7, 3, GeometricalShapes::Rectangle, 2, 2, 4));
        assert_eq!(true, area_fit(5, 5, GeometricalShapes::Square, 1, 2, 4));
    }

    #[test]
    fn all_volumes() {
        assert_eq!(
            true,
            volume_fit(5, 6, 3, GeometricalVolumes::Cube, 2, 3, 3, 4)
        );
        assert_eq!(
            false,
            volume_fit(7, 4, 4, GeometricalVolumes::Cone, 1, 8, 2, 0)
        );
        assert_eq!(
            true,
            volume_fit(2, 5, 3, GeometricalVolumes::Sphere, 1, 1, 1, 1)
        );
        assert_eq!(
            false,
            volume_fit(2, 5, 3, GeometricalVolumes::Parallelepiped, 31, 1, 1, 1)
        );
        assert_eq!(
            true,
            volume_fit(7, 5, 3, GeometricalVolumes::Pyramid, 3, 3, 2, 1)
        );
    }
}