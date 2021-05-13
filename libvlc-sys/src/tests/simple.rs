#[cfg(test)]
use create::*;
use std::mem;

#[test]
fn simple_test() {
    unsafe {
        let inst = libvlc_new(0, 0);
        libvlc_release(inst);
    }
}
