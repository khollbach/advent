use crate::{interference, subtract_points, Point};
use std::cmp::Ordering;
use std::collections::HashSet;

/// What is the nth asteroid to be zapped? See Day 10 for details.
pub fn asteroid_n(asteroids: &HashSet<Point>, station: Point, n: usize) -> Point {
    assert_ne!(n, 0);

    let mut candidates: Vec<AsteroidInfo> = asteroids
        .iter()
        .cloned()
        .filter(|&point| point != station)
        .map(|point| AsteroidInfo {
            point,
            layer: interference(asteroids, station, point),
            angle: Angle::new(station, point),
        })
        .collect();

    candidates.sort_by_key(|info| (info.layer, info.angle));

    candidates[n - 1].point
}

/// Helper struct for sorting asteroids.
#[derive(Debug, Clone, Copy)]
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
    /// The direction matters! The angle is flipped 180* if you swap a and b.
    fn new(a: Point, b: Point) -> Self {
        assert_ne!(a, b);
        Self(subtract_points(b, a))
    }

    /// Compares clockwise from noon (accounting for inverted y-axis).
    fn sort_key(self) -> (bool, f64) {
        let (dx, dy) = self.0;
        let up: bool = dx == 0 && dy < 0;
        let down: bool = dx == 0 && dy > 0;

        let slope = if up || down {
            f64::NEG_INFINITY
        } else {
            dy as f64 / dx as f64
        };

        // The left half of the clock, ie [6pm, noon)
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
