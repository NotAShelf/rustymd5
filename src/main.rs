const K: [u32; 64] = [
    0xD76AA478, 0xE8C7B756, 0x242070DB, 0xC1BDCEEE, 0xF57C0FAF, 0x4787C62A, 0xA8304613, 0xFD469501,
    0x698098D8, 0x8B44F7AF, 0xFFFF5BB1, 0x895CD7BE, 0x6B901122, 0xFD987193, 0xA679438E, 0x49B40821,
    0xF61E2562, 0xC040B340, 0x265E5A51, 0xE9B6C7AA, 0xD62F105D, 0x02441453, 0xD8A1E681, 0xE7D3FBC8,
    0x21E1CDE6, 0xC33707D6, 0xF4D50D87, 0x455A14ED, 0xA9E3E905, 0xFCEFA3F8, 0x676F02D9, 0x8D2A4C8A,
    0xFFFA3942, 0x8771F681, 0x6D9D6122, 0xFDE5380C, 0xA4BEEA44, 0x4BDECFA9, 0xF6BB4B60, 0xBEBFBC70,
    0x289B7EC6, 0xEAA127FA, 0xD4EF3085, 0x04881D05, 0xD9D4D039, 0xE6DB99E5, 0x1FA27CF8, 0xC4AC5665,
    0xF4292244, 0x432AFF97, 0xAB9423A7, 0xFC93A039, 0x655B59C3, 0x8F0CCC92, 0xFFEFF47D, 0x85845DD1,
    0x6FA87E4F, 0xFE2CE6E0, 0xA3014314, 0x4E0811A1, 0xF7537E82, 0xBD3AF235, 0x2AD7D2BB, 0xEB86D391,
];

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const M: [usize; 64] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 1, 6, 11, 0, 5, 10, 15, 4, 9, 14, 3, 8,
    13, 2, 7, 12, 5, 8, 11, 14, 1, 4, 7, 10, 13, 0, 3, 6, 9, 12, 15, 2, 0, 7, 14, 5, 12, 3, 10, 1,
    8, 15, 6, 13, 4, 11, 2, 9,
];

const ABCD: [u32; 4] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476];

struct MD5Context {
    buffer: [u32; 16],
    state: [u32; 4],
}

impl MD5Context {
    fn step(&mut self, m: u32, k: u32, s: u32, c: fn(u32, u32, u32) -> u32) {
        let new_b = self.state[0]
            .wrapping_add(c(self.state[1], self.state[2], self.state[3]))
            .wrapping_add(m)
            .wrapping_add(k)
            .rotate_left(s)
            .wrapping_add(self.state[1]);

        self.state[0] = self.state[3];
        self.state[3] = self.state[2];
        self.state[2] = self.state[1];
        self.state[1] = new_b;
    }

    fn encode_message(&mut self, message: &str) {
        let mut encoded_message: Vec<u8> = message.into();
        encoded_message.push(0x80);

        let encoded_words = encoded_message.len().div_ceil(4);
        let message_bits = message.len() * 8;

        self.buffer[..encoded_words].copy_from_slice(u8_to_u32_array(&encoded_message).as_slice());
        self.buffer[14] = message_bits as u32;
        self.buffer[15] = (message_bits >> 32) as u32;
    }

    fn transform(&mut self) -> String {
        for j in 0..64 {
            match j {
                0..=15 => self.step(self.buffer[M[j]], K[j], S[j], f),
                16..=31 => self.step(self.buffer[M[j]], K[j], S[j], g),
                32..=47 => self.step(self.buffer[M[j]], K[j], S[j], h),
                48..=63 => self.step(self.buffer[M[j]], K[j], S[j], i),
                _ => (),
            }
        }
        let result: [u32; 4] = [
            self.state[0].wrapping_add(ABCD[0]),
            self.state[1].wrapping_add(ABCD[1]),
            self.state[2].wrapping_add(ABCD[2]),
            self.state[3].wrapping_add(ABCD[3]),
        ];

        result
            .iter()
            .flat_map(|word| word.to_ne_bytes())
            .map(|byte| format!("{:02x}", byte))
            .collect()
    }
}

fn f(b: u32, c: u32, d: u32) -> u32 {
    (b & c) | (!b & d)
}

fn g(b: u32, c: u32, d: u32) -> u32 {
    (b & d) | (c & !d)
}

fn h(b: u32, c: u32, d: u32) -> u32 {
    b ^ c ^ d
}

fn i(b: u32, c: u32, d: u32) -> u32 {
    c ^ (b | !d)
}

fn u8_to_u32_array(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks(4)
        .map(|chunk| {
            let mut buf = [0u8; 4];
            buf.iter_mut()
                .zip(chunk)
                .for_each(|(b, &chunk_b)| *b = chunk_b);
            u32::from_ne_bytes(buf)
        })
        .collect()
}

fn md5_init() -> MD5Context {
    MD5Context {
        buffer: [0u32; 16],
        state: ABCD,
    }
}

fn md5_digest(message: &str) -> String {
    let mut context = md5_init();
    context.encode_message(message);
    context.transform()
}

fn main() {
    let message = "Fuck you MD5";
    println!("Input: {message}");
    let hash = md5_digest(message);
    println!("MD5 Hash: {hash}");
}

#[test]
fn test_bitwise_operations() {
    assert_eq!(f(0x89abcdef, 0xfedcba98, 0x76543210), 0xfedcba98);
    assert_eq!(g(0x2c34dfa2, 0xde1673be, 0x4b976282), 0x9c1453be);
    assert_eq!(h(0xd5071367, 0xc058ade2, 0x63c603d7), 0x7699bd52);
    assert_eq!(i(0x7d502063, 0x8b3d715d, 0x1de3a739), 0x746109ba);
}

#[test]
fn test_expected_final_output() {
    assert_eq!(
        md5_digest("Fuck you MD5"),
        String::from("0cca3d88c27d3c9f6b8a3c025f638687")
    );
}
