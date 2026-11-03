use git_koi::{
    coat::KoiBody,
    history::AccountSnapshot,
    koi::derive_koi_development,
    output::{encode_gif, encode_png, encode_preview_html},
    patrol::PATROL_FRAME_COUNT,
    render::{render_koi_with_growth, render_patrol_frame},
};
use std::{env, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let snapshot: AccountSnapshot = serde_json::from_str(&fs::read_to_string(&args[1])?)?;
    let koi = derive_koi_development(&snapshot)?;
    let directory = Path::new(&args[2]);
    fs::create_dir_all(directory)?;
    let frames: Vec<_> = (0..PATROL_FRAME_COUNT)
        .map(|i| render_patrol_frame(&koi, KoiBody::White, i, true))
        .collect();
    let gif_bytes = encode_gif(&frames)?;
    let mut decoder = gif::DecodeOptions::new().read_info(std::io::Cursor::new(&gif_bytes))?;
    assert_eq!((decoder.width(), decoder.height()), (600, 512));
    let mut decoded_frames = 0;
    let mut duration_hundredths = 0;
    while let Some(frame) = decoder.read_next_frame()? {
        decoded_frames += 1;
        duration_hundredths += u32::from(frame.delay);
        assert_eq!(frame.dispose, gif::DisposalMethod::Background);
    }
    assert_eq!(decoded_frames, 240);
    assert_eq!(duration_hundredths, 1600);
    assert_eq!(decoder.repeat(), gif::Repeat::Infinite);
    for (index, frame) in frames.iter().enumerate() {
        for (i, pixel) in frame.as_rgba_bytes().as_chunks::<4>().0.iter().enumerate() {
            if pixel[3] == 0 {
                continue;
            }
            let (x, y) = (i % 300, i / 300);
            assert!(
                x > 0 && x < 299 && y > 0 && y < 255,
                "clipping at frame {index}"
            );
        }
    }
    fs::write(directory.join("koi.gif"), gif_bytes)?;
    fs::write(
        directory.join("koi.png"),
        encode_png(&render_koi_with_growth(&koi, KoiBody::White, true))?,
    )?;
    fs::write(
        directory.join("preview.html"),
        encode_preview_html(
            "Articulated patrol: 2 U-turns, 16 seconds, maximum anatomy, white body",
        ),
    )?;
    // Contact sheet samples the complete loop without replacing the animated preview.
    let (tile_w, tile_h) = (300usize, 256usize);
    let mut sheet = vec![0u8; tile_w * 4 * tile_h * 4 * 4];
    for (tile, index) in (0..PATROL_FRAME_COUNT as usize).step_by(15).enumerate() {
        let pixels = frames[index].as_rgba_bytes();
        fs::write(
            directory.join(format!("frame-{index:03}.png")),
            encode_png(&frames[index])?,
        )?;
        for y in 0..tile_h {
            let destination = ((tile / 4 * tile_h + y) * tile_w * 4 + tile % 4 * tile_w) * 4;
            sheet[destination..destination + tile_w * 4]
                .copy_from_slice(&pixels[y * tile_w * 4..(y + 1) * tile_w * 4]);
        }
    }
    let mut file = fs::File::create(directory.join("contact-sheet.png"))?;
    let mut encoder = png::Encoder::new(&mut file, (tile_w * 4) as u32, (tile_h * 4) as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.write_header()?.write_image_data(&sheet)?;
    println!(
        "Wrote {} patrol frames and 16 review stills into {}",
        frames.len(),
        directory.display()
    );
    Ok(())
}
