#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ModMask(u16);
impl ModMask {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CAPS_LOCK: Self = Self(1 << 1);
    pub const CONTROL: Self = Self(1 << 2);
    pub const MOD1: Self = Self(1 << 3); // alt
    pub const MOD2: Self = Self(1 << 4); // num lock
    pub const MOD3: Self = Self(1 << 5); // not exist anymore (AltGr?)
    pub const MOD4: Self = Self(1 << 6); // super/win/cmd
    pub const MOD5: Self = Self(1 << 7); // scroll lock
    pub const ANY: Self = Self(1 << 15);
}

impl From<ModMask> for u16 {
    #[inline]
    fn from(input: ModMask) -> Self {
        input.0
    }
}
impl From<ModMask> for Option<u16> {
    #[inline]
    fn from(input: ModMask) -> Self {
        Some(input.0)
    }
}
impl From<ModMask> for u32 {
    #[inline]
    fn from(input: ModMask) -> Self {
        u32::from(input.0)
    }
}
impl From<ModMask> for Option<u32> {
    #[inline]
    fn from(input: ModMask) -> Self {
        Some(u32::from(input.0))
    }
}
impl From<u8> for ModMask {
    #[inline]
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}
impl From<u16> for ModMask {
    #[inline]
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl std::ops::BitOr for ModMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for ModMask {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Self(self.0 | rhs.0);
    }
}

impl std::ops::BitAnd for ModMask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
