use git_koi::behavior::Behavior;
use git_koi::coat::KoiBody;
use git_koi::history::AccountSnapshot;
use git_koi::koi::derive_koi_development;
use git_koi::output::{GifEncodingError, encode_gif};
use git_koi::patrol::{
    PATROL_FRAME_COUNT, PATROL_STRAIGHT_PIXELS, PATROL_WIDTH_PIXELS, head_distance, path_length,
    path_position, spine_position,
};
use git_koi::render::{render_patrol_frame, render_swim_frame_with_growth};

fn direction(distance: f64) -> [f64; 2] {
    let a = path_position(distance - 0.5);
    let b = path_position(distance + 0.5);
    [b[0] - a[0], b[1] - a[1]]
}

#[test]
fn longer_straights_start_with_the_head_cruising_at_their_midpoint() {
    assert_eq!(PATROL_STRAIGHT_PIXELS, 580.0);
    let start = path_position(head_distance(0));
    assert!(start[0].abs() < 1e-9 && (start[1] + 58.0).abs() < 1e-9);
    for distance in [1.0, 145.0, 290.0, 435.0, 579.0] {
        let top = path_position(distance);
        let bottom = path_position(path_length() / 2.0 + distance);
        assert!((top[1] + 58.0).abs() < 1e-9);
        assert!((bottom[1] - 58.0).abs() < 1e-9);
        assert!(direction(distance)[0] > 0.9);
        assert!(direction(path_length() / 2.0 + distance)[0] < -0.9);
    }
}

#[test]
fn the_head_traverses_both_far_ends_and_both_vertical_lanes() {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for frame in 0..PATROL_FRAME_COUNT {
        let point = path_position(head_distance(frame));
        for axis in 0..2 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }
    let half_width = f64::from(PATROL_WIDTH_PIXELS) / 2.0;
    assert!(min[0] < -half_width * 0.8);
    assert!(max[0] > half_width * 0.8);
    assert!(min[1] <= -58.0 && max[1] >= 58.0);
    // All anatomy can start on the long straight, not in the previous U-turn.
    for behind in [0.0, 40.0, 80.0, 120.0, 166.0] {
        assert!((spine_position(0, behind)[1] + 58.0).abs() < 1e-9);
    }
}

#[test]
fn both_turns_are_articulated_head_first_without_reflection() {
    let half = path_length() / 2.0;
    let turn_length = half - PATROL_STRAIGHT_PIXELS;
    for (start, sign) in [
        (PATROL_STRAIGHT_PIXELS, 1.0),
        (half + PATROL_STRAIGHT_PIXELS, -1.0),
    ] {
        let distance = start + turn_length * 0.75;
        assert!(direction(distance)[0] * sign < -0.2, "head has turned");
        assert!(
            direction(distance - 160.0)[0] * sign > 0.8,
            "tail has not yet turned"
        );
        assert!(
            direction(distance - 80.0)[1] * sign > 0.2,
            "middle bends around the end"
        );
        let before = direction(start - 0.1);
        let after = direction(start + 0.1);
        assert!(
            (before[1] - after[1]).abs() < 0.001,
            "smooth curvature entry"
        );
    }
}

#[test]
fn every_spine_point_advances_continuously_through_the_loop_join() {
    for behind in [0.0, 40.0, 80.0, 120.0, 166.0] {
        assert_eq!(
            spine_position(0, behind),
            spine_position(PATROL_FRAME_COUNT, behind)
        );
        for frame in 0..PATROL_FRAME_COUNT {
            let a = spine_position(frame, behind);
            let b = spine_position(frame + 1, behind);
            let step = (b[0] - a[0]).hypot(b[1] - a[1]);
            assert!(
                (1.0..6.0).contains(&step),
                "frame {frame}, behind {behind}: {step}"
            );
        }
    }
    for frame in 0..PATROL_FRAME_COUNT - 1 {
        assert!(head_distance(frame + 1) > head_distance(frame));
    }
}

#[test]
fn patrol_frames_fit_and_encode_on_the_wider_canvas() {
    let snapshot: AccountSnapshot =
        serde_json::from_str(include_str!("fixtures/account-with-history.json")).unwrap();
    let koi = derive_koi_development(&snapshot).unwrap();
    let first = render_patrol_frame(&koi, KoiBody::White, 0, true);
    assert_eq!(
        first,
        render_patrol_frame(&koi, KoiBody::White, PATROL_FRAME_COUNT, true)
    );
    let mut frames = Vec::new();
    for index in (0..PATROL_FRAME_COUNT).step_by(6) {
        let frame = render_patrol_frame(&koi, KoiBody::White, index, true);
        assert_eq!(frame.width_pixels(), PATROL_WIDTH_PIXELS);
        let pixels = frame.as_rgba_bytes().as_chunks::<4>().0;
        let mut count = 0;
        for (i, p) in pixels.iter().enumerate() {
            if p[3] == 0 {
                continue;
            }
            count += 1;
            let (x, y) = (
                i % PATROL_WIDTH_PIXELS as usize,
                i / PATROL_WIDTH_PIXELS as usize,
            );
            assert!(
                x > 0 && x < PATROL_WIDTH_PIXELS as usize - 1 && y > 0 && y < 255,
                "edge clipping at frame {index}"
            );
        }
        assert!(count > 1000);
        frames.push(frame);
    }
    let gif = encode_gif(&frames).unwrap();
    assert_eq!(&gif[6..10], &[128, 6, 0, 2]); // 1664 x 512 little endian.
    let ordinary = render_swim_frame_with_growth(&koi, KoiBody::White, Behavior::Cruise, 0, true);
    assert!(matches!(
        encode_gif(&[ordinary, first]),
        Err(GifEncodingError::CanvasMismatch { frame_index: 1 })
    ));
}
