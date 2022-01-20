# Shamir's Secret Sharing

This is an implementation of [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir's_Secret_Sharing) in Rust. The idea is to be able to split up a [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) based seed phrase into segments from which the seed phrase can be reconstructed if one has access to a predefined number of segments.


## Should you use it

Here are a few things to think about:
* This is a hobby project to learn the programming language Rust.
* Never use cryptographic algorithms implemented by yourself or an anonymous person on the internet in production. Only trust well known and tested implementations.
* Did you make sure that this program does not just send your seed phrase to an external server?
* The usefulness of Shamir's secret sharing for securing seed phrases is [disputed](https://en.bitcoin.it/wiki/Shamir_Secret_Snakeoil) and introduces [additional attack vectors](https://blog.keys.casa/shamirs-secret-sharing-security-shortcomings/).
* Initially the implementation will use integer arithmetics instead of using finite fields. So each piece of the secret an attacker gets will leak a small amount of information about the secret itself. An attacker can therefore start to guess the secret before they have the last piece needed.
* The pseudo random number generator used her is not cryptographically secure.

If you are still here, you can run the program with
```
cargo run
```
and test that everything works using
```
cargo test
```

