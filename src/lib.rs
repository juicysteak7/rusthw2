use toy_rsa_lib::rsa_prime;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of prime numbers in the range `2^31..2^32` suitable for RSA encryption.
///
/// This function generates a pair of prime numbers, `p` and `q`, within the specified range suitable for RSA encryption. The range is limited to 2^31 (2,147,483,648) to 2^32 (4,294,967,295).
///
/// # Returns
///
/// A tuple containing two prime numbers, `(p, q)`, where `p` and `q` are within the specified range.
///
/// # Example
///
/// ```
/// use toyrsa::genkey;
/// let (p, q) = genkey();
/// println!("Generated prime pair: ({}, {})", p, q);
/// ```
pub fn genkey() -> (u32, u32) {
    loop {
        let p = rsa_prime();
        let q = rsa_prime();

        let totient = (p - 1) as u64 * (q - 1) as u64;

        match mod_inverse(EXP, totient) {
            Some(_) => {}
            // mod inverse doesn't exist, need new prime pair.
            None => {
                continue;
            }
        }

        let lambda_pq: u64 = (p - 1) as u64 * (q - 1) as u64;

        if EXP < lambda_pq && gcd(EXP, lambda_pq) == 1 {
            return (p, q);
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Encrypt the plaintext `msg` using the RSA public `key` and return the ciphertext.
///
/// This function takes a plaintext message `msg` as a 32-bit unsigned integer and encrypts it using the RSA public key `key`. The result is the ciphertext.
///
/// # Arguments
///
/// * `key` - The RSA public key used for encryption.
/// * `msg` - The plaintext message to be encrypted, represented as a 32-bit unsigned integer.
///
/// # Returns
///
/// The ciphertext resulting from encrypting the `msg` using the RSA public key `key`.
///
/// # Example
///
/// ```
/// use toyrsa::encrypt;
/// let public_key = 65537; // Example RSA public key
/// let plaintext = 42; // Example plaintext message
/// let ciphertext = encrypt(public_key, plaintext);
/// println!("Ciphertext: {}", ciphertext);
/// ```
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg.try_into().unwrap(), EXP, key)
}

/// Decrypt the ciphertext `msg` using the RSA private `key` and return the resulting plaintext.
///
/// This function takes a ciphertext message `msg` and decrypts it using the RSA private key `key`. The result is the original plaintext message.
///
/// # Arguments
///
/// * `key` - The RSA private key used for decryption, represented as a tuple of two 32-bit unsigned integers (p and q).
/// * `msg` - The ciphertext to be decrypted, represented as a 64-bit unsigned integer.
///
/// # Returns
///
/// The original plaintext message obtained by decrypting the `msg` using the RSA private key `key`, represented as a 32-bit unsigned integer.
///
/// # Example
///
/// ```
/// use toyrsa::decrypt;
/// let private_key = (49169, 47111); // Example RSA private key
/// let ciphertext = 8497541; // Example ciphertext
/// let plaintext = decrypt(private_key, ciphertext);
/// println!("Plaintext: {}", plaintext);
/// ```
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let totient = (key.0 - 1) as u64 * (key.1 - 1) as u64;
    let d = mod_inverse(EXP, totient).unwrap();
    modexp(msg, d, key.0 as u64 * key.1 as u64)
        .try_into()
        .unwrap()
}

/// Performs modular exponentiation.
///
/// This function calculates `(x^y) % m` efficiently using the binary exponentiation method.
///
/// # Arguments
///
/// * `x` - The base value.
/// * `y` - The exponent.
/// * `m` - The modulus.
///
/// # Returns
///
/// The result of `(x^y) % m`.
///
/// # Panics
///
/// This function will panic if:
///
/// - `m` is equal to 0.
/// - The intermediate values exceed the maximum value for `u64`.
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    let mut z: u128 = 1;
    let mut y: u128 = y as u128;
    let mut x: u128 = x as u128;
    let m: u128 = m as u128;

    if m == 0 {
        error("m can't be zero");
    }
    assert!(m != 0);

    while y > 0 {
        if y % 2 == 1 {
            z = (z * x) % m;
        }
        y /= 2;
        x = (x * x) % m;
    }

    // Make sure z is small enough to fit as u64.
    z.try_into().unwrap()
}

/// Calculate the modular multiplicative inverse of `a` modulo `m`.
///
/// The modular multiplicative inverse of `a` modulo `m` is an integer `x` such that `(a * x) % m = 1`.
///
/// # Arguments
///
/// * `a` - The number for which the modular inverse is calculated.
/// * `m` - The modulus for the operation.
///
/// # Returns
///
/// If a modular inverse exists, the function returns `Some(x)`, where `x` is the modular inverse of `a` modulo `m`.
/// If no modular inverse exists (when `m` is non-positive or `a` is not coprime with `m`), the function returns `None`.
fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    // Convert a and m to signed integers (i64) to handle negative values
    let a = a as i64;
    let m = m as i64;

    // Check if m is non-positive (0 or negative), and if so, the modular inverse doesn't exist
    if m <= 0 {
        return None;
    }

    // Initialize variables for the extended Euclidean algorithm
    let mut t = 0i64; // t at step 0
    let mut newt = 1i64; // t at step 1
    let mut r = m; // r at step 0
    let mut newr = a; // r at step 1

    // Apply the extended Euclidean algorithm
    while newr != 0 {
        // Calculate the quotient of the current r and newr
        let quotient = r / newr;

        // Swap and update t values
        let tmp = t;
        t = newt;
        newt = tmp - quotient * newt;

        // Swap and update r values
        let tmp = r;
        r = newr;
        newr = tmp - quotient * newr;
    }

    // If the final r is greater than 1, the modular inverse doesn't exist
    if r > 1 {
        return None;
    }

    // Ensure that the result (t) is positive
    if t < 0 {
        t += m;
    }

    // Convert t back to u64 and return it as an Option
    Some(t as u64)
}

// Print a usage error message and exit.
fn error(e: &str) -> ! {
    eprintln!("Error: {}", e);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    // Test for mod inverse
    #[test]
    fn test_mod_inverse() {
        // Test a valid case
        let a = 3;
        let m = 11;
        assert_eq!(mod_inverse(a, m), Some(4));

        // Test when a is not coprime with m
        let a = 6;
        let m = 9;
        assert_eq!(mod_inverse(a, m), None);

        // Test when m is non-positive
        let a = 7;
        let m = 0;
        assert_eq!(mod_inverse(a, m), None);
    }

    // Test from hw1 for modexp
    #[test]
    fn test_modexp() {
        // Largest prime less than 2**64.
        // https://primes.utm.edu/lists/2small/0bit.html
        let bigm = u64::max_value() - 58;
        assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
        assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
        assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
        // https://practice.geeksforgeeks.org/problems/
        //    modular-exponentiation-for-large-numbers/0
        assert_eq!(4, modexp(10, 9, 6));
        assert_eq!(34, modexp(450, 768, 517));
    }

    // encrypt a random u32 10 times and check the result - tests decrypt, encrypt and genkey
    #[test]
    fn test_random_10_rsa_encryption_decryption() {
        for _ in 0..10 {
            // Generate an RSA key pair
            let key = genkey();

            // Generate a random u32 number
            let mut rng = rand::thread_rng(); // Initialize a random number generator
            let original_message: u32 = rng.gen();
            //let original_message: u32 = rng.gen();
            println!("message: {}", original_message);

            // Encrypt the random number
            let encrypted_message = encrypt(
                (key.0 as u64 * key.1 as u64).try_into().unwrap(),
                original_message as u32,
            );

            // Decrypt the encrypted message
            let decrypted_message = decrypt(key, encrypted_message);

            // Check if the decrypted message matches the original one
            assert_eq!(decrypted_message, original_message as u32);
        }
    }
}
