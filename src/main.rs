use toyrsa::{decrypt, encrypt, genkey};

fn main() {
    //let message = 4198843561;
    let message = 1850209054;

    let key = genkey();
    let encrypted = encrypt(key.0 as u64 * key.1 as u64,message);
    //let decrypted = modexp(encrypted, d, n);
    let decrypted = decrypt(key, encrypted);

    println!("Original: {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}