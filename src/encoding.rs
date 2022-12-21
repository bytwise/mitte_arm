#[derive(Clone, Copy)]
pub struct RegisterIndex {
    pub index: u32,
}

impl From<RegisterIndex> for u32 {
    #[inline]
    fn from(index: RegisterIndex) -> u32 {
        index.index
    }
}


macro_rules! encode {
    ($($e:expr),*) => {
        {
            let mut bits = crate::encoding::BitBuffer::new();
            $(
            bits.push($e);
            )*
            bits.bits
        }
    };
}

pub(crate) use encode;


pub struct BitBuffer {
    pub bits: u32,
    pub shift: u8,
}

impl BitBuffer {
    #[inline]
    pub fn new() -> BitBuffer {
        BitBuffer { bits: 0, shift: 32, }
    }

    #[inline]
    pub fn push<const N: u8>(&mut self, bits: Bits<N>) {
        self.shift -= N;
        self.bits |= bits.0 << self.shift;
    }
}


pub struct Bits<const N: u8>(u32);

macro_rules! define_bits {
    ($($f:ident: $n:literal),*) => {
        $(
            #[inline]
            pub fn $f(x: u32) -> Bits<$n> {
                Bits(x & ((1 << $n) - 1))
            }
        )*
    };
}

define_bits! {
    i1: 1, i2: 2, i3: 3, i4: 4, i5: 5, i6: 6, i7: 7, i8: 8,
    i9: 9, i10: 10, i11: 11, i12: 12, i14: 14, i15: 15, i16: 16,
    i19: 19, i24: 24, i26: 26
}
