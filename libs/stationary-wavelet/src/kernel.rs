#[derive(Copy, Clone)]
pub struct LinearInterpolation {
    values: [[f32; 3]; 3],
}

impl LinearInterpolation {
    const ONE_SIXTEENTH: f32 = 1. / 16.;
    const ONE_EIGHTH: f32 = 1. / 8.;
    const ONE_QUARTER: f32 = 1. / 4.;

    const DEFAULT_VALUES: [[f32; 3]; 3] = [
        [Self::ONE_SIXTEENTH, Self::ONE_EIGHTH, Self::ONE_SIXTEENTH],
        [Self::ONE_EIGHTH, Self::ONE_QUARTER, Self::ONE_EIGHTH],
        [Self::ONE_SIXTEENTH, Self::ONE_EIGHTH, Self::ONE_SIXTEENTH],
    ];

    pub fn index(&self, [x, y]: [usize; 2]) -> f32 {
        if x >= self.size() || y >= self.size() {
            panic!("Index out of bounds");
        }
        self.values[x][y]
    }

    pub fn size(&self) -> usize {
        self.values.len()
    }

    pub fn middle_index(&self) -> usize {
        self.size() / 2
    }
}

impl Default for LinearInterpolation {
    fn default() -> Self {
        Self {
            values: Self::DEFAULT_VALUES,
        }
    }
}
