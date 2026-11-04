# Full-width articulated patrol — unpublished refinement

The live GitHub profile measured 831.2 CSS pixels of README content width at a
1280-pixel desktop viewport, and 293.6 pixels on a narrow phone. The current
published GIF occupies only 300 x 256 CSS pixels of that row.

This refinement uses an **832 x 256 native transparent canvas**, exported by the
existing exact 2x pixel encoder as **1664 x 512**. Proposed README integration:

```html
<img
  src="koi/koi.gif"
  width="100%"
  alt="A koi generated from my public GitHub activity"
/>
```

At the measured desktop width, this displays about 831 x 256 pixels. Native fish
anatomy stays at approximately one CSS pixel per native pixel. Height follows
the aspect ratio; there is no fixed-height GitHub Koi section.

On mobile the whole scene scales to about 294 x 90 pixels, preventing overflow
and clipping but making the fish smaller. A single GIF cannot preserve desktop
fish size on phones while showing the entire wide route. No fish stretching,
enlargement, or artificial changes to GitHub activity are used.

## Motion and appearance

Two 580-native-pixel straight passes join smooth quintic Bezier U-turns near the
far ends. Upper/lower lanes sit at y = -58/+58 relative to canvas centre. The
U-turn shape is unchanged from the published compact patrol; each end is
translated outward. The head starts halfway through the upper straight, so the
entire spine can begin in a clean forward pass.

Each spine point follows `path(head_distance - body_distance)`: head first,
middle next, tail last. Matching tangents and zero endpoint curvature join turns
to straights. Nothing mirrors, flips or teleports. Upstream waves, fins, tail
lobes, whiskers, mouth, shading, colours and markings are reused.

Six complete upstream cruise pose cycles make one **48-second, 720-frame loop at
15 fps**, with two U-turns. Arc-length sampling and upstream beat/glide
variation remain. All path and pose phases wrap together.

`PATROL_ANIMATION=true` / `patrol-animation: "true"` remains optional and
defaults to false. Keep `FORCE_MAX_GROWTH=true` and omit the body input for the
original white body. Patrol selects continuous cruising instead of
activity-selected rest/feeding paths. Genuine pigments, identity and streaks
remain. `extra-swim-motion` is ignored during patrol. Disabling patrol restores
original activity-selected movement. Still PNG output remains 512 x 512.

## Reproduce and verify

With the workspace-local Cargo environment configured, from the workspace root:

```powershell
cargo run --release --locked --manifest-path generator/Cargo.toml --example preview-patrol -- .tools/frank-live-snapshot.json preview-frank/patrol-full-width
```

The example writes the full GIF, PNG, 16 inspection stills, contact sheet and
`metrics.json`. It checks every frame for clear borders and decodes the GIF to
verify dimensions, frames, duration, transparent clearing and infinite looping.
Metrics include head-path extrema and actual opaque-pixel occupied bounds.

Tests cover full geometry clearance including fins and whiskers, head-first
turns, horizontal passes, traversal of both far ends, continuous spine movement
through the loop join, and mixed-canvas GIF rejection. Run the full tests,
formatting, Clippy, release build, documentation and workflow lint checks.

Preview pages:

- `preview-frank/full-width-comparison.html`: current public GIF versus
  proposal, each inside the entire README image row.
- `preview-frank/full-width-profile.html`: complete current public README text,
  links and attribution in a GitHub-like profile layout.
- `preview-frank/proposed-README.md`: current public README with only the image
  width changed to `100%`; profile repository is untouched.

The proposal uses the genuine snapshot observed on 2 November 2026. The baseline
is downloaded from the live public repository; daily activity may differ.
Nothing is committed or published. Approval is required before changing the
profile's README, generator SHA pin, or generated image. Preserve image path,
attribution, licence and current public README text when integrating.
