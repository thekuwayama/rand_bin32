use base64;
use getrandom::getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub fn random_bin32_base64() -> Result<JsValue, JsValue> {
    let mut buf = [0u8; 32];
    if let Err(_) = getrandom(&mut buf) {
        return Err(JsValue::from("getrandom error"));
    }

    Ok(JsValue::from(base64::encode(buf)))
}

#[wasm_bindgen]
pub fn random_bin32_hex() -> Result<JsValue, JsValue> {
    let mut buf = [0u8; 32];
    if let Err(_) = getrandom(&mut buf) {
        return Err(JsValue::from("getrandom error"));
    }

    Ok(JsValue::from(
        buf.iter().map(|x| format!("{:02x}", x)).collect::<String>(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_random_bin32_base64() {
        let result = random_bin32_base64();
        assert!(result.is_ok());

        let s = result.unwrap();
        assert!(s.is_string());
        assert_eq!(s.as_string().unwrap().len(), 44);
    }

    #[wasm_bindgen_test]
    fn test_random_bin32_hex() {
        let result = random_bin32_hex();
        assert!(result.is_ok());

        let s = result.unwrap();
        assert!(s.is_string());
        assert_eq!(s.as_string().unwrap().len(), 64);
    }
}
