use clap::Parser;
use image::{DynamicImage, ImageBuffer, Luma};
use lib_stationary_wavelet::{self, recompose::RecomposableLayers};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the original image
    original_path: String,

    ///Path to the filtered image
    filtered_path: String,

    /// Number of level the image is decomposed into
    #[arg(short, long, default_value_t = 7)]
    number_of_levels: u32,

    /// The level until filtering is applied
    #[arg(short, long, default_value_t = 3)]
    apply_filter_until_level: u32,
}

fn main() {
    let args = Args::parse();

    // let image = image::open("test.jpg").unwrap();
    let image = image::open(args.original_path).unwrap();

    let transform = lib_stationary_wavelet::wavelet_transform::StationaryWaveletTransform::new(
        &image,
        args.number_of_levels as usize,
    );

    transform
        .map(|(mut buffer, pixel_scale)| {
            let mut new_buffer = ImageBuffer::<Luma<u16>, Vec<u16>>::new(
                buffer.ncols() as u32,
                buffer.nrows() as u32,
            );

            for (x, y, pixel) in new_buffer.enumerate_pixels_mut() {
                *pixel = Luma([(buffer[[y as usize, x as usize]] * u16::MAX as f32) as u16])
            }

            // Apply noise reduction for the background structures only
            if pixel_scale.is_some_and(|scale| scale < args.apply_filter_until_level as usize) {
                let mut image = DynamicImage::ImageLuma16(new_buffer).to_luma8();

                image = imageproc::filter::bilateral_filter(&image, 10, 10., 3.);

                for (x, y, pixel) in image.enumerate_pixels() {
                    buffer[[y as usize, x as usize]] = pixel.0[0] as f32 / u8::MAX as f32;
                }

                (buffer, pixel_scale)
            } else {
                (buffer, pixel_scale)
            }
        })
        .recompose_into_image(image.width() as usize, image.height() as usize)
        .to_luma8()
        .save(args.filtered_path)
        .unwrap();
}
