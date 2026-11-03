use git_koi::behavior::Behavior;
use git_koi::coat::KoiBody;
use git_koi::history::AccountSnapshot;
use git_koi::koi::{KoiDevelopment, derive_koi_development};
use git_koi::motion::frame_count;
use git_koi::render::{KoiFrame, render_swim_frame_with_growth, render_swim_frame_with_motion};

fn development() -> KoiDevelopment {
    let snapshot: AccountSnapshot =
        serde_json::from_str(include_str!("fixtures/account-with-history.json")).unwrap();
    derive_koi_development(&snapshot).unwrap()
}

fn centroid_y(frame: &KoiFrame) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;
    for (i, pixel) in frame.as_rgba_bytes().as_chunks::<4>().0.iter().enumerate() {
        if pixel[3] != 0 {
            sum += (i / 256) as f64;
            count += 1.0;
            let (x, y) = (i % 256, i / 256);
            assert!(
                x > 0 && x < 255 && y > 0 && y < 255,
                "clipped pixel at {x},{y}"
            );
        }
    }
    assert!(count > 0.0);
    sum / count
}

#[test]
fn increased_motion_is_contained_and_seamless_for_every_behavior() {
    let koi = development();
    for behavior in [
        Behavior::Cruise,
        Behavior::Dart,
        Behavior::Rest,
        Behavior::SurfaceFeeding,
        Behavior::DeepRest,
    ] {
        let first = render_swim_frame_with_motion(&koi, KoiBody::White, behavior, 0, true, true);
        assert_eq!(
            first,
            render_swim_frame_with_motion(
                &koi,
                KoiBody::White,
                behavior,
                frame_count(behavior),
                true,
                true
            )
        );
        for index in (0..frame_count(behavior)).step_by(5) {
            centroid_y(&render_swim_frame_with_motion(
                &koi,
                KoiBody::White,
                behavior,
                index,
                true,
                true,
            ));
        }
        assert_eq!(
            render_swim_frame_with_motion(&koi, KoiBody::White, behavior, 17, true, false),
            render_swim_frame_with_growth(&koi, KoiBody::White, behavior, 17, true)
        );
    }
}

#[test]
fn feeding_has_measurably_more_travel_without_changing_growth_or_timing() {
    let koi = development();
    let travel = |extra| {
        let ys: Vec<_> = (0..frame_count(Behavior::SurfaceFeeding))
            .step_by(5)
            .map(|index| {
                centroid_y(&render_swim_frame_with_motion(
                    &koi,
                    KoiBody::White,
                    Behavior::SurfaceFeeding,
                    index,
                    true,
                    extra,
                ))
            })
            .collect();
        ys.iter().copied().fold(f64::NEG_INFINITY, f64::max)
            - ys.iter().copied().fold(f64::INFINITY, f64::min)
    };
    let normal = travel(false);
    let enhanced = travel(true);
    println!("Native-pixel vertical centroid travel: normal={normal:.2}, enhanced={enhanced:.2}");
    assert!(enhanced > normal + 20.0, "motion must visibly increase");
}
