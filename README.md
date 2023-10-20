# Rust HW2
CS410 - Rust Programming

Professor Bart Massey

Pedro Gonzalez

# Background
“Public key cryptography” is super-important in the 21st century. The basic idea is an encryption scheme with two “keys”: a “public key” used for encryption that can be shared with anyone, and a corresponding “private key” that is the only way to decrypt. 

Thus, if Alice wants to send a secret message M to Bob, Alice canLook up Bob’s public key Kpub in a public key directory. Encrypt M using Kpub to get a ciphertext C. Send C to Bob in public.

Bob can then decrypt C using his corresponding private key Kpriv to get M. No one else has Kpriv, so no one else can decrypt C.

Normally, because reasons, the message sent by Alice will itself be just a number Ksymm that is the key in some “symmetric” cipher that Alice and Bob will switch to for future communications.

RSA (named for its creators Rivest, Shamir and Adleman) is the earliest public-key cryptosystem. It is still widely used today.

# Assignment
In this assignment, you will write a library crate that provides RSA key generation, encryption and decryption. I cannot emphasize enough that this RSA crate will be a toy exercise, and should not be used for anything that needs to be kept secure in real-life situations.

Start by carefully reading the Wikipedia page linked in the previous section. There’s a lot of number theory there, and you are not expected to understand that part. You are looking for the algorithms and pseudocode used for RSA.

Create a library crate called “toy-rsa” (the crate name is in the Cargo.toml.) This crate should provide the following interface:

    /// Fixed RSA encryption exponent.
    pub const EXP: u64 = 65_537;

    /// Generate a pair of primes in the range `2**31..2**32`
    /// suitable for RSA encryption with exponent.
    pub fn genkey() -> (u32, u32)

    /// Encrypt the plaintext `msg` using the RSA public `key`
    /// and return the ciphertext.
    pub fn encrypt(key: u64, msg: u32) -> u64

    /// Decrypt the cipertext `msg` using the RSA private `key`
    /// and return the resulting plaintext.
    pub fn decrypt(key: (u32, u32), msg: u64) -> u32

(Note that, as explained in the Background section above, the plaintext msg is just a 32-bit unsigned integer. No strings are involved in this assignment.)

# Project Notes
I started the logic behind this before I knew how to create a library so most of the development went into the main file, then was transfered once I learned how to make a library.

That being said that added more headache than I would have liked, given this assignment was focused on building a library I should have done that research initially.

Having experience with functional programming in Haskell has transfered well. There is a lot of similarities between them besides the fact they are built for completely different purposes (Haskell is more focused on research/education from what I understand). Not that Haskell isn't practical (I built an tcp based chat server using Haskell!), but I feel like Rust has the potential to become a more prevalent language.

Generating valid keys was a challenge that is worth mentioning. Getting some very large primes is easy, but it took me a while to get the mod inverse function right and to check the keys were valid before returning.

Once you have modexp and modinverse, and genkey() the rest really fell into place.

Rust doc comments... Amazing.

In order to test my code I figured I would generate some random u32 numbers a specific number of times and run it through the encryption/decryption loop and validate the original message matches the decrypted message.

That tested the main functions well enough genkey(), encrypt(), decrypt(). I also wanted to test the non public functions that do the heavy lifting, modexp() and mod_inverse(). I re-used the tests from hw1 for modexp and made a test for mod_inverse.