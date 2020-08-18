use crate::Point;
use std::collections::HashSet;

/// What is the monitoring station with the best visibility?
/// Also returns the number of other asteroids visible from that station.
pub fn best_station(asteroids: &HashSet<Point>) -> (Point, usize) {
    let (num_vis, best) = asteroids
        .iter()
        .map(|&a| (num_visible(&asteroids, a), a))
        .max_by_key(|&(num_vis, _a)| num_vis)
        .unwrap();

    (best, num_vis)
}

/// How many other asteroids are visible from this location?
fn num_visible(asteroids: &HashSet<Point>, base: Point) -> usize {
    asteroids
        .iter()
        .filter(|&&other| base != other && crate::is_visible(asteroids, base, other))
        .count()
}
