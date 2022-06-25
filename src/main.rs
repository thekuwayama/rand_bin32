fn main() {
    let mut buf = [0u8; 32];
    let _ = getrandom::getrandom(&mut buf);
    println!("{}", base64::encode(buf));
}
