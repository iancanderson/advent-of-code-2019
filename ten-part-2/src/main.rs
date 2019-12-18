use std::collections::BTreeMap;
use std::collections::HashSet;

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

    let order = vaporization_order(input);
    println!("200th asteroid to be vaporized: {:?}", order[199]);
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

fn vaporization_order(input: &str) -> Vec<(i32, i32)> {
    let monitoring_station = best_monitoring_station(input);
    println!("Station at {:?}", monitoring_station.position);

    let mut all_asteroids = asteroids(input);
    let station_position_in_vector = all_asteroids.iter().position(|&a| a == monitoring_station.position).unwrap();
    all_asteroids.remove(station_position_in_vector);

    let mut asteroids_to_vaporize: Vec<(i32, i32)> = Vec::new();

    let mut by_angles = group_by_angles(monitoring_station.position, all_asteroids);

    while !by_angles.is_empty() {
        // println!("by_angles keys(): {:?}", by_angles.keys());

        // Loop over angles in increasing order
        for angle in 0..3600 {
            // If the hash has the angle, pop the closest asteroid
            // If that angle has no other asteroids, remove the angle from the hash
            if let Some(asteroids_at_angle) = by_angles.get_mut(&angle) {
                println!("At angle {}, asteroids: {:?}", angle, asteroids_at_angle);

                let asteroids_at_angle_clone = asteroids_at_angle.clone();
                let closest = asteroids_at_angle_clone.iter().min_by_key(|&a| {
                    // Can't order by f32, so multiply and go to integer
                    return (distance(&monitoring_station.position, a) * 1000.0).trunc() as i32;
                }).unwrap();

                asteroids_at_angle.remove(closest);

                if asteroids_at_angle.is_empty() {
                    by_angles.remove(&angle);
                }
                asteroids_to_vaporize.push(*closest);
            }
        }
    }

    return asteroids_to_vaporize;
}

//TODO: have this return radians instead
//degrees are not precise enough
fn angle_from(reference: (i32, i32), point: (i32, i32)) -> i32 {
    let y_delta = point.1 - reference.1;
    let x_delta = point.0 - reference.0;
    let radians =
        if y_delta == 0 {
            0.0
        } else {
            ((x_delta as f32 / y_delta as f32)).atan()
        };

    let degrees = (radians.abs().to_degrees() * 10.0).round() as i32;

    // println!("raw degrees for {},{}: {}", point.0, point.1, degrees);

    if degrees == 0 {
        if y_delta.is_negative() {
            return 0;
        } else if x_delta.is_positive() {
            return 900;
        } else if y_delta.is_positive() {
            return 1800;
        } else if x_delta.is_negative() {
            return 2700;
        }
    }

    if x_delta.is_positive() {
        if y_delta.is_negative() {
            // Quadrant 1
            return degrees;
        } else {
            // Quadrant 2
            return 900 + degrees;
        }
    } else {
        if y_delta.is_positive() {
            // Quadrant 3
            return 1800 + degrees;
        } else {
            // Quadrant 4
            return 2700 + degrees;
        }
    }
}

// Group by degrees
fn group_by_angles(reference: (i32, i32), points: Vec<(i32, i32)>) -> BTreeMap<i32, HashSet<(i32, i32)>> {
    return points.iter().fold(BTreeMap::new(), |mut map, point| {
        let angle = angle_from(reference, *point);

        if let Some(points_with_same_angle) = map.get_mut(&angle) {
            points_with_same_angle.insert(*point);
        } else {
            let mut set = HashSet::new();
            set.insert(*point);
            map.insert(angle, set);
        }

        return map;
    });
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

    #[test]
    fn test_ten_part_two_example_one() {
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";

        let order = vaporization_order(input);
        assert_eq!(order[0], (8, 1));
        assert_eq!(order[1], (9, 0));
        assert_eq!(order[2], (9, 1));
        assert_eq!(order[3], (10, 0));
        assert_eq!(order[4], (9, 2));
        assert_eq!(order[5], (11, 1));
        assert_eq!(order[6], (12, 1));
        assert_eq!(order[7], (11, 2));
        assert_eq!(order[8], (15, 1));
    }

    #[test]
    fn test_group_by_angles() {
        let groups = group_by_angles((2,2), vec![(0,0), (1,1), (2,0), (3,3), (4,2), (4,1)]);
        println!("{:?}", groups);
        assert!(groups.get(&3150).unwrap().contains(&(0,0)));
        assert!(groups.get(&3150).unwrap().contains(&(1,1)));
        assert!(groups.get(&0).unwrap().contains(&(2,0)));
        assert!(groups.get(&1350).unwrap().contains(&(3,3)));
        assert!(groups.get(&900).unwrap().contains(&(4,2)));
    }

    #[test]
    fn test_ten_part_two_example_one_group_by_angles() {
        let groups = group_by_angles((8,3), vec![(8,1), (9,0)]);
        println!("{:?}", groups);
        assert!(groups.get(&0).unwrap().contains(&(8,1)));
        assert!(groups.get(&184).unwrap().contains(&(9,0)));
    }

    #[test]
    fn test_ten_part_2_big_example() {
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

        print_vaporization_order_all(input);

        let order = vaporization_order(input);
        assert_eq!(order[0], (11,12));
        assert_eq!(order[1], (12,1));
        assert_eq!(order[2], (12,2));
        assert_eq!(order[9], (12,8));
        assert_eq!(order[19], (16,0));
        assert_eq!(order[49], (16,9));
        assert_eq!(order[99], (10,16));
        assert_eq!(order[198], (9,6));
        assert_eq!(order[199], (8,2));
    }

    fn print_vaporization_order(input: &str) {
        let mut input_copy = input.clone().to_string();
        let order = vaporization_order(input);

        let station_position = 21 * 13 + 11;
        input_copy.replace_range(station_position..station_position + 1, "X");

        let line_length_with_newline = input_copy.find('\n').unwrap() + 1;

        // Replace # with 1 for first vaporized asteroid, etc
        let mut n = 1;
        for asteroid in order[0..10].iter() {
            // x axis
            let position_in_line = asteroid.0;

            // y axis
            let line = asteroid.1;

            let character_position = line as usize * line_length_with_newline + position_in_line as usize;
            let new_string = &mut n.to_string();
            input_copy.replace_range(character_position..character_position + 1, &new_string);

            n += 1;
            if n == 10 {
                break;
            }
        }

        print!("{}", input_copy);
    }

    fn print_vaporization_order_all(input: &str) {
        let mut input_copy = input.clone().to_string();
        let order = vaporization_order(input);

        let station_position = 21 * 13 + 11;
        input_copy.replace_range(station_position..station_position + 1, "X");

        let line_length_with_newline = input_copy.find('\n').unwrap() + 1;

        // Replace # with 1 for first vaporized asteroid, etc
        let mut n = 1;
        for asteroid in order {
            // x axis
            let position_in_line = asteroid.0;

            // y axis
            let line = asteroid.1;

            let character_position = line as usize * line_length_with_newline + position_in_line as usize;
            input_copy.replace_range(character_position..character_position + 1, ".");

            n += 1;
        }

        print!("{}", input_copy);
    }
}
