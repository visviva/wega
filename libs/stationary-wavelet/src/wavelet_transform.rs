use image::GenericImageView;
use ndarray::Array2;

use crate::convolution::Convolution;

pub struct StationaryWaveletTransform {
    pub input: Array2<f32>,
    pub levels: usize,
    pub current_level: usize,
    pub image_width: usize,
    pub image_height: usize,
}

impl StationaryWaveletTransform {
    pub fn new(input: &image::DynamicImage, levels: usize) -> Self {
        let (width, height) = {
            let (w, h) = input.dimensions();
            (w as usize, h as usize)
        };

        let mut data = Array2::<f32>::zeros((height, width));

        for (x, y, pixel) in input.to_luma32f().enumerate_pixels() {
            data[[y as usize, x as usize]] = pixel.0[0];
        }

        Self {
            input: data,
            levels,
            current_level: 0,
            image_width: width,
            image_height: height,
        }
    }
}

impl Iterator for StationaryWaveletTransform {
    type Item = (Array2<f32>, Option<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let pixel_scale = self.current_level;
        self.current_level += 1;

        if pixel_scale > self.levels {
            return None;
        }

        if pixel_scale == self.levels {
            return Some((self.input.clone(), None));
        }

        let (width, height) = (self.image_width, self.image_height);

        let distance = 2_usize.pow(pixel_scale as u32);

        let mut current_data = Array2::<f32>::zeros((height, width));

        for x in 0..width {
            for y in 0..height {
                current_data[[y, x]] = self.compute_convoluted_pixel(distance, [x, y]);
            }
        }

        let final_data = self.input.clone() - &current_data;
        self.input = current_data;

        Some((final_data, Some(self.current_level)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageBuffer, Luma};

    #[test]
    fn test_new_stationary_wavelet_transform() {
        let img_x = 10;
        let img_y = 10;
        let image = ImageBuffer::<Luma<f32>, Vec<f32>>::from_fn(img_x, img_y, |x, y| {
            Luma([(x + y) as f32 / 2.0])
        });

        let image = DynamicImage::from(image);

        // Number of levels for the wavelet transform
        let levels = 3;

        // Create the wavelet transform object
        let swt = StationaryWaveletTransform::new(&image, levels);

        // Check that the dimensions are correctly set
        assert_eq!(swt.image_width, 10);
        assert_eq!(swt.image_height, 10);

        // Check the number of levels
        assert_eq!(swt.levels, levels);

        // Check the current level (should be initialized to 0)
        assert_eq!(swt.current_level, 0);

        // Optionally, check some values in the 'input' matrix
        // This assumes the input processing to DynamicImage and into Array2<f32> is correct.
        // Here we might check if the top left pixel (0,0) is correctly set
        assert_eq!(swt.input[[0, 0]], 0.0); // Adapt this based on your specific test image values
    }
}
