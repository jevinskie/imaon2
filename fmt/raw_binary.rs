extern crate exec;
extern crate util;
use exec::arch;

pub struct RawBinary {
    eb: exec::ExecBase,
}

impl exec::Exec for RawBinary {
    fn get_exec_base<'a>(&'a self) -> &'a exec::ExecBase {
        &self.eb
    }

    fn as_any(&self) -> &std::any::Any { self as &std::any::Any }
}

impl RawBinary {
    pub fn new(buf: util::MCRef, _args: Vec<String>) -> RawBinary {
        let len = buf.get().len();
        // todo: parse args
        let seg = exec::Segment {
            vmaddr: exec::VMA(0),
            vmsize: len as u64,
            fileoff: 0u64,
            filesize: len as u64,
            name: None,
            prot: exec::prot_all,
            data: Some(buf.clone()),
            seg_idx: None,
            private: 0,
        };
        RawBinary {
            eb: exec::ExecBase {
                arch: arch::UnknownArch,
                pointer_size: 0,
                endian: util::BigEndian,
                segments: vec!(seg.clone()),
                sections: vec!(seg.clone()),
                whole_buf: Some(buf),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct RawProber;

impl exec::ExecProber for RawProber {
    fn name(&self) -> &str {
        "raw"
    }
    fn probe(&self, _eps: &Vec<&'static exec::ExecProber>, _: util::MCRef) -> Vec<exec::ProbeResult> {
        vec!(exec::ProbeResult {
            desc: "raw".to_string(),
            arch: arch::UnknownArch,
            likely: false,
            cmd: vec!("raw".to_string()),
        })
    }
    fn create(&self, _eps: &Vec<&'static exec::ExecProber>, buf: util::MCRef, args: Vec<String>) -> exec::ExecResult<(Box<exec::Exec>, Vec<String>)> {
        let m = try!(exec::usage_to_invalid_args(util::do_getopts_or_usage(&*args, "raw ...", 0, std::usize::MAX, &mut vec!(
            // ...
        ))));
        Ok((Box::new(RawBinary::new(buf, args)) as Box<exec::Exec>, m.free))
    }
}

