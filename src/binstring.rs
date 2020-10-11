use crate::array_byte_vec::ArrayByteVec;
use std::convert::TryInto;

const ALPHA_CODES: [u8; 32] = [
    b'3', // 00000
    b'H', // 00001
    b'G', // 00010
    b'F', // 00011
    b'R', // 00100
    b'6', // 00101
    b'8', // 00110
    b'I', // 00111
    b'Q', // 01000
    b'W', // 01001
    b'J', // 01010
    b'5', // 01011
    b'X', // 01100
    b'T', // 01101
    b'K', // 01110
    b'Z', // 01111
    b'A', // 10000
    b'Y', // 10001
    b'7', // 10010
    b'O', // 10011
    b'9', // 10100
    b'4', // 10101
    b'P', // 10110
    b'D', // 10111
    b'U', // 11000
    b'C', // 11001
    b'E', // 11010
    b'S', // 11011
    b'M', // 11100
    b'N', // 11101
    b'B', // 11110
    b'L', // 11111
];

const CKSUM_INTS: [u32; 100] = [
    608356525, 403119806, 600082856, 501903605, 395995676, 639983625, 520697153, 373011710,
    481025613, 609081731, 423005485, 362660979, 736545212, 334902753, 172235946, 995274085,
    518629176, 285694472, 421177161, 558665377, 324282222, 360158599, 878917102, 868418206,
    481304371, 975617486, 284706427, 927526243, 861246474, 241390708, 689060809, 976999545,
    989781527, 323910607, 635771485, 627673660, 112575133, 599103942, 422611191, 833680556,
    246423091, 877687856, 440233940, 674718290, 769786302, 251938542, 558968770, 695034124,
    588955750, 641879279, 772658258, 736177599, 680518529, 514691103, 897768422, 142764968,
    745785222, 940625572, 745145654, 621274084, 576319507, 202006004, 956856885, 225397510,
    908993516, 827097658, 824553838, 698243440, 287555233, 695171056, 611240526, 877537650,
    816292354, 830244639, 966764077, 166510545, 544190140, 337634650, 438485562, 786330704,
    164532512, 551752949, 258288608, 118631355, 581662189, 780678019, 183604061, 833274178,
    256709138, 377882729, 782366535, 739375209, 337783228, 848717587, 478815886, 903176896,
    466108725, 639641238, 486777548, 986302837,
];

#[derive(Debug)]
pub struct BinString(ArrayByteVec<90>);

impl BinString {
    pub fn from_alphanumeric(alnum: &[u8]) -> Self {
        let mut vec = ArrayByteVec::<90>::zeroed_with_len(alnum.len() * 5);
        for (i, alpha_val) in alnum.iter().enumerate() {
            let idx = ALPHA_CODES
                .iter()
                .position(|&v| v == *alpha_val)
                .unwrap_or(0);
            let pack: u8 = idx.try_into().unwrap();
            for n in 0..5 {
                vec[i * 5 + n as usize] = pack.nth_bit_from_left(n);
            }
        }
        unshuffle(&mut vec);
        Self(vec)
    }
    pub fn hash(&mut self, key: &BinString) {
        assert!(self.0.len() <= 90);
        for (sb, &kb) in self.0.iter_mut().zip(key.0.iter().cycle()) {
            if kb == 1 {
                if *sb == 0 {
                    *sb = 1;
                } else {
                    *sb = 0;
                }
            }
        }
    }
    pub fn reader(&self) -> Reader {
        Reader { source: &self.0 }
    }
    pub fn calc_checksum(&self) -> u32 {
        const DIGITS_TO_READ: u8 = crate::LEN * 5 - crate::CKSUM_BITS;
        let mut checksum = CKSUM_INTS[0];
        for i in 0..DIGITS_TO_READ {
            if self.0[i as usize] == 0 {
                checksum = checksum.wrapping_add(CKSUM_INTS[i as usize % CKSUM_INTS.len()]);
            } else {
                checksum = checksum.wrapping_add(CKSUM_INTS[(i as usize + 17) % CKSUM_INTS.len()]);
            }
        }
        checksum % 2u32.pow(crate::CKSUM_BITS as u32)
    }
}

pub fn hash_name(name: &[u8]) -> BinString {
    let filtered: Vec<u8> = name.iter().cloned().filter_map(hash_filter_map).collect();
    let mut hash_bin = BinString::from_alphanumeric(&filtered);
    // Avoid empty hash thingy
    hash_bin.0.insert(0, 0);
    hash_bin.0.insert(0, 1);
    hash_bin
}

pub struct Reader<'a> {
    source: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn next_int(&mut self, len: usize) -> u16 {
        let value = read_bin(&self.source[..len]);
        self.advance(len);
        value
    }
    pub fn advance(&mut self, len: usize) {
        self.source = &self.source[len..]
    }
    pub fn len(&self) -> usize {
        self.source.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn hash_filter_map(key: u8) -> Option<u8> {
    match key.to_ascii_uppercase() {
        b'0' | b'O' => Some(b'A'),
        b'1' | b'I' => Some(b'B'),
        b' ' | b'.' => None,
        c => Some(c),
    }
}

trait FiveBitPackExt {
    fn nth_bit_from_left(self, idx: u8) -> u8;
}

impl FiveBitPackExt for u8 {
    fn nth_bit_from_left(self, idx: u8) -> u8 {
        if self & 0b0001_0000 >> idx == 0 {
            0
        } else {
            1
        }
    }
}

#[test]
fn test_nth_bit_from_left() {
    assert_eq!(0b10110.nth_bit_from_left(0), 1);
    assert_eq!(0b10110.nth_bit_from_left(1), 0);
    assert_eq!(0b10110.nth_bit_from_left(2), 1);
    assert_eq!(0b10110.nth_bit_from_left(3), 1);
    assert_eq!(0b10110.nth_bit_from_left(4), 0);
}

fn unshuffle(input: &mut [u8]) {
    if input.len() == 90 {
        unshuffle_90(input);
    } else {
        let mut work_buffer = ArrayByteVec::<90>::zeroed_with_len(input.len());
        one_unshuffle::<5>(input, &mut work_buffer);
        one_unshuffle::<3>(&work_buffer, input);
        one_unshuffle::<2>(input, &mut work_buffer);
        input.copy_from_slice(&work_buffer);
    }
}

fn one_unshuffle<const PARTS: usize>(input: &[u8], output: &mut [u8]) {
    let mut strs = [&[][..]; PARTS];
    let mut l = 0;
    let mut r = 0;
    let mut leftover = input.len() % PARTS;
    for s in strs.iter_mut() {
        r += input.len() / PARTS;
        if leftover > 0 {
            r += 1;
            leftover -= 1;
        }
        *s = &input[l..r];
        l = r;
    }
    for (i, out) in output.iter_mut().enumerate().take(input.len()) {
        let j = i % PARTS;
        let value = if j % 2 == 0 {
            strs[j][strs[j].len() - i / PARTS - 1]
        } else {
            strs[j][i / PARTS]
        };
        *out = value;
    }
}

// Thank you Teddy for the awesome optimization!
fn unshuffle_90(input: &mut [u8]) {
    const INDEX_VEC: [usize; 90] = [
        2, 87, 81, 8, 14, 75, 33, 56, 62, 27, 21, 68, 38, 51, 45, 44, 50, 39, 69, 20, 26, 63, 57,
        32, 74, 15, 9, 80, 86, 3, 1, 88, 82, 7, 13, 76, 34, 55, 61, 28, 22, 67, 37, 52, 46, 43, 49,
        40, 70, 19, 25, 64, 58, 31, 73, 16, 10, 79, 85, 4, 0, 89, 83, 6, 12, 77, 35, 54, 60, 29,
        23, 66, 36, 53, 47, 42, 48, 41, 71, 18, 24, 65, 59, 30, 72, 17, 11, 78, 84, 5,
    ];
    let mut new = [0; 90];
    for i in 0..90 {
        new[i] = input[INDEX_VEC[i]];
    }
    input.copy_from_slice(&new);
}

fn read_bin(bin: &[u8]) -> u16 {
    let mut result = 0;
    for &digit in bin {
        result *= 2;
        if digit == 1 {
            result += 1;
        }
    }
    result
}

#[test]
fn test_one_unshuffle() {
    let mut work_buf_5 = [0; 5];
    let mut work_buf_15 = [0; 15];
    let input = [1, 0, 0, 0, 0];
    one_unshuffle::<5>(&input, &mut work_buf_5);
    assert_eq!(&work_buf_5, &[1, 0, 0, 0, 0,]);
    let input = [1, 1, 0, 0, 1];
    one_unshuffle::<5>(&input, &mut work_buf_5);
    assert_eq!(&work_buf_5, &[1, 1, 0, 0, 1,]);
    let input = [1, 1, 0, 0, 1];
    one_unshuffle::<3>(&input, &mut work_buf_5);
    assert_eq!(&work_buf_5, &[1, 0, 1, 1, 0,]);
    let input = [1, 0, 1, 1, 0];
    one_unshuffle::<2>(&input, &mut work_buf_5);
    assert_eq!(&work_buf_5, &[1, 1, 0, 0, 1,]);
    let input = [1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1];
    one_unshuffle::<5>(&input, &mut work_buf_15);
    assert_eq!(
        &work_buf_15,
        &[0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0,]
    );
    let input = [0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0];
    one_unshuffle::<3>(&input, &mut work_buf_15);
    assert_eq!(
        &work_buf_15,
        &[1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1,]
    );
    let input = [1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1];
    one_unshuffle::<2>(&input, &mut work_buf_15);
    assert_eq!(
        &work_buf_15,
        &[1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1,]
    );
}

#[test]
fn test_unshuffle() {
    let mut input = [1, 0, 0, 0, 0];
    unshuffle(&mut input);
    assert_eq!(&input, &[0, 1, 0, 0, 0]);
    let mut input = [1, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 1];
    unshuffle(&mut input);
    assert_eq!(&input, &[1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1]);
}

#[test]
fn test_from_alnum() {
    assert_eq!(&*BinString::from_alphanumeric(b"A").0, &[0, 1, 0, 0, 0]);
    assert_eq!(&*BinString::from_alphanumeric(b"B").0, &[0, 1, 1, 1, 1]);
    assert_eq!(&*BinString::from_alphanumeric(b"C").0, &[1, 1, 0, 0, 1]);
    assert_eq!(&*BinString::from_alphanumeric(b"0").0, &[0, 0, 0, 0, 0]);
    assert_eq!(&*BinString::from_alphanumeric(b"1").0, &[0, 0, 0, 0, 0]);
    assert_eq!(&*BinString::from_alphanumeric(b"2").0, &[0, 0, 0, 0, 0]);
    assert_eq!(
        &*BinString::from_alphanumeric(b"000").0,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"111").0,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"222").0,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"ABC").0,
        &[1, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"ABC012").0,
        &[
            1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0,
            0
        ]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"AABBCCDDEEFFGGHHII").0,
        &[
            0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 0,
            0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1,
            0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1,
            0, 1, 1
        ]
    );
    assert_eq!(
        &*BinString::from_alphanumeric(b"STUUBXGG5K8BY45ZN7").0,
        &[
            0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1,
            1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1,
            1, 1, 0
        ]
    );
}

#[test]
fn test_hash() {
    let mut bs = BinString::from_alphanumeric(b"QWERTYUIOPASDFGHJK");
    assert_eq!(
        &*bs.0,
        &[
            0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1,
            0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1,
            0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1,
            0, 0, 0
        ]
    );
    bs.hash(&hash_name(b"Dew"));
    assert_eq!(
        &*bs.0,
        &[
            1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0,
            0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0,
            0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1,
            0, 1, 1
        ]
    );
}

#[test]
fn test_read_bin() {
    assert_eq!(read_bin(&[0]), 0);
    assert_eq!(read_bin(&[1]), 1);
    assert_eq!(read_bin(&[0, 0, 1]), 1);
    assert_eq!(read_bin(&[1, 1, 1]), 7);
    assert_eq!(read_bin(&[0, 1, 1, 1, 1, 1]), 31);
    assert_eq!(read_bin(&[1, 1, 0]), 6);
    assert_eq!(read_bin(&[0, 1, 1, 1, 0, 1, 0]), 58);
    assert_eq!(read_bin(&[0, 1, 1, 1, 0, 0, 0]), 56);
    assert_eq!(read_bin(&[0, 1, 1, 0, 1, 0, 1, 0, 1]), 213);
    assert_eq!(read_bin(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]), 1);
}
