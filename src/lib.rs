use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calcular(num1: f32, num2: f32, op: &str) -> String {
    match op {
        "add" => format!("{:.2}", num1 + num2),
        "sub" => format!("{:.2}", num1 - num2),
        "mul" => format!("{:.2}", num1 * num2),
        "div" => if num2 != 0.0 { format!("{:.2}", num1 / num2) } else { "Error: Div por cero".to_string() },
        _ => "Error".to_string(),
    }
}