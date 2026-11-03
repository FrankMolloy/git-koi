# Maximum growth customisation

Based on [git-koi](https://github.com/yalongwastaken/git-koi), MIT licensed, by
Anthony Yalong. Original README and LICENSE are preserved.

Set `FORCE_MAX_GROWTH=true` when rendering, or use this Action with
`force-max-growth: "true"`. The default is false. Invalid values fail. Both
still and animated rendering use the original maximum anatomy (the
100,000-contribution cap), without altering any recorded contribution count.
Language colours/pigment maturation, identity, streaks and daily swimming
behaviour continue to use genuine activity. The gold body is selected in the
profile workflow, not hardcoded in the renderer.

This composite Action builds its checked-out source with Cargo instead of
fetching the upstream release binary. Install Rust >=1.88 before using it. For a
public fork, the existing built-in workflow token is sufficient.

```yaml
- name: Install Rust
  run: rustup toolchain install stable --profile minimal
- uses: FrankMolloy/git-koi@max-growth # Prefer a reviewed full custom commit SHA.
  with:
    body: gold
    force-max-growth: "true"
    output-dir: koi
```

Verify with `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets -- -D warnings`, `cargo test --locked`, and
`cargo build --release --locked`. Tests in `tests/max_growth.rs` cover maximum
anatomy, unchanged default rendering and CLI configuration. Local verification
passed on Windows using workspace-local Rust 1.88 GNU: 98 release tests,
rustfmt, Clippy, build and documentation checks. A concurrent scratch-folder
race was fixed, then all five growth tests passed again. Hosted Ubuntu CI
remains to be run.

Preview using an authenticated `gh` and real snapshot:

```powershell
cargo run --release --locked -- collect FrankMolloy --output snapshot.json
$env:FORCE_MAX_GROWTH = 'true'
cargo run --release --locked -- render snapshot.json --output-dir koi --body gold
Start-Process koi/preview.html
```

For an offline demo, use `tests/fixtures/account-with-history.json` instead of
`snapshot.json`. That fixture is not Frank's activity. Fresh default and maximum
GIF/PNG previews were generated locally; default output matched untouched
upstream byte for byte. Maximum output was visibly larger and different in both
formats.
