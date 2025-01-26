use embedded_hal::{digital::OutputPin, spi::SpiBus};

use avr_progmem::raw::read_byte;
use libfont::FONT;

const DECODE_MODE: u8 = 0x09;
const INTENSITY: u8 = 0x0A;
const SCAN_LIMIT: u8 = 0x0B;
const SHUTDOWN: u8 = 0x0C;
const DISPLAY_TEST: u8 = 0x0F;

#[derive(Clone)]
pub struct MAX7219<SPI: SpiBus, CS: OutputPin> {
    spi: SPI,
    cs: CS,
}
pub fn rev_bits(x: u8) -> u8 {
    // Lookup table for reversing 4 bits (nibble)
    const NIBBLE_REVERSE: [u8; 16] = [
        0b0000, // 0  -> 0
        0b1000, // 1  -> 8
        0b0100, // 2  -> 4
        0b1100, // 3  -> 12
        0b0010, // 4  -> 2
        0b1010, // 5  -> 10
        0b0110, // 6  -> 6
        0b1110, // 7  -> 14
        0b0001, // 8  -> 1
        0b1001, // 9  -> 9
        0b0101, // 10 -> 5
        0b1101, // 11 -> 13
        0b0011, // 12 -> 3
        0b1011, // 13 -> 11
        0b0111, // 14 -> 7
        0b1111, // 15 -> 15
    ];

    // Reverse the high and low nibbles using the lookup table
    let low_nibble = x & 0b00001111;
    let high_nibble = (x & 0b11110000) >> 4;

    (NIBBLE_REVERSE[low_nibble as usize] << 4) | NIBBLE_REVERSE[high_nibble as usize]
}
impl<SPI: SpiBus, CS: OutputPin> MAX7219<SPI, CS> {
    pub fn new(spi: SPI, cs: CS) -> Self {
        let mut i = Self { spi, cs };
        i.init().unwrap();
        i.clear().unwrap();
        i
    }

    #[inline]
    pub fn init(&mut self) -> Result<(), SPI::Error> {
        self.send_command(DECODE_MODE, 0)?;
        self.send_command(INTENSITY, 1)?;
        self.send_command(SCAN_LIMIT, 0x07)?;
        self.send_command(SHUTDOWN, 0x01)?;
        self.send_command(DISPLAY_TEST, 0x00)?;
        Ok(())
    }

    #[inline]
    fn send_command(&mut self, addr: u8, data: u8) -> Result<(), SPI::Error> {
        self.cs.set_low().unwrap();
        let r = self.spi.write(&[addr, data]);
        self.cs.set_high().unwrap();

        // let f = self.spi.read(&mut rb);
        r
    }

    #[inline]
    pub fn set_row(&mut self, row: u8, value: u8) -> Result<(), SPI::Error> {
        self.send_command(row + 1, value)
    }

    #[inline]
    pub fn set_pic(&mut self, pic: [u8; 8]) -> Result<(), SPI::Error> {
        let mut i: u8 = 0;
        for v in pic {
            i += 1;
            self.send_command(i, rev_bits(v))?;
        }
        Ok(())
    }

    #[inline]
    pub fn clear(&mut self) -> Result<(), SPI::Error> {
        for i in 0..8 {
            self.set_row(i, 0)?;
        }
        Ok(())
    }

    /*pub fn print_char(&mut self, c: &char) -> Result<(), SPI::Error> {
        if (*c as u8) <= 126 {
            let mut ctemp: [u8; 5] = [0_u8; 5];
            get_char_bitmap(c, &mut ctemp);
            for i in 0..5 {
                ctemp[i] = rev_bits(ctemp[i]);
            }
            return self.set_pic(&ctemp);
        }
        Ok(())
    }*/
}


#[inline]
pub fn set_column(x: u8, v: u8, grid: &mut [u8]) {
    grid[x as usize] = v;
}
#[inline]
pub fn set_pixel(x: u8, y: u8, v: bool, grid: &mut [u8]) {
    if v {
        grid[x as usize] |= 1 << y;
    } else {
        grid[x as usize] &= !(1 << y);
    }
}
