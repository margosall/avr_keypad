#![no_std]
use embedded_hal::i2c::I2c;
const CMD: u8 = 0x00;
const DAT: u8 = 0x40;

#[derive(Copy, Clone, Debug)]
pub struct Ssd1306<DI, SIZE> {
    interface: DI,
    size: SIZE,
}
pub trait DisplaySize {
    const WIDTH: u8 = 128;

    /// Height in pixels
    const HEIGHT: u8 = 32;
}
pub struct DisplaySize128x32;
impl DisplaySize for DisplaySize128x32 {
    const WIDTH: u8 = 128;
    const HEIGHT: u8 = 32;
}

#[allow(dead_code)]

pub enum AddrMode {
    /// Horizontal mode
    Horizontal = 0b00,
    /// Vertical mode
    Vertical = 0b01,
    /// Page mode (default)
    Page = 0b10,
}
const ADDR: u8 = 0x3C;
const HEIGHT: u8 = 32;
const WIDTH: u8 = 128;
impl<DI: I2c, SIZE> Ssd1306<DI, SIZE>
where
    SIZE: DisplaySize,
{
    pub fn new(interface: DI, size: SIZE) -> Self {
        let mut i = Self { interface, size };
        i.init();
        // i.set_draw_area((0, 0), (128, 32));
        i
    }
    pub fn init(self: &mut Self) -> bool {
        // let (phase1, phase2) = (1, 2); // 2 is brightness normal
        // let contrast = 0x5F;
        // let (alt, lr) = (false, false);
        let initsetup: &[&[u8]] = &[
            &[CMD, 0xAE],         /*display off*/
            &[CMD, 0xD5, 0x80],   /*clockdiv */
            &[CMD, 0xA8, 32 - 1], /*multiplex */
            &[CMD, 0xD3, 0],      /*offset*/
            &[CMD, 0x40],         /*startline */
            &[CMD, 0x8D, 0x14],   /*chargepump */
            &[CMD, 0x20, 0],      /*address horisontal */
            //compin
            &[CMD, 0xDA, 0x2], /*COMPIN */
            //rotation
            &[CMD, 0xA1],             /*remap true for rotate 0 */
            &[CMD, 0xC8],             /*reversecomdir for rotate0 */
            &[CMD, 0xD9, 0x12],       /*precharge */
            &[CMD, 0x81, 0x5F],       /*contrast */
            &[CMD, 0xDB, 0b100 << 4], /*vcomdeselect auto */
            &[CMD, 0xAF],             /*display on */
            &[CMD, 0x21, 0, WIDTH.saturating_sub(1)],
            &[CMD, 0x22, 0, HEIGHT.saturating_sub(1)],
        ];
        for setup in initsetup {
            // self.interface.write(ADDR, setup).unwrap();
            self.write_cmd(setup);
        }
        true
    }
    // pub fn set_draw_area(&mut self, start: (u8, u8), end: (u8, u8)) -> bool {
    //     let _ = self
    //         .interface
    //         .write(self.addr, &[CMD, 0x21, 0, 128_u8.saturating_sub(1)]);
    //     let _ = self
    //         .interface
    //         .write(self.addr, &[CMD, 0x22, 0, 32_u8.saturating_sub(1)]);
    //     true
    // }
    fn write_cmd(&mut self, cmd_data: &[u8]) {
        let _ = self.interface.write(ADDR, cmd_data);
    }
    pub fn set_column(&mut self, addr: u8) -> bool {
        // let _ = self
        //     .interface
        //     .write(ADDR, &[CMD, 0xF & addr, 0x10 | (0xF & (addr >> 4))]);
        self.write_cmd(&[CMD, 0xF & addr, 0x10 | (0xF & (addr >> 4))]);
        true
    }
    pub fn set_row(&mut self, page: u8) -> bool {
        // let _ = self.interface.write(ADDR, &[CMD, 0xB0 | page]);
        self.write_cmd(&[CMD, 0xB0 | page]);
        self.write_cmd(&[CMD, 0, 0x10]); //column 0

        // Command::PageStart(row.into()).send(&mut self.interface)
        true
    }

    pub fn draw(&mut self, data: &[u8]) -> bool {
        let mut buffer = [0u8; 64]; // Adjust size as needed
        let total = data.len() + 1;
        buffer[0] = DAT;
        buffer[1..total].copy_from_slice(data);
        self.write_cmd(&buffer[..total]);

        // let _ = self.interface.write(ADDR); //display on

        // Command::PageStart(row.into()).send(&mut self.interface)
        true
    }
    pub fn clear(&mut self) {
        for row in 0..4 {
            self.set_row(row);
            self.set_column(0);
            for _ in 0..(128_u8 >> 3) {
                self.draw(&[0_u8; 8]);
            }
        }
    }
}
