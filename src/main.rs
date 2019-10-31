use raster::{editor, ResizeMode};
use raster::BlendMode;
use raster::PositionMode;
use std::fs::read_dir;
use std::io;
use threadpool::ThreadPool;

fn main() {
    apply_watermark().unwrap();
}

fn apply_watermark() -> io::Result<()> {
    let total_cpus = num_cpus::get();
    println!("Numero de processadores: {}", total_cpus);
    let pool = ThreadPool::new(total_cpus);

    for (i, entry) in read_dir("./photos")?.enumerate() {
        let entry = entry?;

        let path = entry.path().into_os_string().into_string().unwrap();
        if !path.contains(".jpg"){
            continue;
        }

        pool.execute(move || {
            let image = raster::open(&path).unwrap();
            let mut watermark = raster::open("watermark.png").unwrap();

            let teste = format!("{}.jpg", &i);
            let file_name = entry.path().file_name().and_then(|inner_path| inner_path.to_str()).map(|s| String::from(s)).unwrap_or(teste);
            editor::resize(&mut watermark, 500, 500, ResizeMode::Fit);
            let watermarked_image = editor::blend(&image, &watermark, BlendMode::Normal, 1.0, PositionMode::Center, 0, 0).unwrap();
            raster::save(&watermarked_image, &file_name).unwrap();
            println!("Saved {:?}", &file_name);
        });

    }

    pool.join();
    Ok(())
}
