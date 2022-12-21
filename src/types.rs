#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Eq = 0,
    Ne = 1,
    Cs = 2,
    Cc = 3,
    Mi = 4,
    Pl = 5,
    Vs = 6,
    Vc = 7,
    Hi = 8,
    Ls = 9,
    Ge = 10,
    Lt = 11,
    Gt = 12,
    Le = 13,
    Al = 14,
}

#[allow(non_upper_case_globals)]
impl Condition {
    pub const Hs: Condition = Condition::Cs;
    pub const Lo: Condition = Condition::Cc;

    #[inline]
    pub fn invert(&self) -> Option<Condition> {
        match *self {
            Condition::Eq => Some(Condition::Ne),
            Condition::Ne => Some(Condition::Eq),
            Condition::Cs => Some(Condition::Cc),
            Condition::Cc => Some(Condition::Cs),
            Condition::Mi => Some(Condition::Pl),
            Condition::Pl => Some(Condition::Mi),
            Condition::Vs => Some(Condition::Vc),
            Condition::Vc => Some(Condition::Vs),
            Condition::Hi => Some(Condition::Ls),
            Condition::Ls => Some(Condition::Hi),
            Condition::Ge => Some(Condition::Lt),
            Condition::Lt => Some(Condition::Ge),
            Condition::Gt => Some(Condition::Le),
            Condition::Le => Some(Condition::Gt),
            Condition::Al => None,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Label {
    pub(crate) offset: i64,
}

impl Label {
    #[inline]
    pub fn byte_offset(&self) -> i64 {
        self.offset
    }

    #[inline]
    pub fn from_byte_offset(offset: i64) -> Label {
        Label { offset }
    }
}


#[derive(Clone, Copy)]
pub enum Shift<T> {
    Lsl(T),
    Lsr(T),
    Asr(T),
    Ror(T),
}

impl<T: Copy> Shift<T> {
    pub fn ty(&self) -> u8 {
        match *self {
            Shift::Lsl(_) => 0,
            Shift::Lsr(_) => 1,
            Shift::Asr(_) => 2,
            Shift::Ror(_) => 3,
        }
    }

    pub fn amount(&self) -> T {
        match *self {
            Shift::Lsl(amount) |
            Shift::Lsr(amount) |
            Shift::Asr(amount) |
            Shift::Ror(amount)
                => amount,
        }
    }
}
