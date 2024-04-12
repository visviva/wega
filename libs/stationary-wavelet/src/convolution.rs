use crate::{kernel::LinearInterpolation, wavelet_transform::StationaryWaveletTransform};

pub trait Convolution {
    fn compute_pixel_index(
        &self,
        distance: usize,
        kernel_index: [isize; 2],
        target_pixel_index: [usize; 2],
    ) -> [usize; 2];

    fn compute_convoluted_pixel(&self, distance: usize, index: [usize; 2]) -> f32;
}

impl Convolution for StationaryWaveletTransform {
    fn compute_pixel_index(
        &self,
        distance: usize,
        kernel_index: [isize; 2],
        target_pixel_index: [usize; 2],
    ) -> [usize; 2] {
        let [kernel_x, kernel_y] = kernel_index;

        //Compute distance from adjacent pixel
        let x_dist = kernel_x * distance as isize;
        let y_dist = kernel_y * distance as isize;

        let [pixel_x, pixel_y] = target_pixel_index;

        // Advance from target center pixel
        let mut x = pixel_x as isize + x_dist;
        let mut y = pixel_y as isize + y_dist;

        // Boundary conditions
        if x < 0 {
            x = 0;
        } else if x > self.image_width as isize - 1 {
            x = self.image_width as isize - 1;
        }

        if y < 0 {
            y = 0;
        } else if y > self.image_height as isize - 1 {
            y = self.image_height as isize - 1;
        }

        [y as usize, x as usize]
    }

    fn compute_convoluted_pixel(&self, distance: usize, [x, y]: [usize; 2]) -> f32 {
        let mut pixels_sum = 0.0;

        let kernel = LinearInterpolation::default();

        for kernel_x in -1..=1 {
            for kernel_y in -1..=1 {
                let pixel_index = self.compute_pixel_index(distance, [kernel_x, kernel_y], [x, y]);

                let normalized_x = (kernel_x + kernel.middle_index() as isize) as usize;
                let normalized_y = (kernel_y + kernel.middle_index() as isize) as usize;

                let kernel_value = kernel.index([normalized_x, normalized_y]);

                pixels_sum += kernel_value * self.input[pixel_index];
            }
        }

        pixels_sum
    }
}
