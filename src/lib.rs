use base64;
use getrandom::{getrandom, Error};

pub fn random_bin32_base64() -> Result<String, Error> {
    let mut buf = [0u8; 32];
    getrandom(&mut buf)?;
    Ok(base64::encode(buf))
}

pub fn random_bin32_hex() -> Result<String, Error> {
    let mut buf = [0u8; 32];
    getrandom(&mut buf)?;
    Ok(buf.iter().map(|x| format!("{:02x}", x)).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_bin32_base64() {
        let s = random_bin32_base64();
        assert!(s.is_ok());
        assert_eq!(s.unwrap().len(), 44);
    }

    #[test]
    fn test_random_bin32_hex() {
        let s = random_bin32_hex();
        assert!(s.is_ok());
        assert_eq!(s.unwrap().len(), 64);
    }
}
