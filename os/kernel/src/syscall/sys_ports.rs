use x86_64::instructions::port::{Port, PortWriteOnly};
pub unsafe fn sys_write_port(port_num: u16, value: u8){
    let mut port: PortWriteOnly<u8> = PortWriteOnly::new(port_num);
    port.write(value);

}

pub unsafe fn sys_read_port(port_num: u16) -> isize {
    let mut port: Port<u8> = Port::new(port_num);
    let ret = port.read() as isize;
    ret
}