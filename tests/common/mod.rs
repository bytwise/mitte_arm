use mitte_arm::types::Condition;
use mitte_arm::types::Shift;


pub trait TestCases: Sized + Copy + 'static {
    fn test_cases() -> Vec<(Self, String)>;
}

impl TestCases for Condition {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Condition::Eq, "eq".into()),
            (Condition::Ne, "ne".into()),
            (Condition::Cc, "lo".into()),
            (Condition::Cs, "hs".into()),
            (Condition::Mi, "mi".into()),
            (Condition::Pl, "pl".into()),
            (Condition::Vs, "vs".into()),
            (Condition::Vc, "vc".into()),
            (Condition::Hi, "hi".into()),
            (Condition::Ls, "ls".into()),
            (Condition::Ge, "ge".into()),
            (Condition::Lt, "lt".into()),
            (Condition::Gt, "gt".into()),
            (Condition::Le, "le".into()),
        ]
    }
}

impl TestCases for u8 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "#0".into()),
            (1, "#1".into()),
            (2, "#2".into()),
            (3, "#3".into()),
            (4, "#4".into()),
            (7, "#7".into()),
            (8, "#8".into()),
            (0xf, "#0xf".into()),
            (0x10, "#0x10".into()),
            (0x1f, "#0x1f".into()),
            (0x20, "#0x20".into()),
            (0x3f, "#0x3f".into()),
            (0x40, "#0x40".into()),
            (0x7f, "#0x7f".into()),
            (0x80, "#0x80".into()),
            (0xff, "#0xff".into()),
        ]
    }
}

impl TestCases for u16 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "#0".into()),
            (1, "#1".into()),
            (2, "#2".into()),
            (3, "#3".into()),
            (4, "#4".into()),
            (7, "#7".into()),
            (8, "#8".into()),
            (0xf, "#0xf".into()),
            (0x10, "#0x10".into()),
            (0x1f, "#0x1f".into()),
            (0x20, "#0x20".into()),
            (0x3f, "#0x3f".into()),
            (0x40, "#0x40".into()),
            (0x7f, "#0x7f".into()),
            (0x80, "#0x80".into()),
            (0xff, "#0xff".into()),
            (0x100, "#0x100".into()),
            (0x1ff, "#0x1ff".into()),
            (0x200, "#0x200".into()),
            (0x3ff, "#0x3ff".into()),
            (0x400, "#0x400".into()),
            (0x7ff, "#0x7ff".into()),
            (0x800, "#0x800".into()),
            (0xfff, "#0xfff".into()),
            (0x1000, "#0x1000".into()),
            (0x1fff, "#0x1fff".into()),
            (0x2000, "#0x2000".into()),
            (0x3fff, "#0x3fff".into()),
            (0x4000, "#0x4000".into()),
            (0x7fff, "#0x7fff".into()),
            (0x8000, "#0x8000".into()),
            (0xffff, "#0xffff".into()),
        ]
    }
}

impl TestCases for i16 {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (0, "#0".into()),
            (0x7f, "#0x7f".into()),
            (0xff, "#0xff".into()),
            (0x1ff, "#0x1ff".into()),
            (0x3ff, "#0x3ff".into()),
            (0x7ff, "#0x7ff".into()),
            (-0x80, "#-0x80".into()),
            (-0x100, "#-0x100".into()),
            (-0x200, "#-0x200".into()),
            (-0x400, "#-0x400".into()),
            (-0x800,"#-0x800".into()),
        ]
    }
}

impl TestCases for Shift<u8> {
    fn test_cases() -> Vec<(Self, String)> {
        vec![
            (Shift::Lsl(7), "lsl #7".into()),
            (Shift::Lsl(31), "lsl #31".into()),
            (Shift::Lsr(7), "lsr #7".into()),
            (Shift::Lsr(31), "lsr #31".into()),
            (Shift::Asr(7), "asr #7".into()),
            (Shift::Asr(31), "asr #31".into()),
            (Shift::Ror(7), "ror #7".into()),
            (Shift::Ror(31), "ror #31".into()),
        ]
    }
}
