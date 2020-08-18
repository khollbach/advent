use day_10::{asteroids, grid, part1, part2};

fn main() {
    let grid = grid::read_input();
    let asteroids = asteroids(&grid);

    let (station, num_vis) = part1::best_station(&asteroids);
    println!("{}", num_vis);

    let (x, y) = part2::asteroid_200(&asteroids, station);
    println!("{}", x * 100 + y);
}
