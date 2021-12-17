mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn reduce_waveform_data(numbers: &[f32], size: usize) -> Vec<f32> {
    if numbers.len() == 0 {
        return Vec::new();
    }

    normalize(&downsample(&numbers, size))
}

#[wasm_bindgen]
pub fn reduce_waveform_data_ptr(numbers: &[f32], size: usize) -> *const f32 {
    reduce_waveform_data(&numbers, size).as_ptr()
}

fn downsample(data: &[f32], size: usize) -> Vec<f32> {
    let chunk_size = (data.len() / size) as usize;
    let chunks: Vec<&[f32]> = data.chunks(chunk_size).collect();

    chunks.iter().map(|chunk| average(chunk)).collect()
}

fn normalize(numbers: &[f32]) -> Vec<f32> {
    numbers
        .iter()
        .map(|n| {
            n * numbers
                .iter()
                .fold(0f32, |max, &val| if val > max { val } else { max })
                .powi(-1)
        })
        .collect()
}

fn average(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() / numbers.len() as f32
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_normalize_dataset_to_one() {
        let given = [0.0, 0.25, 0.5];
        let expected = [0.0, 0.5, 1.0];

        assert_eq!(reduce_waveform_data(&given, 3), expected);
    }

    #[test]
    fn test_reduces_dataset_by_size() {
        let given = [0.0, 0.5, 0.5, 1.0];
        let expected = Vec::from([1.0 / 3.0, 1.0]);

        assert_eq!(reduce_waveform_data(&given, given.len() / 2), expected);
    }
}
