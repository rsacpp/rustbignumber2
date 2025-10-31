/*
 */
use log::info;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use tokio::sync::Semaphore;

const _BITS0X20: u64 = 0xffffffff;
const _SIZE: usize = 0x40;
const _HALF_SIZE: u32 = 0x20;

pub struct Bn64 {
    _len: usize,
    _dat: Vec<u64>,
    _tag: usize,
}
/*
impl Drop for Bn64 {
    fn drop(&mut self) {
        info!("Dropping Bn64 {:p}", &self);
    }
}
*/
impl Bn64 {
    pub fn new(len: usize) -> Bn64 {
        Bn64 {
            _len: len,
            _dat: vec![0; len],
            _tag: 0,
        }
    }

    pub fn from(raw_text: String) -> Bn64 {
        let raw_len: usize = raw_text.len();
        let mut length: usize = raw_len / 0x10;
        if (raw_len % 0x10) > 0 {
            length += 1;
        }
        let mut dat: Vec<u64> = vec![0; length];
        for index in 0..raw_len {
            let ch: char = raw_text.chars().nth(index).unwrap();
            if ch.is_ascii_hexdigit() {
                let val: u64 = ch.to_digit(0x10).unwrap() as u64;
                let (result, _) = val.overflowing_shl((index % 0x10) as u32 * 4);
                dat[index / 0x10] += result;
            }
        }
        Bn64 {
            _len: length,
            _dat: dat,
            _tag: 0,
        }
    }

    pub fn to_hex(&self) {
        let mut text: String = String::new();
        for index in 0..self._len {
            let val: u64 = self._dat[index];
            let f: u32 = 0xf;
            for offset in 0..0x10 {
                let (result, _) = val.overflowing_shr(offset as u32 * 4);
                let d = result as u32 & f;
                let ch: char = char::from_digit(d, 0x10).unwrap();
                text.push(ch);
            }
        }
        info!("{text}");
    }

    pub fn shrink(&mut self) {
        let mut index = self._len - 1;
        while index > 0 {
            if self._dat[index] == 0 {
                index -= 1;
            } else {
                break;
            }
        }
        self._len = index + 1;
    }

    pub fn bits(&mut self) -> usize {
        self.shrink();
        let mut length = self._len * _SIZE;
        let mut gauge: u64 = 0x8000000000000000;
        let v: u64 = self._dat[self._len - 1];
        while (v & gauge) == 0 && v > 0 {
            let (result, _) = gauge.overflowing_shr(1);
            gauge = result;
            length -= 1;
        }
        return length;
    }
    pub fn bit(&mut self, index: usize) -> bool {
        let external_offset = index / _SIZE;
        let internal_offset = index % _SIZE;
        let v: u64 = 0x1 << internal_offset;
        if (self._dat[external_offset] & v) > 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn add_at(&mut self, index: usize, input: u64) {
        let value = self._dat[index];
        let (result, overflow) = value.overflowing_add(input);
        self._dat[index] = result;
        if overflow {
            self.add_at(index + 1, 1);
        }
    }

    pub fn sub_at(&mut self, index: usize, input: u64) {
        let value = self._dat[index];
        let (result, overflow) = value.overflowing_sub(input);
        self._dat[index] = result;
        if overflow {
            self.sub_at(index + 1, 1);
        }
    }

    /*
    self > bn => 1;
    self < bn => -1;
    self == bn => 0;
    */
    pub fn cmp(&mut self, bn: &mut Bn64) -> i8 {
        self.shrink();
        bn.shrink();
        if self._len > bn._len {
            return 1;
        }
        if self._len < bn._len {
            return -1;
        }
        let mut index = self._len - 1;
        while index > 0 && self._dat[index] == bn._dat[index] {
            index -= 1;
        }
        if self._dat[index] > bn._dat[index] {
            return 1;
        }
        if self._dat[index] < bn._dat[index] {
            return -1;
        }
        return 0;
    }
    /*self << bits*/
    pub fn left_push(&mut self, bits: usize) -> Bn64 {
        if bits == 0 {
            return self.clone();
        }
        let external_offset: usize = bits / _SIZE;
        let internal_offset: usize = bits % _SIZE;
        let length = self._len + external_offset;
        if internal_offset == 0 {
            /* push without splitting the elements */
            let mut bn: Bn64 = Bn64::new(length);
            for index in 0..self._len {
                bn.add_at(index + external_offset, self._dat[index]);
            }
            return bn;
        } else {
            /* push with splitting the elements */
            let mut bn: Bn64 = Bn64::new(length + 1);
            for index in 0..self._len {
                let (left_shifted, _) = self._dat[index].overflowing_shl(internal_offset as u32);
                bn.add_at(index + external_offset, left_shifted);
                let (right_shifted, _) =
                    self._dat[index].overflowing_shr((_SIZE - internal_offset) as u32);
                bn.add_at(index + external_offset + 1, right_shifted);
            }
            bn.shrink();
            return bn;
        }
    }
    /*self - bn;*/
    pub fn sub(&mut self, bn: &mut Bn64) -> Bn64 {
        self.shrink();
        bn.shrink();
        let mut result: Bn64 = Bn64::new(self._len);
        for index in 0..self._len {
            result.add_at(index, self._dat[index]);
        }
        for index in 0..bn._len {
            result.sub_at(index, bn._dat[index]);
        }
        result.shrink();
        return result;
    }
    /*self * bn;*/
    pub fn mul(&mut self, bn: &mut Bn64) -> Bn64 {
        self.shrink();
        bn.shrink();
        let length = self._len + bn._len;
        let mut result: Bn64 = Bn64::new(length);
        for index_a in 0..self._len {
            let right_a = self._dat[index_a] & _BITS0X20;
            let left_a = self._dat[index_a] >> _HALF_SIZE;
            for index_b in 0..bn._len {
                let right_b = bn._dat[index_b] & _BITS0X20;
                let left_b = bn._dat[index_b] >> _HALF_SIZE;
                result.add_at(index_a + index_b, right_a * right_b);
                result.add_at(index_a + index_b + 1, left_a * left_b);
                let (tmp1, _) = (left_a * right_b).overflowing_shl(_HALF_SIZE);
                result.add_at(index_a + index_b, tmp1);
                result.add_at(index_a + index_b + 1, (left_a * right_b) >> _HALF_SIZE);
                let (tmp2, _) = (left_b * right_a).overflowing_shl(_HALF_SIZE);
                result.add_at(index_a + index_b, tmp2);
                result.add_at(index_a + index_b + 1, (left_b * right_a) >> _HALF_SIZE);
            }
        }

        result.shrink();
        return result;
    }

    pub fn clone(&mut self) -> Bn64 {
        let mut bn: Bn64 = Bn64::new(self._len);
        for index in 0..self._len {
            bn.add_at(index, self._dat[index]);
        }
        bn._tag = self._tag;
        return bn;
    }
}
/* a % m;*/
pub fn mode(a: &mut Bn64, m: &mut Bn64) -> Bn64 {
    let mut result = a.clone();
    let bits_of_m = m.bits() as i32;
    loop {
        result.shrink();
        if result.cmp(m) < 0 {
            return result;
        }
        let mut diff: i32 = result.bits() as i32;
        diff = diff - bits_of_m;
        if diff == 0 {
            return result.sub(m);
        }
        let mut nx = m.left_push(diff as usize);
        if result.cmp(&mut nx) >= 0 {
            result = result.sub(&mut nx);
        } else {
            let mut nx_1 = m.left_push(diff as usize - 1);
            result = result.sub(&mut nx_1);
        }
    }
}

/* a^b % c*/
/*
pub fn npmod(a: &mut Bn64, b: &mut Bn64, c: &mut Bn64) -> Bn64 {
    let bits = b.bits();
    let mut array: Vec<Bn64> = Vec::with_capacity(bits);
    let m = mode(a, c);
    array.push(m);
    for index in 0..bits {
        let mut current: Bn64 = array[index].clone();
        let mut current_copy: Bn64 = current.clone();
        let mut v: Bn64 = current.mul(&mut current_copy);
        let re = mode(&mut v, c);
        array.push(re);
    }
    let mut result = Bn64::new(1);
    result.add_at(0, 1);
    let mut result = Box::new(result);
    for index in 0..bits {
        if b.bit(index) {
            let mut re = result.mul(&mut array[index]);
            re = mode(&mut re, c);
            result = Box::new(re);
        }
    }
    return *result;
}


pub fn npmod2(a: &mut Bn64, b: &mut Bn64, c: &mut Bn64) -> Bn64 {
    let bits = b.bits();
    let mut tmp = mode(a, &mut c.clone());
    let (tx, rx) = channel();

    let mut total_tags: usize = 0;
    for index in 0..bits {
        if b.bit(index) {
            tmp._tag = index;
            tx.send(tmp.clone()).unwrap();
            total_tags += index;
        }
        let mut copy0 = tmp.clone();
        tmp = tmp.mul(&mut copy0);
        tmp = mode(&mut tmp, c);
    }

    loop {
        let mut v0 = rx.recv().unwrap();
        if v0._tag == total_tags {
            /* the aggregation is done; */
            return v0;
        }
        let mut v1 = rx.recv().unwrap();
        let mut c_copy = c.clone();
        let sender = tx.clone();
        thread::spawn(move || {
            let mut r0 = v0.mul(&mut v1);
            r0 = mode(&mut r0, &mut c_copy);
            r0._tag = v0._tag + v1._tag;
            sender.send(r0).unwrap();
        });
    }
}
*/
pub fn npmod3(a: &mut Bn64, b: &mut Bn64, c: &mut Bn64) -> Bn64 {
    let bits = b.bits();
    let mut tmp = mode(a, &mut c.clone());
    let (tx, rx) = channel();
    let arc = Arc::new(Semaphore::new(0));
    let mut b_copy = b.clone();
    let mut c_copy = c.clone();
    let tx_copy = tx.clone();
    let arc_copy = arc.clone();
    thread::spawn(move || {
        let mut total_tags: usize = 0;
        for index in 0..bits {
            if b_copy.bit(index) {
                tmp._tag = index + 1;
                tx_copy.send(tmp.clone()).unwrap();
                total_tags += index + 1;
            }
            let mut copy0 = tmp.clone();
            tmp = tmp.mul(&mut copy0);
            tmp = mode(&mut tmp, &mut c_copy);
        }
        arc_copy.add_permits(total_tags);
    });

    loop {
        let mut v0 = rx.recv().unwrap();
        let available_permits = arc.available_permits();
        if available_permits > 0 && v0._tag == available_permits {
            return v0;
        }
        let mut v1 = rx.recv().unwrap();
        let mut c_copy = c.clone();
        let sender = tx.clone();
        thread::spawn(move || {
            let mut r0 = v0.mul(&mut v1);
            r0 = mode(&mut r0, &mut c_copy);
            r0._tag = v0._tag + v1._tag;
            sender.send(r0).unwrap();
        });
    }
}

pub fn mersenne(n: usize) -> Bn64 {
    let len = n / _SIZE + 1;
    let pos = n % _SIZE;
    let mut result = Bn64::new(len);
    result.add_at(len - 1, 0x1 << pos);
    result.sub_at(0, 1);
    return result;
}
