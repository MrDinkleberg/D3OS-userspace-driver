
#![no_std]

use spin::lock_api::Mutex;




const CTRL_PORT: u8 = 0x43;
const DATA_PORT: u8 = 0x42;
const PPI_PORT: u8 = 0x61;


//hardcoded from pit.rs
const PIT_BASE_FREQUENCY: usize = 1193182;
pub struct Speaker {
    registers: Mutex<Registers>
}

struct Registers {
    ctrl_port: u8,
    data_port_2: u8,
    ppi_port: u8,
}

impl Registers {
    pub const fn new() -> Self {
        Self {
            ctrl_port: CTRL_PORT,
            data_port_2: DATA_PORT,
            ppi_port: PPI_PORT,
        }
    }
}

impl Speaker {
    pub const fn new() -> Self {
        Self { registers: Mutex::new(Registers::new()) }
    }

    pub fn on(&self, freq: usize) {
        let mut registers = self.registers.lock();
        let counter = PIT_BASE_FREQUENCY / freq;


        // Config counter
        /*registers.ctrl_port.write(0xb6);
        registers.data_port_2.write((counter % 256) as u8);
        registers.data_port_2.write((counter / 256) as u8);*/
        ports::write(CTRL_PORT as usize, 0xb6);
        ports::write(DATA_PORT as usize, (counter % 256));
        ports::write(DATA_PORT as usize, (counter / 256));





        /*// Turn speaker on
        let status = registers.ppi_port.read();
        registers.ppi_port.write(status | 0x03);*/
        let status = ports::read(PPI_PORT as usize);
        ports::write(PPI_PORT as usize, (status | 0x03) );

        // match status {
        //     Ok(status) => {
        //         ports::write(PPI_PORT as usize, (status | 0x03) );
        //     }
        //     Err(_) => core::panic!("SPEAKER ON ERROR")
        // }

    }

    pub fn off(&self) {
        let mut registers = self.registers.lock();

        /*unsafe {
            let status = registers.ppi_port.read();
            registers.ppi_port.write(status & 0xfc);
        }*/
        let status = ports::read(PPI_PORT as usize);
        ports::write(PPI_PORT as usize, (status & 0xfc) );

        // match status {
        //     Ok(status) => {
        //         ports::write(PPI_PORT as usize, (status & 0xfc) );
        //     }
        //     Err(_) => core::panic!("SPEAKER OFF ERROR")
        // }
    }

    pub fn play(&self, freq: usize, duration_ms: usize) {


        self.on(freq);
        //TODO Timer implementieren
        //timer.wait(duration_ms);
        //self.off();
    }
}