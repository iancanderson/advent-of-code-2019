fn main() {
    let input = ".###..#......###..#...#
#.#..#.##..###..#...#.#
#.#.#.##.#..##.#.###.##
.#..#...####.#.##..##..
#.###.#.####.##.#######
..#######..##..##.#.###
.##.#...##.##.####..###
....####.####.#########
#.########.#...##.####.
.#.#..#.#.#.#.##.###.##
#..#.#..##...#..#.####.
.###.#.#...###....###..
###..#.###..###.#.###.#
...###.##.#.##.#...#..#
#......#.#.##..#...#.#.
###.##.#..##...#..#.#.#
###..###..##.##..##.###
###.###.####....######.
.###.#####.#.#.#.#####.
##.#.###.###.##.##..##.
##.#..#..#..#.####.#.#.
.#.#.#.##.##########..#
#####.##......#.#.####.";

    println!("Best monitoring station: {:?}", best_monitoring_station(input));
}

fn asteroids(input: &str) -> Vec<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut result: Vec<(i32, i32)> = vec![];

    // First, get width of grid by finding first newline
    for line in input.split('\n') {
        println!("{:?}", line);

        for c in line.chars() {
            if c == '#' {
                result.push((x, y));
            }
            x += 1;
        }

        y += 1;
        x = 0;
    }

    return result;
}

#[derive(Debug)]
struct MonitoringStation {
    position: (i32, i32),
    visible_asteroid_count: i32,
}

fn best_monitoring_station(input: &str) -> MonitoringStation {
    // Find the asteroid with the most visible other asteroids
    let all_asteroids = asteroids(input);
    let monitoring_stations = all_asteroids.iter().map(|&a| {
        return MonitoringStation {
            position: a,
            visible_asteroid_count: visible_asteroid_count(&all_asteroids, &a),
        };
    });
    return monitoring_stations.max_by_key(|station| station.visible_asteroid_count).expect("No min?");
}

fn visible_asteroid_count(all_asteroids: &Vec<(i32, i32)>, reference_asteroid: &(i32, i32)) -> i32 {
    // Each other asteroid is visible to the reference asteroid if
    // there is not another asteroid blocking the line of sight
    let mut result = 0;

    for candidate_asteroid in all_asteroids {
        if candidate_asteroid == reference_asteroid { continue }

        // Increment result if there are no other asteroids that block it
        if is_visible(candidate_asteroid, reference_asteroid, all_asteroids) {
            result += 1;
        }
    }

    return result;
}

fn is_visible(asteroid: &(i32, i32), reference_asteroid: &(i32, i32), all_asteroids: &Vec<(i32, i32)>) -> bool {
    // Visible if not any other asteroids block the line of sight
    return !all_asteroids.iter().any(|&potential_blocker| {
        // Return true if potential_blocker is blocking line of sight between asteroid and reference
        if asteroid == &potential_blocker { return false }
        if asteroid == reference_asteroid { return false }
        if reference_asteroid == &potential_blocker  { return false }

        // https://stackoverflow.com/a/17693146
        let ref_to_blocker = distance(reference_asteroid, &potential_blocker);
        let asteroid_to_blocker = distance(asteroid, &potential_blocker);
        let ref_to_asteroid = distance(reference_asteroid, asteroid);

        if ((ref_to_blocker + asteroid_to_blocker) - ref_to_asteroid).abs() < 0.0001 {
            return true;
        }

        return false;
    });
}

fn distance(a: &(i32, i32), b: &(i32, i32)) -> f32 {
    let d = ((b.0 - a.0).pow(2) as f32 + (b.1 - a.1).pow(2) as f32).sqrt();
    return d;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_visible() {
        let all_asteroids = vec![(1,0), (4,0), (0,2), (1,2), (2,2), (3,2), (4,2), (4,3), (3,4), (4,4)];
        let reference = (3,4);
        let visible_asteroids = vec![(4,0), (0,2), (1,2), (2,2), (3,2), (4,2), (4,3), (4,4)];
        let not_visible_asteroid = (1,0);

        for other_asteroid in visible_asteroids {
            assert!(is_visible(&other_asteroid, &reference, &all_asteroids));
        }

        assert!(!is_visible(&not_visible_asteroid, &reference, &all_asteroids));
    }

    #[test]
    fn test_ten_example_one() {
let input=".#..#
.....
#####
....#
...##";

        let station = best_monitoring_station(input);
        assert_eq!(station.position, (3,4));
        assert_eq!(station.visible_asteroid_count, 8);
    }

    #[test]
    fn test_asteroids() {
let input=".#..#
.....
#####
....#
...##";

        assert_eq!(asteroids(input), [(1,0), (4,0), (0,2), (1,2), (2,2), (3,2), (4,2), (4,3), (3,4), (4,4)]);
    }

    #[test]
    fn test_ten_example_two() {
let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

        let station = best_monitoring_station(input);
        assert_eq!(station.position, (5,8));
        assert_eq!(station.visible_asteroid_count, 33);
    }

    #[test]
    fn test_ten_example_three() {
let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

        let station = best_monitoring_station(input);
        assert_eq!(station.position, (1,2));
        assert_eq!(station.visible_asteroid_count, 35);
    }

    #[test]
    fn test_ten_example_four() {
let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

        let station = best_monitoring_station(input);
        assert_eq!(station.position, (6,3));
        assert_eq!(station.visible_asteroid_count, 41);
    }

    #[test]
    fn test_ten_example_five() {
let input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let station = best_monitoring_station(input);
        assert_eq!(station.position, (11,13));
        assert_eq!(station.visible_asteroid_count, 210);
    }
}
