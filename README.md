# Rust HW2
CS410 - Rust Programming

Professor Bart Massey

Pedro Gonzalez

# Background
“Public key cryptography” is super-important in the 21st century. The basic idea is an encryption scheme with two “keys”: a “public key” used for encryption that can be shared with anyone, and a corresponding “private key” that is the only way to decrypt. Thus, if Alice wants to send a secret message M to Bob, Alice can

    Look up Bob’s public key Kpub in a public key directory.
    Encrypt M using Kpub to get a ciphertext C.
    Send C to Bob in public.

Bob can then decrypt C using his corresponding private key Kpriv to get M. No one else has Kpriv, so no one else can decrypt C.

Normally, because reasons, the message sent by Alice will itself be just a number Ksymm that is the key in some “symmetric” cipher that Alice and Bob will switch to for future communications.

RSA (named for its creators Rivest, Shamir and Adleman) is the earliest public-key cryptosystem. It is still widely used today.