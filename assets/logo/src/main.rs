
use std::fs;
use std::path::Path;

fn main() {
    let sizes = [16, 24, 32, 48, 64, 128, 256, 512, 1024];
    let icons = ["biggit", "gows"];
    let svg_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let out_dir = svg_dir;

    for icon in icons {
        let svg_path = svg_dir.join(format!("{}.svg", icon));
        let micro_svg_path = svg_dir.join(format!("{}-micro.svg", icon));
        let mut png_paths = Vec::new();
        for &size in &sizes {
            let use_micro = size == 16 || size == 24;
            let svg = if use_micro { &micro_svg_path } else { &svg_path };
            let png_path = out_dir.join(format!("{}-{}.png", icon, size));
            render_svg_to_png(svg, &png_path, size);
            png_paths.push(png_path.clone());
        }
        // Generate ICO from PNGs
        let ico_path = out_dir.join(format!("{}.ico", icon));
        make_ico(&png_paths, &ico_path);
    }
    // ICNS not supported in Rust ecosystem yet (2026)
}

fn render_svg_to_png(svg_path: &Path, png_path: &Path, size: u32) {
    use usvg::{Options, Tree, fontdb::Database};
    use tiny_skia::Pixmap;
    use resvg::tiny_skia::Transform;
    let svg_data = fs::read(svg_path).expect("read svg");
    let opt = Options::default();
    let mut fontdb = Database::new();
    fontdb.load_system_fonts();
    let tree = Tree::from_data(&svg_data, &opt, &fontdb).expect("parse svg");
    let mut pixmap = Pixmap::new(size, size).expect("pixmap");

    // Compute scale and translation to fit SVG viewBox into output size
    let vb = tree.view_box();
    let scale = f32::min(size as f32 / vb.rect.width() as f32, size as f32 / vb.rect.height() as f32);
    let dx = (size as f32 - vb.rect.width() as f32 * scale) / 2.0;
    let dy = (size as f32 - vb.rect.height() as f32 * scale) / 2.0;
    let transform = Transform::from_translate(dx, dy).post_scale(scale, scale);

    let mut pixmap_mut = pixmap.as_mut();
    resvg::render(&tree, transform, &mut pixmap_mut);
    pixmap.save_png(png_path).expect("save png");
}

fn make_ico(png_paths: &[std::path::PathBuf], ico_path: &Path) {
    use std::fs::File;
    use ico::{IconDir, IconDirEntry, IconImage, ResourceType};
    let mut icon_dir = IconDir::new(ResourceType::Icon);
    for png in png_paths {
        if let Ok(img) = image::open(png) {
            let icon_img = IconImage::from_rgba_data(img.width(), img.height(), img.to_rgba8().into_raw());
            let entry = IconDirEntry::encode(&icon_img).expect("encode ico entry");
            icon_dir.add_entry(entry);
        }
    }
    let mut file = File::create(ico_path).expect("ico file");
    icon_dir.write(&mut file).expect("write ico");
}
