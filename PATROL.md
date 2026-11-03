# Articulated patrol mode (local preview, not published)

Set `PATROL_ANIMATION=true` for rendering, or use the Action input
`patrol-animation: "true"`. It defaults to false. Keep `FORCE_MAX_GROWTH=true`
and omit `--body`/the body input for the upstream white body.

## What is reused

Upstream's circular lap renderer already articulates its spine: each body point
sits one pixel of arc behind the head, then the renderer applies its travelling
body wave. Fins attach to that spine and rotate/bend in its local tangent frame.
Upstream does not contain a back-and-forth patrol course.

The extension replaces only the circular centreline with a closed patrol path.
Two straight passes join quintic Bezier U-turns. Their tangents and zero
endpoint curvature match the straights. Arc-length sampling makes speed
independent of the Bezier parameter. A body point is drawn at
`path(head_distance - body_distance)`; therefore the head turns before the
middle and tail. Nothing reflects or mirrors an image. The same wave, fin, tail,
whisker, mouth, coat and outline code draws it.

Original cruising beat/glide speed drives progress. Two full upstream cruising
pose cycles make one 16-second, 240-frame loop at 15 fps, containing a U-turn at
each end. All path positions and fin/wave phases wrap together.

## Space and behaviour

The maximum native anatomy is unchanged. Patrol uses a 300 x 256 native canvas,
exported as an 600 x 512 transparent GIF. Its still PNG stays the original 512 x
512 maximum-growth image. Render helpers and the encoders now accept the frame
width; ordinary frames remain 256 x 256 and ordinary output remains unchanged.

The refined README embed is prepared locally at width 300, giving the same
native pixel scale and 256-pixel display height as the live 256-pixel square
animation. Straight passes shorten from 156 to 48 native pixels; the relative
U-turn control points and lane separation remain unchanged. This compacts the
footprint without tightening the turns. No live workflow, SHA pin or image has
changed.

Patrol intentionally selects continuous upstream cruising movement instead of
the activity-selected resting/feeding course. Genuine colours, pigment
development, identity and streaks are retained. `extra-swim-motion` is ignored
while patrol is active. Disable patrol to restore the existing activity-selected
movement unchanged.

## Reproduce

With the workspace-local Cargo environment configured, at the workspace root:

```powershell
$env:FORCE_MAX_GROWTH = 'true'
$env:PATROL_ANIMATION = 'true'
cargo run --release --locked --manifest-path generator/Cargo.toml -- render .tools/frank-live-snapshot.json --output-dir preview-frank/patrol-cli
cargo run --release --locked --manifest-path generator/Cargo.toml --example preview-patrol -- .tools/frank-live-snapshot.json preview-frank/patrol
```

The example generates the complete GIF, the unchanged maximum-growth still, 16
inspection frames and a contact sheet. It decodes the GIF to verify 240 frames,
16-second duration, infinite repetition and background clearing, and checks the
opaque border of every rendered frame. Tests check smooth motion of
head/body/tail through the wrap, head-first turns at both ends, and full
geometry clearance on all 240 frames (including fins and whiskers). Mismatched
GIF canvas widths fail explicitly rather than producing corrupt output.

The local preview uses FrankMolloy's genuine public snapshot observed on 2
November 2026. No contributions or history are changed. Approval is required
before committing, publishing or changing the profile's SHA pin.

## Profile prominence comparison

The local three-way animated comparison is
`preview-frank/prominence-comparison.html`: live (256-pixel embed), previous
patrol (416-native-pixel canvas in a 256-pixel embed), and refined patrol
(300-native-pixel canvas in a 300-pixel embed). The previous patrol reduced
linear pixel scale to 61.5%; the refined version restores 100% on desktop. The
full-size fish consequently occupies the same pixel scale as the current live
version while retaining its articulated turns.

The actual public profile measured approximately 831 pixels of README width at a
1280-pixel viewport, so a 300-pixel embed fits comfortably. At a 390-pixel phone
viewport, its README content is approximately 294 pixels wide: refined native
scale is about 98%, compared with 71% for a full-size 416-pixel embed. Smaller
screens can still reduce any wide image. The preview includes this constrained
case.

The profile change is a single local README width edit, preserving the image URL
and attribution. It must be published together with a verified SHA-pinned patrol
generator workflow when approved. Nothing has been committed or published.
