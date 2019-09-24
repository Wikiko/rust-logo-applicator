use raster::editor;
use raster::BlendMode;
use raster::PositionMode;
use std::fs::read_dir;
use std::io;

fn main() {

    // let image1 = raster::open("sample.jpg").unwrap();
    // let image2 = raster::open("watermark.png").unwrap();

    // let image3 = editor::blend(&image1, &image2, BlendMode::Normal, 1.0, PositionMode::BottomRight, -15, -15).unwrap();

    // raster::save(&image3, "watermarked.jpg").unwrap();

    
    apply_watermark().unwrap();

}

fn apply_watermark() -> io::Result<()> {
    let watermark = raster::open("watermark.png").unwrap();

    for (i, entry) in read_dir("./photos")?.enumerate() {
        let entry = entry?;

        let path = entry.path().into_os_string().into_string().unwrap();
        if !path.contains(".jpg"){
            continue;
        }

        let image = raster::open(&path).unwrap();

        // let watermarked_image = editor::blend(&image, &watermark, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();

        // raster::save(&watermarked_image, &path).unwrap();
        let teste = format!("{}.jpg", &i);
        let file_name = entry.path().file_name().and_then(|inner_path| inner_path.to_str()).map(|s| String::from(s)).unwrap_or(teste);
        raster::save(&image, &file_name);
        println!("Saved {:?}", &file_name);
    }

    Ok(())
}
