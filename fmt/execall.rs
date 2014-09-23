extern crate macho;
extern crate elf;
extern crate raw_binary;
extern crate exec;
use self::exec::ExecProber;
use std::mem;

pub fn all_probers() -> Vec<&'static ExecProber+'static> {
    // unsafe due to https://github.com/mozilla/rust/issues/13887
    unsafe {
        return vec!(
            mem::transmute(&self::macho::MachOProber    as &ExecProber),
            mem::transmute(&self::macho::FatMachOProber as &ExecProber),
            mem::transmute(&self::raw_binary::RawProber as &ExecProber),
        );
    }
}
