use std::fs;
use std::process::Command;

use git_koi::behavior::{Behavior, select_behavior};
use git_koi::coat::KoiBody;
use git_koi::history::AccountSnapshot;
use git_koi::koi::{MAX_GROWTH_CONTRIBUTIONS, derive_koi_development};
use git_koi::render::{
    render_koi, render_koi_with_growth, render_swim_frame, render_swim_frame_with_growth,
};

fn development() -> git_koi::koi::KoiDevelopment {
    let snapshot: AccountSnapshot =
        serde_json::from_str(include_str!("fixtures/account-with-history.json")).unwrap();
    derive_koi_development(&snapshot).unwrap()
}

#[test]
fn override_matches_maximum_anatomy_and_keeps_original_data() {
    let mut koi = development();
    // Both totals already have fully developed pigment, isolating anatomy.
    koi.total_contributions = 500;
    let before = format!("{koi:?}");
    let behavior = select_behavior(&koi);
    let forced_still = render_koi_with_growth(&koi, KoiBody::Gold, true);
    let forced_frame = render_swim_frame_with_growth(&koi, KoiBody::Gold, behavior, 17, true);
    assert_eq!(format!("{koi:?}"), before);
    assert_eq!(select_behavior(&koi), behavior);
    assert_ne!(forced_still, render_koi(&koi, KoiBody::Gold));
    koi.total_contributions = MAX_GROWTH_CONTRIBUTIONS;
    assert_eq!(forced_still, render_koi(&koi, KoiBody::Gold));
    assert_eq!(
        forced_frame,
        render_swim_frame(&koi, KoiBody::Gold, behavior, 17)
    );
}

#[test]
fn disabled_override_is_identical_for_every_body_and_behavior() {
    let koi = development();
    for body in [
        KoiBody::White,
        KoiBody::Black,
        KoiBody::Asagi,
        KoiBody::Tea,
        KoiBody::Gold,
        KoiBody::Platinum,
    ] {
        assert_eq!(
            render_koi(&koi, body),
            render_koi_with_growth(&koi, body, false)
        );
        for behavior in [
            Behavior::DeepRest,
            Behavior::Rest,
            Behavior::SurfaceFeeding,
            Behavior::Dart,
            Behavior::Cruise,
        ] {
            assert_eq!(
                render_swim_frame(&koi, body, behavior, 17),
                render_swim_frame_with_growth(&koi, body, behavior, 17, false)
            );
        }
    }
}

#[test]
fn command_override_renders_maximum_and_rejects_invalid_values() {
    let directory = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("max-growth-cli-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/account-with-history.json"
    );
    let run = |value: &str| {
        Command::new(env!("CARGO_BIN_EXE_git-koi"))
            .args([
                "render",
                fixture,
                "--output-dir",
                directory.to_str().unwrap(),
                "--body",
                "gold",
            ])
            .env("FORCE_MAX_GROWTH", value)
            .env_remove("EXTRA_SWIM_MOTION")
            .output()
            .unwrap()
    };
    assert!(run("true").status.success());
    let koi = development();
    let expected =
        git_koi::output::encode_png(&render_koi_with_growth(&koi, KoiBody::Gold, true)).unwrap();
    assert_eq!(fs::read(directory.join("koi.png")).unwrap(), expected);
    let gif = fs::read(directory.join("koi.gif")).unwrap();
    let behavior = select_behavior(&koi);
    let frames: Vec<_> = (0..git_koi::motion::frame_count(behavior))
        .map(|index| render_swim_frame_with_growth(&koi, KoiBody::Gold, behavior, index, true))
        .collect();
    assert_eq!(gif, git_koi::output::encode_gif(&frames).unwrap());
    assert!(!run("typo").status.success());
    assert_eq!(fs::read(directory.join("koi.gif")).unwrap(), gif);
    assert!(run("false").status.success());
    assert_eq!(
        fs::read(directory.join("koi.png")).unwrap(),
        git_koi::output::encode_png(&render_koi(&development(), KoiBody::Gold)).unwrap()
    );
}

#[test]
fn forced_geometry_stays_maximum_while_colors_and_streaks_remain_live() {
    let mut koi = development();
    let original = render_koi_with_growth(&koi, KoiBody::Gold, true);
    let alpha = |frame: &git_koi::render::KoiFrame| {
        frame
            .as_rgba_bytes()
            .as_chunks::<4>()
            .0
            .iter()
            .map(|p| p[3])
            .collect::<Vec<_>>()
    };
    koi.total_contributions = 0;
    let zero = render_koi_with_growth(&koi, KoiBody::Gold, true);
    koi.total_contributions = MAX_GROWTH_CONTRIBUTIONS;
    let mature = render_koi_with_growth(&koi, KoiBody::Gold, true);
    assert_eq!(alpha(&zero), alpha(&mature));
    assert_ne!(zero, mature, "genuine pigment maturation must still work");
    koi.total_contributions = u64::MAX;
    assert_eq!(mature, render_koi_with_growth(&koi, KoiBody::Gold, true));
    koi.total_contributions = development().total_contributions;
    koi.streaks.longest.clear();
    koi.streaks.recent = None;
    let unmarked = render_koi_with_growth(&koi, KoiBody::Gold, true);
    assert_eq!(alpha(&original), alpha(&unmarked));
    assert_ne!(
        original, unmarked,
        "real streak marks must still affect appearance"
    );
    koi.language_shares.clear();
    koi.language_colors.clear();
    let uncolored = render_koi_with_growth(&koi, KoiBody::Gold, true);
    assert_eq!(alpha(&unmarked), alpha(&uncolored));
    assert_ne!(
        unmarked, uncolored,
        "real languages must still color the fish"
    );
}

#[test]
fn forced_animation_moves_and_wraps_for_every_behavior() {
    let koi = development();
    for behavior in [
        Behavior::DeepRest,
        Behavior::Rest,
        Behavior::SurfaceFeeding,
        Behavior::Dart,
        Behavior::Cruise,
    ] {
        let first = render_swim_frame_with_growth(&koi, KoiBody::Gold, behavior, 0, true);
        assert_ne!(
            first,
            render_swim_frame_with_growth(&koi, KoiBody::Gold, behavior, 17, true)
        );
        assert_eq!(
            first,
            render_swim_frame_with_growth(
                &koi,
                KoiBody::Gold,
                behavior,
                git_koi::motion::frame_count(behavior),
                true
            )
        );
    }
}
