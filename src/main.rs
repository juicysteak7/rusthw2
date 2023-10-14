extern crate rand;
use rand::Rng;

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..(n / 2 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn generate_prime(bits: u32) -> u64 {
    let mut rng = rand::thread_rng();
    let min_value = 2u64.pow(bits - 1);
    let max_value = 2u64.pow(bits) - 1;
    loop {
        let candidate = rng.gen_range(min_value..=max_value);
        if is_prime(candidate) {
            return candidate;
        }
    }

}

fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let a = a as i64;
    let m = m as i64;

    if m <= 0 {
        return None; // Modular inverse doesn't exist
    }

    let mut t = 0i64;
    let mut newt = 1i64;
    let mut r = m;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;

        let tmp = t;
        t = newt;
        newt = tmp - quotient * newt;

        let tmp = r;
        r = newr;
        newr = tmp - quotient * newr;
    }

    if r > 1 {
        return None; // Modular inverse doesn't exist
    }

    if t < 0 {
        t += m;
    }

    Some(t as u64)
}

fn modexp(x:u64,y:u64,m:u64) -> u64 {
    assert!(m != 0);
    let mut z:u128 = 1;
    let mut y:u128 = y as u128;
    let mut x:u128 = x as u128;
    let m:u128 = m as u128;

    // Overflow check
    if (m-1).checked_mul(m-1).is_none() {
        eprintln!("m overflow caught.");
        error();
    }

    if m == 0 {
        error();
    }


    while y > 0 {
        if y % 2 == 1{
            // Overflow check
            match z.checked_mul(x) {
                Some(result) => {
                    z = result % m;
                }
                None => {
                    eprintln!("z overflow caught");
                    error()
                }
            }
        }
        y /= 2 ;
        // Overflow check
        match x.checked_mul(x) {
            Some(result) => {
                x = result % m;
            }
            None => {
                eprintln!("x overflow caught");
                error();
            }
        }
    }

    // Make sure z is small enough to fit as u64.
    if z > u64::MAX as u128 {
        eprintln!("z is larger than u64 max.");
        error();
    }
    z.try_into().unwrap()
}

fn main() {
    let bits = 16; // Adjust the number of bits as needed

    let p = generate_prime(bits);
    let q = generate_prime(bits);
    let n = p * q;
    let totient = (p - 1) * (q - 1);
    let e = 65537; // A common choice for the public exponent
    let d = mod_inverse(e, totient).expect("Modular inverse does not exist");
    // println!("p: {}, q: {}", p,q);
    // println!("n: {}, totient: {}", n,totient);
    // println!("e: {}, d: {}", e,d);

    let message = 42;
    let encrypted = modexp(message, e, n);
    let decrypted = modexp(encrypted, d, n);

    println!("Original: {}", message);
    println!("Encrypted: {}", encrypted);
    println!("Decrypted: {}", decrypted);
}

// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp: usage: cargo run <x:u64> <y:u64> <m:u64>");
    std::process::exit(1);
}