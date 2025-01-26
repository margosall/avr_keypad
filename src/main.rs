#![no_std]
#![no_main]

use libfont::get_char_bitmap;
// use embedded_hal::digital::OutputPin;
use panic_halt as _; // Minimal panic handler

use arduino_hal::{Peripherals, delay_ms, pins};

//use ssd1306::{mode::DisplayConfig, size::DisplaySize128x32, I2CDisplayInterface, Ssd1306};
pub mod libkeypad;
use libkeypad::KEYPAD4X4;
use myssd1306::{DisplaySize128x32, Ssd1306};

type CalcType = u8;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = pins!(dp);
    let mut keypad = KEYPAD4X4::new(
        (
            pins.d9.into_pull_up_input().downgrade(),
            pins.d8.into_pull_up_input().downgrade(),
            pins.d7.into_pull_up_input().downgrade(),
            pins.d6.into_pull_up_input().downgrade(),
        ),
        (
            pins.d5.into_output().downgrade(),
            pins.d4.into_output().downgrade(),
            pins.d3.into_output().downgrade(),
            pins.d2.into_output().downgrade(),
        ),
    );
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        400_000,
    );
    let mut disp: Ssd1306<_, _> = Ssd1306::new(i2c, DisplaySize128x32);
    disp.clear();
    delay_ms(50);

    let mut stack: [u8; 10] = [0; 10];
    let mut op1: CalcType = 0;
    let mut op2: CalcType = 0;
    loop {
        if let Some(x) = keypad.read_position() {
            //stack[p] = z;
            let con_numeric = (x.0 < 3) && (x.1 < 3);
            if con_numeric || ((x.1 == 3) && (x.0 == 1)) {
                op1 *= 10;
                if con_numeric {
                    op1 += (1 + x.1 * 3 + x.0) as CalcType;
                }
                //else is zero, do nothing here
            } else {
                match x {
                    //#
                    (2, 3) => {
                        op2 = op1;
                        op1 = 0;
                    }
                    //A
                    (3, 0) => {
                        op2 += op1;
                        op1 = op2;
                    }
                    (3, 1) => {
                        op2 -= op1;
                        op1 = op2;
                    }
                    (3, 2) => {
                        op2 = 0;
                        op1 = 0;
                    }

                    _ => (),
                }
                disp.clear();
            }
            // p += 1;
        }
        let mut opout = op1;
        let mut ii: u8 = 0;
        disp.set_row(2);

        if opout != 0 {
            while opout > 0 {
                let cp = ((opout % 10) + 0x30) as u8;
                opout /= 10;
                stack[ii as usize] = cp;
                ii += 1;
            }
            for i in (0..ii).rev() {
                let c = stack[i as usize] as char;
                disp.draw(&get_char_bitmap(&c));
                disp.draw(&[0]);
            }
        } else {
            disp.draw(&get_char_bitmap(&'0'));
        }
    }
    // disp.set_row(0).unwrap();

    // disp.set_column(0).unwrap();
    // disp.draw(&e).unwrap();
    // disp.set_column(0).unwrap();

    // disp.set_row(8).unwrap();

    // if pp == p {
    //     delay_ms(5);
    // } else {
    //     delay_ms(50);
    //     pp = p;
    // }

    /*const QUOTES: &[&str] = &[
            "Software comes from heaven when you have good hardware.",
            "There is always one more bug to fix.",
            "Talk is cheap. Show me the code.",
            "If, at first, you do not succeed, call it version 1.0.",
            "Computers are fast; developers keep them slow.",
        ];
        let mut chbit = [0; 5];

        for q in QUOTES.iter() {
            let mut ic: u8 = 0;
            let mut r: u8 = 0;
            const ROWC: u8 = 21;
            let mut rowbuf = [0_u8; (ROWC * 6) as usize];
            let arr = q.chars();
            for ch in arr {
                get_char_bitmap(&ch, &mut chbit);
                let s: u8 = ic * 6;
                for i in 0_u8..5_u8 {
                    rowbuf[(s + i) as usize] = chbit[i as usize]
                }
                ic += 1;

                if ic == ROWC {
                    if r == 0 {
                        disp.clear().unwrap();
                    }
                    let _ = disp.set_column(0);
                    let _ = disp.set_row(r);

                    let _ = disp.draw(&rowbuf);

                    r += 8;
                    ic = 0;
                }
            }
            let _ = disp.set_column(0);
            let _ = disp.set_row(r);

            for i in 0..(ic as usize * 6) {
                let _ = disp.draw(&[rowbuf[i]]);
            }
            disp.draw(&a);
            disp.draw(&[0]);

            disp.draw(&b);

            delay_ms(300);
        }
    }*/
}
