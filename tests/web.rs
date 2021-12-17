//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use waveform_utils::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_normalize_dataset_to_one() {
    let given = [0.0, 0.25, 0.5];
    let expected = [0.0, 0.5, 1.0];

    assert_eq!(reduce_waveform_data(&given, 3), expected);
}

#[wasm_bindgen_test]
fn test_reduces_dataset_by_size() {
    let given = [0.0, 0.5, 0.5, 1.0];
    let expected = Vec::from([1.0 / 3.0, 1.0]);

    assert_eq!(
        reduce_waveform_data(&given, (given.len() / 2) as u16),
        expected
    );
}
