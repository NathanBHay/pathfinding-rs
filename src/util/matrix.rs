//! Matrix utilities.

/// Convolve a matrix with a kernel.
/// * `matrix` - The matrix to convolve.
/// * `kernel` - The kernel to convolve with.
/// * `resolution` - The resolution to use when resolving the matrix at the edges.
pub fn convolve2d_with_resolution(
    matrix: &[Vec<f32>],
    kernel: &[Vec<f32>],
    resolution: ConvResolve,
) -> Vec<Vec<f32>> {
    let mut result = vec![vec![0.0; matrix[0].len()]; matrix.len()];
    let kernel_width = kernel.len();
    let kernel_height = kernel[0].len();
    let kernel_center_x = kernel_width / 2;
    let kernel_center_y = kernel_height / 2;
    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            let mut sum = 0.0;
            for i in 0..kernel_width {
                for j in 0..kernel_height {
                    let matrix_x = (x + i) as i64 - kernel_center_x as i64;
                    let matrix_y = (y + j) as i64 - kernel_center_y as i64;
                    sum += resolution.resolve(matrix, matrix_x, matrix_y) * kernel[i][j];
                }
            }
            result[x][y] = sum;
        }
    }
    result
}

/// Resolution method for resolving the value of a matrix at the edges.
pub enum ConvResolve {
    /// Fill the matrix with the given value.
    Fill(f32),
    /// Wrap around the matrix.
    Wrap,
    /// Fill the matrix with the value at the edge rounded to the nearest integer.
    Symmetric,
}

impl ConvResolve {
    /// Resolve the value of a matrix at a given position.
    fn resolve(&self, matrix: &[Vec<f32>], matrix_x: i64, matrix_y: i64) -> f32 {
        match self {
            ConvResolve::Fill(fill) => 
                if matrix_x >= 0 && matrix_y >= 0 && matrix_x < matrix.len() as i64 && matrix_y < matrix[0].len() as i64 {
                    matrix[matrix_x as usize][matrix_y as usize]
                } else {
                    *fill
                },
            ConvResolve::Wrap => {
                let matrix_x = (matrix_x + matrix.len() as i64) % matrix.len() as i64;
                let matrix_y = (matrix_y + matrix[0].len() as i64) % matrix[0].len() as i64;
                matrix[matrix_x as usize][matrix_y as usize]
            },
            ConvResolve::Symmetric => {
                let matrix_x = matrix_x.clamp(0, matrix.len() as i64 - 1);
                let matrix_y = matrix_y.clamp(0, matrix[0].len() as i64 - 1);
                matrix[matrix_x as usize][matrix_y as usize].round()
            },
        }
    }
}

/// Create a gaussian kernel.
/// * `size` - The size of the kernel.
/// * `sigma` - The sigma value of the gaussian.
pub fn gaussian_kernal(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let center = size / 2;
    let sigma = sigma * sigma;
    let mut sum = 0.0;
    for i in 0..size {
        for j in 0..size {
            let x = i as i32 - center as i32;
            let y = j as i32 - center as i32;
            let value = (-((x * x + y * y) as f32) / (2.0 * sigma)).exp();
            sum += value;
            kernel[i][j] = value;
        }
    }
    for i in 0..size {
        for j in 0..size {
            kernel[i][j] /= sum;
        }
    }
    kernel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolve2d_fill() {
        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let kernel = vec![
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let result = convolve2d_with_resolution(&matrix, &kernel, ConvResolve::Fill(0.0));
        assert_eq!(result, vec![
            vec![12.0, 21.0, 16.0],
            vec![27.0, 45.0, 33.0],
            vec![24.0, 39.0, 28.0],
        ]);
    }

    #[test]
    fn test_convolve2d_wrap() {
        let matrix = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
        ];
        let kernel = vec![
            vec![1.0, 1.0],
            vec![1.0, 1.0],
        ];
        let result = convolve2d_with_resolution(&matrix, &kernel, ConvResolve::Wrap);
        assert_eq!(result, vec![
            vec![10.0, 10.0],
            vec![10.0, 10.0],
        ]);
    }

    #[test]
    fn test_gaussian_kernal() {
        let kernel = gaussian_kernal(3, 1.0);
        assert_eq!(kernel, vec![
            vec![0.07511361, 0.12384141, 0.07511361],
            vec![0.12384141, 0.20417996, 0.12384141],
            vec![0.07511361, 0.12384141, 0.07511361],
        ]);
    }
}
