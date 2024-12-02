const JHASH_INITVAL: u32 = 0xdeadbeef;

#[inline(always)]
pub(crate) fn jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c);
    *a ^= c.rotate_left(4);
    *c = c.wrapping_add(*b);

    *b = b.wrapping_sub(*a);
    *b ^= a.rotate_left(6);
    *a = a.wrapping_add(*c);

    *c = c.wrapping_sub(*b);
    *c ^= b.rotate_left(8);
    *b = b.wrapping_add(*a);

    *a = a.wrapping_sub(*c);
    *a ^= c.rotate_left(16);
    *c = c.wrapping_add(*b);

    *b = b.wrapping_sub(*a);
    *b ^= a.rotate_left(19);
    *a = a.wrapping_add(*c);

    *c = c.wrapping_sub(*b);
    *c ^= b.rotate_left(4);
    *b = b.wrapping_add(*a);
}

pub(crate) fn jhash_final(mut a: u32, mut b: u32, mut c: u32) -> u32 {
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(14));

    a ^= c;
    a = a.wrapping_sub(c.rotate_left(11));

    b ^= a;
    b = b.wrapping_sub(a.rotate_left(25));

    c ^= b;
    c = c.wrapping_sub(b.rotate_left(16));

    a ^= c;
    a = a.wrapping_sub(c.rotate_left(4));

    b ^= a;
    b = b.wrapping_sub(a.rotate_left(14));

    c ^= b;
    c = c.wrapping_sub(b.rotate_left(24));
    c
}

pub(crate) fn jhash2(mut key: &[u32], initval: u32) -> u32 {
    let mut a = JHASH_INITVAL
        .wrapping_add(key.len() as u32)
        .wrapping_add(initval);
    let mut b = a;
    let mut c = a;

    /* Handle most of the key */
    while key.len() > 3 {
        a = a.wrapping_add(key[0]);
        b = b.wrapping_add(key[1]);
        c = c.wrapping_add(key[2]);
        jhash_mix(&mut a, &mut b, &mut c);
        key = &key[3..];
    }

    match key.len() {
        3 => {
            c = c.wrapping_add(key[2]);
            b = b.wrapping_add(key[1]);
            a = a.wrapping_add(key[0]);
        }
        2 => {
            b = b.wrapping_add(key[1]);
            a = a.wrapping_add(key[0]);
        }
        1 => {
            a = a.wrapping_add(key[0]);
        }
        0 => {
            return c;
        }
        _ => {
            unreachable!("Never happen");
        }
    }
    jhash_final(a, b, c)
}
