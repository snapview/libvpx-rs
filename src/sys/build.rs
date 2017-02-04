
extern crate pkg_config as pkg;

pub fn main() {
    pkg::probe_library("vpx").unwrap();
}
