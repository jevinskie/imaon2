use VMA;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RelocKind {
    Pointer,
    _32Bit,
    Arm64Adrp,
    Arm64Off12,
    Arm64Br26,
}
use RelocKind::*;

#[derive(Copy, Clone)]
pub struct RelocContext {
    pub kind: RelocKind,
    pub pointer_size: u64,
    pub base_addr: VMA
}

macro_rules! try_opt { ($x:expr) => {
    if let Some(x) = $x { x } else { return None }
} }

fn sign_extend(val: u64, bits: u8) -> u64 {
    val | (0u64.wrapping_sub((val >> (bits - 1)) & 1) << bits)
}
fn un_sign_extend(val: u64, bits: u8) -> Option<u64> {
    let masked = val & ((1 << bits) - 1);
    if sign_extend(masked, bits) == val { Some(masked) } else { None }
}

impl RelocContext {
    pub fn size(&self) -> u64 {
        match self.kind {
            Pointer => self.pointer_size,
            _32Bit | Arm64Adrp | Arm64Off12 | Arm64Br26 => 4,
        }
    }
    pub fn word_to_addr(&self, word: u64) -> Option<VMA> {
        match self.kind {
            Pointer | _32Bit => Some(VMA(word)),
            Arm64Adrp => {
                if word & 0x9f000000 == 0x90000000 {
                    Some(self.base_addr.wrapping_add(
                        sign_extend((word & 0x60000000) >> 17 | (word & 0xffffe0) << 9, 33)
                    ))
                } else { None }
            },
            Arm64Br26 => {
                if word & 0x7c000000 == 0x14000000 {
                    Some(self.base_addr.wrapping_add(
                        sign_extend((word & 0x3ffffff) * 4, 28)
                    ))
                } else { None }
            },
            Arm64Off12 => {
                unimplemented!()
            },
        }
    }
    pub fn addr_to_word(&self, VMA(addr): VMA, old_word: u64) -> Option<u64> {
        let rel = addr.wrapping_sub(self.base_addr.0);
        match self.kind {
            Pointer => Some(addr),
            _32Bit => if addr <= 0xffffffff { Some(addr) } else { None },
            Arm64Adrp => {
                let base = old_word & !0x600fffe0;
                let x = try_opt!(un_sign_extend(rel, 33));
                if x & 0xfff != 0 { return None; }
                Some((x & 0x3000) << 17 | (x & 0x1ffffc000) >> 9 | base)
            },
            Arm64Br26 => {
                let base = old_word & !0x3ffffff;
                if rel & 3 != 0 { return None; }
                let x = try_opt!(un_sign_extend(rel, 28));
                Some(x >> 2 | base)
            },
            Arm64Off12 => {
                unimplemented!()
            },
        }
    }
}
