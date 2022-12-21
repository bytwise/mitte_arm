macro_rules! functions {
    (
        $(
            $f:ident $(# $cond:tt)* $(<$($generics:ident),*>)* ($($args:tt)*)
                $(where $($T:ident : $bound:path),+)*
                $(; $init:stmt)* => $encoding:ident $({ $($fields:tt)* })*;
        )*
    ) => {
        $(
            functions! {
                arg @ $f $(# $cond)* $(<$($generics),*>)* ($($args)*)
                    $(where $($T : $bound),+)*
                    $(; $init)* => $encoding $({ $($fields)* })*;
            }
        )*
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; imm: impl Into<Const>) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* <C>($($arg : $arg_ty,)* imm: C)
                where C: Into<Const>
            => $encoding {
                $($arg : $arg . into(),)*
                imm12: imm.into().to_bits(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; shift: Shift<u8>) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* shift: Shift<u8>) => $encoding {
                $($arg : $arg . into(),)*
                stype: shift.ty().into(),
                imm5: shift.amount().into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; shift: Shift<Register>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* shift: Shift<Register>) => $encoding {
                $($arg : $arg . into(),)*
                stype: shift.ty().into(),
                rs: shift.amount().into()
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; ror: Ror) => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* ror: Ror) => $encoding {
                $($arg : $arg . into(),)*
                rotate: (ror as u8).into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; offset: Offset<u8>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* offset: Offset<u8>) => $encoding {
                $($arg : $arg . into(),)*
                u: offset.is_add().into(),
                imm4h: (offset.abs_offset() >> 4).into(),
                imm4l: (offset.abs_offset() & 0xf).into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; offset12: Offset<u16>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* offset12: Offset<u16>) => $encoding {
                $($arg : $arg . into(),)*
                u: offset12.is_add().into(),
                imm12: offset12.abs_offset().into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; offset: Offset<Register>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty,)* offset: Offset<Register>) => $encoding {
                $($arg : $arg . into(),)*
                u: offset.is_add().into(),
                rm: offset.abs_offset().into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; index: impl Into<Index>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* <I>($($arg : $arg_ty,)* index: I)
                where I: Into<Index>
            ; let index = index.into() => $encoding {
                $($arg : $arg . into(),)*
                u: index.is_add().into(),
                rm: index.index_reg().into(),
                stype: index.shift().ty().into(),
                imm5: index.shift().amount().into(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),* ; regs: impl Into<RegisterList>)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* <L>($($arg : $arg_ty,)* regs: L)
                where L: Into<RegisterList>
            => $encoding {
                $($arg : $arg . into(),)*
                register_list: regs.into().to_bits(),
            }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* $(<$($generics:ident),*>)* ($($arg:ident : $arg_ty:ty),*)
            $(where $($T:ident : $bound:path),+)*
            $(; $init:stmt)* => $encoding:ident { $($fields:tt)* };
    ) => {
        functions! {
            fn @ $f $(# $cond)* $(<$($generics),*>)* ($($arg : $arg_ty),*)
                $(where $($T : $bound),+)*
                $(; $init)* => $encoding { $($fields)* }
        }
    };

    (
        arg @ $f:ident $(# $cond:tt)* ($($arg:ident : $arg_ty:ty),*)
            => $encoding:ident;
    ) => {
        functions! {
            fn @ $f $(# $cond)* ($($arg : $arg_ty),*) => $encoding { $($arg : $arg . into()),* }
        }
    };

    (
        fn @ $f:ident $(<$($generics:ident),*>)* ($($arg:ident : $arg_ty:ty),*)
            $(where $($T:ident : $bound:path),+)*
            $(; $init:stmt)* => $encoding:ident { $($fields:tt)* }
    ) => {
        #[inline]
        pub fn $f $(<$($generics),*>)* ($($arg : $arg_ty),*) -> u32
            $(where $($T : $bound),+)*
        {
            #![allow(redundant_semicolons)]
            $($init;)*
            encoding::$encoding { $($fields)* }.encode()
        }
    };

    (
        fn @ $f:ident # cond $(<$($generics:ident),*>)* ($($arg:ident : $arg_ty:ty),*)
            $(where $($T:ident : $bound:path),+)*
            $(; $init:stmt)* => $encoding:ident { $($fields:tt)* }
    ) => {
        pub mod $f {
            use super::*;

            #[inline]
            pub fn cond $(<$($generics),*>)* (cond: Condition, $($arg : $arg_ty),*) -> u32
                $(where $($T : $bound),+)*
            {
                #![allow(redundant_semicolons)]
                $($init;)*
                encoding::$encoding { cond, $($fields)* }.encode()
            }
        }

        #[inline]
        pub fn $f $(<$($generics),*>)* ($($arg : $arg_ty),*) -> u32
            $(where $($T : $bound),+)*
        {
            $f::cond(Condition::Al, $($arg),*)
        }
    };
}

pub(crate) use functions;
