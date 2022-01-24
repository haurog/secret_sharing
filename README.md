# Shamir's Secret Sharing

This is an implementation of [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir's_Secret_Sharing) in Rust. The idea is to be able to split up a [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki) based seed phrase into segments from which the seed phrase can be reconstructed if one has access to a predefined number of segments.


## Should you use it

Here are a few things to think about:
* This is a hobby project to learn the programming language Rust.
* Never use cryptographic algorithms implemented by yourself or an anonymous person on the internet in production. Only trust well known and tested implementations.
* I am not a cryptographer, so some subtleties which are obvious to the trained eye might elude me and compromise the security of the implementation.
* Did you make sure that this program does not just send your seed phrase to an external server?
* The usefulness of Shamir's secret sharing for securing seed phrases is [disputed](https://en.bitcoin.it/wiki/Shamir_Secret_Snakeoil) and introduces [additional attack vectors](https://blog.keys.casa/shamirs-secret-sharing-security-shortcomings/).
* Initially the implementation will use integer arithmetics instead of using finite fields. So each piece of the secret an attacker gets will leak a small amount of information about the secret itself. An attacker can therefore start to guess the secret before they have the last piece needed.
* The pseudo random number generator used her is not cryptographically secure.

And to add an additional warning, here is a quote by Gregory Maxwell:

> "I think Shamir's Secret Sharing (and a number of other things, RNGs for example), suffer from a property where they are just complex enough that people are excited to implement them often for little good reason, and then they are complex enough (or have few enough reasons to invest significant time) they implement them poorly." 

If you are still here, you can run the program with
```
cargo run
```
and test that everything works using
```
cargo test
```

