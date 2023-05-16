# Cryptoset1

Problem Set 1
Submit your answers in a simple text file or as a link to a google doc with public access.
Find a collision in each of the hash functions below:
H(x) = x mod 512, where x can be any integer
H(x) = number of 0-bits in x, where x can be any bit string
Note: a “bit string” is simply a sequence of 0s and 1s
For example, 01011011 is a bit-string
H(x) = the five least significant bits of x, where x can be any bit string
Assume the bits in the bit string are ordered in little endian encoding, where the least significant bit is on the left. For example, in the bit string 01010, the least significant two bits is 01.
Please read the wikipedia article on little endian vs big endian encoding.
Little endian encoding of the number 4 is 001 and its big endian encoding 100.
Example:
Suppose the little endian encoding of x is 011010111010110
Then, the five least significant bits of x is 01101
Suppose the little endian encoding of x is 011
Then, the five least significant bits of x is 01100
Two different ways to order the bits in the bit string
Little endian: Least significant bit on the left, and most significant bit on the right
Big endian: Most signifFlex Appealicant bit on the left, and least significant bit on the right
Little endian and big endian are the reverse of each other
In the decimal number 539, we use big endian to represent the number in decimal.
Back to the question, suppose we have function F that, when we give at x, it gives us only the 5 least significant of x.
F(11001011010) = 11001
F(110) = 11000

Implement a program to find an x such that H (x ◦ id) ∈ Y where
H = SHA-256
id = 0xED00AF5F774E4135E7746419FEB65DE8AE17D6950C95CEC3891070FBB5B03C77
Y is the set of all 256 bit values that have some byte with the value 0x1D.
Assume SHA-256 is puzzle-friendly. Your answer for x must be in hexadecimal. 
Use any language you’d like, however we highly recommend using RUST as it is a critical language in blockchain:
Learn the necessary RUST by reading chapters 1-4 of the Rust Lang book and the Rust By Example reference. Use the following RUST crates: https://crates.io/crates/hex, https://crates.io/crates/sha2, https://crates.io/crates/rand
Caution:  
The notation “x ◦ id” means the byte array x concatenated with the byte array id. For example, 11110000 ◦ 10101010 is the byte array 1111000010101010.
The following two code segments are not equivalent:
INCORRECT
CORRECT
let id_hex = “1D253A2F";

if id_hex.contains("1D") {
   return;
}
let id_hex = “1D253A2F";

let decoded = hex::decode(id_hex).expect("Decoding failed");
let u = u8::from(29); //29 in decimal is 0x1d in hex
if decoded.contains(&u) {
   return;
}




The second code segment above is the correct way to check whether 0x1D is a byte in 0x1D253A2F. Remember that hex format is only a way to represent a byte sequence in a human readable format. You should never perform operations directly on hex-string representations. Instead, you should first convert hex-strings into byte arrays, then perform operations on the byte arrays directly, and then convert the final byte array into a hex format when giving your answer. Performing operations directly on the hex strings is incorrect.

Alice and Bob want to play a game over SMS text where Alice chooses a number between 1 and 10 in her head, and then Bob tries to guess that number. If Bob guesses correctly, he wins. Otherwise, Alice wins. However, Bob complains that the game isn’t fair, because even if Bob guessed correctly, Alice could lie and claim that she chose a different number than what she initially chose. What can Alice do to prove that she didn’t change the number she initially chose? Devise a mechanism to address Bob’s concern. Provide a detailed explanation of the mechanism and why it works. An answer with insufficient detail will not receive credit. You should only need to use cryptographic hash functions to solve this problem. Keep the solution simple.

