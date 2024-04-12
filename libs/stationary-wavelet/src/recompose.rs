use image::{DynamicImage, ImageBuffer, Luma};
use ndarray::Array2;

pub trait RecomposableLayers: Iterator<Item = (Array2<f32>, Option<usize>)> {
    fn recompose_into_image(self, width: usize, height: usize) -> DynamicImage
    where
        Self: Sized,
    {
        let mut result = Array2::<f32>::zeros((height, width));

        for layer in self {
            result += &layer.0;
        }

        let min_pixel = result.iter().copied().reduce(f32::min).unwrap();
        let max_pixel = result.iter().copied().reduce(f32::max).unwrap();

        let mut result_img: ImageBuffer<Luma<u16>, Vec<u16>> =
            ImageBuffer::new(width as u32, height as u32);

        let rescale_ratio = max_pixel - min_pixel;

        for (x, y, pixel) in result_img.enumerate_pixels_mut() {
            let intensity = result[(y as usize, x as usize)];

            *pixel = Luma([((intensity - min_pixel) / rescale_ratio * u16::MAX as f32) as u16]);
        }

        DynamicImage::ImageLuma16(result_img)
    }
}

impl<T> RecomposableLayers for T where T: Iterator<Item = (Array2<f32>, Option<usize>)> {}
