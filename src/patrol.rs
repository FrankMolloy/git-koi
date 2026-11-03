//! Optional articulated patrol path; body points follow the head by arc distance.
//!
//! Straight passes join quintic Bezier U-turns with matching tangent and zero
//! endpoint curvature. No sprite reflection or instantaneous heading change is used.

use std::f64::consts::{PI, TAU};
use std::sync::OnceLock;

use crate::behavior::Behavior;
use crate::motion::{Course, calculate_swim_pose};

/// Sixteen seconds at the upstream 15 fps: two complete cruising stroke loops.
pub const PATROL_FRAME_COUNT: u32 = 240;
/// Compact native canvas leaves room for a maximum-size koi to turn without resizing.
pub const PATROL_WIDTH_PIXELS: u32 = 300;

/// Distance between the two U-turn entry points, in native pixels.
pub const PATROL_STRAIGHT_PIXELS: f64 = 48.0;

struct Sample {
    distance: f64,
    position: [f64; 2],
}

fn bezier(mut points: [[f64; 2]; 6], t: f64) -> [f64; 2] {
    for remaining in (1..6).rev() {
        for i in 0..remaining {
            points[i] = [0, 1].map(|axis| points[i][axis] * (1.0 - t) + points[i + 1][axis] * t);
        }
    }
    points[0]
}

fn samples() -> &'static [Sample] {
    static PATH: OnceLock<Vec<Sample>> = OnceLock::new();
    PATH.get_or_init(|| {
        let mut points = Vec::new();
        for section in 0..4 {
            for step in 0..=1024 {
                if section > 0 && step == 0 {
                    continue;
                }
                let t = f64::from(step) / 1024.0;
                let point = match section {
                    0 => [-24.0 + PATROL_STRAIGHT_PIXELS * t, -58.0],
                    1 => bezier(
                        [
                            [24.0, -58.0],
                            [56.0, -58.0],
                            [88.0, -58.0],
                            [88.0, 58.0],
                            [56.0, 58.0],
                            [24.0, 58.0],
                        ],
                        t,
                    ),
                    2 => [24.0 - PATROL_STRAIGHT_PIXELS * t, 58.0],
                    _ => bezier(
                        [
                            [-24.0, 58.0],
                            [-56.0, 58.0],
                            [-88.0, 58.0],
                            [-88.0, -58.0],
                            [-56.0, -58.0],
                            [-24.0, -58.0],
                        ],
                        t,
                    ),
                };
                points.push(point);
            }
        }
        let mut distance = 0.0;
        points
            .iter()
            .enumerate()
            .map(|(i, &position)| {
                if i > 0 {
                    distance +=
                        (position[0] - points[i - 1][0]).hypot(position[1] - points[i - 1][1]);
                }
                Sample { distance, position }
            })
            .collect()
    })
}

/// Length of one closed patrol in native pixels.
pub fn path_length() -> f64 {
    samples().last().expect("the path has samples").distance
}

/// Position at a signed arc distance. Negative distances follow the previous loop.
pub fn path_position(distance: f64) -> [f64; 2] {
    let path = samples();
    let distance = distance.rem_euclid(path_length());
    let upper = path
        .partition_point(|sample| sample.distance <= distance)
        .min(path.len() - 1);
    if upper == 0 {
        return path[0].position;
    }
    let left = &path[upper - 1];
    let right = &path[upper];
    let fraction = (distance - left.distance) / (right.distance - left.distance);
    [0, 1].map(|axis| left.position[axis] * (1.0 - fraction) + right.position[axis] * fraction)
}

/// Head progress uses the original cruise beat/glide speed, completing one patrol.
pub fn head_distance(frame_index: u32) -> f64 {
    let frame = frame_index % PATROL_FRAME_COUNT;
    let pose = calculate_swim_pose(Behavior::Cruise, frame);
    let Course::Lap { angle_radians } = pose.course else {
        unreachable!("cruise follows a lap")
    };
    let cruise_turns = f64::from(frame / 120) + (angle_radians + PI / 2.0) / TAU;
    path_length() * cruise_turns / 2.0
}

/// A spine point trails the head by its distance along the body, not by a rotation.
pub fn spine_position(frame_index: u32, distance_behind_head: f64) -> [f64; 2] {
    path_position(head_distance(frame_index) - distance_behind_head)
}
