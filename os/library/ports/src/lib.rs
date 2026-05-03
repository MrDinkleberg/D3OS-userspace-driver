#![no_std]

use syscall::{syscall, SystemCall};

// TODO Error Handling

pub fn write(port_nr: usize, val: usize) {
    match syscall(SystemCall::WritePort, &[port_nr, val]) {
        Ok(_) => {}
        _ => {}
    }

    ;
}

pub fn read(port_nr: usize,) -> usize {
    let res = syscall(SystemCall::ReadPort, &[port_nr]);
    match res {
        Ok(..) => {res.unwrap()}
        _ => { 0 }
    }
}