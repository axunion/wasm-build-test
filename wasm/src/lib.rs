#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[derive(Clone)]
pub struct AdditionalData {
    pub field: i32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl AdditionalData {
    pub fn new() -> AdditionalData {
        AdditionalData { field: 42 }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(getter_with_clone))]
pub struct SampleData {
    pub int_value: i32,
    pub float_value: f64,
    pub bool_value: bool,
    pub string_value: String,
    pub array_value: Vec<i32>,
    pub additional_data: AdditionalData,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl SampleData {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> SampleData {
        SampleData {
            int_value: 10,
            float_value: 3.14,
            bool_value: true,
            string_value: "Hello, World!".to_string(),
            array_value: vec![1, 2, 3, 4],
            additional_data: AdditionalData::new(),
        }
    }

    pub fn get_int_value(&self) -> i32 {
        self.int_value
    }

    pub fn get_float_value(&self) -> f64 {
        self.float_value
    }

    pub fn get_bool_value(&self) -> bool {
        self.bool_value
    }

    pub fn get_string_value(&self) -> String {
        self.string_value.clone()
    }

    pub fn get_array_value(&self) -> Vec<i32> {
        self.array_value.clone()
    }

    pub fn get_additional_data(&self) -> AdditionalData {
        self.additional_data.clone()
    }
}
