macro_rules! functions {
    (
        $(
            $f:ident($($args:tt)*) => $encoding:ident $({ $($fields:tt)* })*;
        )*
    ) => {
        $(
            functions! {
                arg @ $f($($args)*) => $encoding $({ $($fields)* })*;
            }
        )*
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm9: i16) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty,)* imm9: i16) => $encoding {
                $($arg : $arg . into(),)*
                imm9: imm9 as u32 & 0x1ff,
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; extend: Extend) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty,)* extend: Extend) => $encoding {
                $($arg : $arg . into(),)*
                option: extend.ty().into(),
                imm3: extend.amount().into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; shift: Shift<u8>) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty,)* shift: Shift<u8>) => $encoding {
                $($arg : $arg . into(),)*
                shift: shift.ty().into(),
                imm6: shift.amount().into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm: impl Into<ShiftedImm12>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f<I>($($arg : $arg_ty,)* imm: I)
                where I: Into<ShiftedImm12>
            ; let imm = imm.into() => $encoding {
                $($arg : $arg . into(),)*
                sh: imm.sh.into(),
                imm12: imm.imm12.into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm: impl Into<MovImm32>) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f<I>($($arg : $arg_ty,)* imm: I)
                where I: Into<MovImm32>
            ; let imm = imm.into() => $encoding {
                $($arg : $arg . into(),)*
                hw: imm.hw.into(),
                imm16: imm.imm16.into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm: impl Into<MovImm64>) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f<I>($($arg : $arg_ty,)* imm: I)
                where I: Into<MovImm64>
            ; let imm = imm.into() => $encoding {
                $($arg : $arg . into(),)*
                hw: imm.hw.into(),
                imm16: imm.imm16.into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm: Bitmask32) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty,)* imm: Bitmask32) => $encoding {
                $($arg : $arg . into(),)*
                immr: imm.immr.into(),
                imms: imm.imms.into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),* ; imm: Bitmask64) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty,)* imm: Bitmask64) => $encoding {
                $($arg : $arg . into(),)*
                n: imm.n.into(),
                immr: imm.immr.into(),
                imms: imm.imms.into(),
            }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),*) => $encoding:ident { $($fields:tt)* };
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty),*) => $encoding { $($fields)* }
        }
    };

    (
        arg @ $f:ident($($arg:ident : $arg_ty:ty),*) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f($($arg : $arg_ty),*) => $encoding { $($arg : $arg . into()),* }
        }
    };

    (
        fn @ $f:ident $(<$($generics:ident)*>)* ($($arg:ident : $arg_ty:ty),*)
            $(where $($T:ident : $bound:path),+)*
            $(; $init:stmt)* => $encoding:ident { $($fields:tt)* }
    ) => {
        #[inline]
        pub fn $f $(<$($generics)*>)* ($($arg : $arg_ty),*) -> u32
            $(where $($T : $bound),+)*
        {
            #![allow(redundant_semicolons)]
            $($init;)*
            encoding::$encoding { $($fields)* }.encode()
        }
    };
}

pub(crate) use functions;
