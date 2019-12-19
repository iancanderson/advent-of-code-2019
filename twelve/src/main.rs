use itertools::{Itertools, Combinations};
use std::collections::HashMap;
use std::ops;
use core::cmp::Ordering;

fn main() {
    let mut moons = vec![
        Moon {
            position: Position {
                x: -2,
                y: 9,
                z: -5,
            },
            velocity: Velocity {
                x: 0,
                y: 0,
                z: 0,
            },
        },
        Moon {
            position: Position {
                x: 16,
                y: 19,
                z: 9,
            },
            velocity: Velocity {
                x: 0,
                y: 0,
                z: 0,
            },
        },
        Moon {
            position: Position {
                x: 0,
                y: 3,
                z: 6,
            },
            velocity: Velocity {
                x: 0,
                y: 0,
                z: 0,
            },
        },
        Moon {
            position: Position {
                x: 11,
                y: 0,
                z: 11,
            },
            velocity: Velocity {
                x: 0,
                y: 0,
                z: 0,
            },
        },
    ];

    for _ in 0..1000 {
        moons = time_step(&moons);
    };

    println!("moons_after_1000_steps: {:?}", moons);

    let total_energy: i64 = moons.iter().map(|&moon| {
        return moon.total_energy();
    }).sum();
    println!("total energy of all moons: {}", total_energy);
}

fn time_step(moons: &Vec<Moon>) -> Vec<Moon> {
    let mut new_moons = moons.clone();
    let mut velocity_deltas: HashMap<Moon, Velocity> = new_moons.iter().fold(HashMap::new(), |mut acc, &moon| {
        acc.insert(moon, Velocity { x: 0, y: 0, z: 0 });
        return acc;
    });

    // Calculate new velocities due to gravity
    for moon_pair in new_moons.iter().combinations(2) {
        let moon1 = moon_pair[0];
        let moon2 = moon_pair[1];

        let moon1_delta = velocity_deltas.get(moon1).unwrap();
        let moon1_new_delta = *moon1_delta + moon1.position.apply_gravity(moon2.position);
        // println!("moon1_new_delta: {:?}", moon1_new_delta);
        velocity_deltas.insert((*moon1).clone(), moon1_new_delta);

        let moon2_delta = velocity_deltas.get(moon2).unwrap();
        let moon2_new_delta = *moon2_delta + moon2.position.apply_gravity(moon1.position);
        // println!("moon2_new_delta: {:?}", moon2_new_delta);
        velocity_deltas.insert((*moon2).clone(), moon2_new_delta);
    }

    // println!("velocity_deltas: {:?}", velocity_deltas);

    return new_moons.iter().map(|&moon| {
        let new_velocity = moon.velocity + *velocity_deltas.get(&moon).unwrap();

        // Apply new velocities to positions
        return Moon {
            position: moon.position.apply_velocity(&new_velocity),
            velocity: new_velocity,
        }
    }).collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn total_energy(&self) -> i64 {
        return self.potential_energy() * self.kinetic_energy();
    }

    fn potential_energy(&self) -> i64 {
        return self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
    }

    fn kinetic_energy(&self) -> i64 {
        return self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn apply_velocity(&self, velocity: &Velocity) -> Position {
        return Position {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
            z: self.z + velocity.z,
        };
    }

    // Return a velocity delta
    fn apply_gravity(&self, other_moon_position: Position) -> Velocity {
        let velocity_x_delta =
            match self.x.cmp(&other_moon_position.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

        let velocity_y_delta =
            match self.y.cmp(&other_moon_position.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

        let velocity_z_delta =
            match self.z.cmp(&other_moon_position.z) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

        return Velocity {
            x: velocity_x_delta,
            y: velocity_y_delta,
            z: velocity_z_delta,
        }
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Velocity {
    x: i64,
    y: i64,
    z: i64,
}

impl ops::Add for Velocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
