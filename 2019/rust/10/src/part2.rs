use crate::{interference, subtract_points, Point};
use std::cmp::Ordering;
use std::collections::HashSet;

/// What is the 200th asteroid to be zapped? See Day 10 for details.
pub fn asteroid_200(asteroids: &HashSet<Point>, station: Point) -> Point {
    let mut candidates: Vec<AsteroidInfo> = asteroids
        .iter()
        .cloned()
        .filter(|&point| point != station)
        .map(|point| AsteroidInfo {
            point,
            layer: interference(asteroids, station, point),
            angle: Angle::new(point, station),
        })
        .collect();

    candidates.sort_by_key(|info| (info.layer, info.angle));

    candidates[199].point
}

/// Helper struct for sorting asteroids.
struct AsteroidInfo {
    /// The location of an asteroid other than `station`.
    point: Point,
    /// How much is the interference between `station` and `point`?
    layer: usize,
    /// A representation of the angle from `station` to `point`.
    angle: Angle,
}

/// The angle between two points, stored as (dx, dy).
#[derive(Debug, Clone, Copy)]
struct Angle(Point);

impl Angle {
    fn new(a: Point, b: Point) -> Self {
        Self(subtract_points(b, a))
    }

    /// Compares clockwise from noon (accounting for inverted y-axis).
    fn sort_key(self) -> (bool, f64) {
        let (dx, dy) = self.0;
        let slope = dy as f64 / dx as f64;

        let down = dx == 0 && dy > 0;
        let flipside = dx < 0 || down;

        // Note that we *don't* flip slope, since the y-axis is already inverted.
        (flipside, slope)
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.sort_key() == other.sort_key()
    }
}

impl Eq for Angle {}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.sort_key().partial_cmp(&other.sort_key())
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        // This is safe because the slope is never NaN.
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_sort() {
        let origin = (0, 0);

        let n = (0, -1);
        let ne = (1, -1);
        let e = (1, 0);
        let se = (1, 1);
        let s = (0, 1);
        let sw = (-1, 1);
        let w = (-1, 0);
        let nw = (-1, -1);

        let dirs = vec![n, ne, e, se, s, sw, w, nw];
        let angs: Vec<_> = dirs.into_iter().map(|d| Angle::new(origin, d)).collect();

        let mut sorted = angs.clone();
        sorted.sort();

        assert_eq!(angs, sorted);
    }

    #[test]
    fn north() {
        let origin = (0, 0);
        let n = (0, -1);
        assert_eq!((false, f64::NEG_INFINITY), Angle::new(origin, n).sort_key());
    }

    #[test]
    fn south() {
        let origin = (0, 0);
        let s = (0, 1);
        assert_eq!((true, f64::NEG_INFINITY), Angle::new(origin, s).sort_key());
    }
}
