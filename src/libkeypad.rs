use embedded_hal::digital::{InputPin, OutputPin};

pub struct KEYPAD4X4<R, C>
where
    R: InputPin,
    C: OutputPin,
{
    pins: ((R, R, R, R), (C, C, C, C)), // Tuples for rows and columns
}

impl<R, C> KEYPAD4X4<R, C>
where
    R: InputPin,
    C: OutputPin,
{
    pub fn new(rows: (R, R, R, R), columns: (C, C, C, C)) -> Self {
        KEYPAD4X4 {
            pins: (rows, columns),
        }
    }

    // pub fn get_key(&mut self, v: (u8, u8)) -> char {
    //     const LUT: [[char; 4]; 4] = [
    //         ['1', '2', '3', 'A'],
    //         ['4', '5', '6', 'B'],
    //         ['7', '8', '9', 'C'],
    //         ['*', '0', '#', 'D'],
    //     ];

    //     let (x, y) = (v.0 as usize, v.1 as usize);
    //     LUT[y][x]
    // }

    pub fn read_position(&mut self) -> Option<(u8, u8)> {
        let mut return_value: (u8, u8) = (0, 0);
        let mut count: u8 = 0;
        let mut col_idx: u8 = 0;

        // Destructure columns for iteration
        let columns = &mut self.pins.1;
        // Destructure rows for iteration
        let rows = &mut self.pins.0;
        let mut colarr = [
            &mut columns.0,
            &mut columns.1,
            &mut columns.2,
            &mut columns.3,
        ];

        let mut rowarr = [&mut rows.0, &mut rows.1, &mut rows.2, &mut rows.3];

        for column in colarr.iter_mut() {
            // Set the current column low (active)
            if column.set_low().is_err() {
                continue; // Skip this column if setting it low fails
            }

            let mut row_idx: u8 = 0;

            for row in rowarr.iter_mut() {
                if row.is_low().unwrap_or(false) {
                    count += 1;
                    return_value = (row_idx, col_idx);

                    // Wait for key release
                    while row.is_low().unwrap_or(false) {
                        ()
                    }
                }
                row_idx += 1;
            }

            // Reset the column to high (inactive) after scanning
            column.set_high().ok();
            col_idx += 1;
        }

        if count == 1 {
            return Some(return_value);
        }

        // No key press detected
        None
    }
}
