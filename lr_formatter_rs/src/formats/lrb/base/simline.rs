use crate::{
    formats::lrb::{ModHandler, mod_flags},
    track::Vec2,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// count: u32 = the amount of lines written
// lines: simline[count] = [
//   id: u32 = the line's ID
//   flags: u8 = Line flags 0000DCBA
//   x1: f64 = the x position of the 1st point
//   y1: f64 = the y position of the 1st point
//   x2: f64 = the x position of the 2nd point
//   y2: f64 = the y position of the 2nd point
// ]
// Line flag defs: A = Red line, B = inverted, C = left extension, D = right extension

pub const FLAG_RED: u8 = 1 << 0;
pub const FLAG_INVERTED: u8 = 1 << 1;
pub const FLAG_LEFT_EXTENSION: u8 = 1 << 2;
pub const FLAG_RIGHT_EXTENSION: u8 = 1 << 3;

pub(in crate::formats::lrb) static SIMLINE: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA | mod_flags::PHYSICS | mod_flags::SCENERY,
    read: Box::new(|cursor, track_builder| {
        let num_lines = cursor.read_u32::<LittleEndian>()?;
        for _ in 0..num_lines {
            let id = cursor.read_u32::<LittleEndian>()?;
            let line_flags = cursor.read_u8()?;
            let x1 = cursor.read_f64::<LittleEndian>()?;
            let y1 = cursor.read_f64::<LittleEndian>()?;
            let x2 = cursor.read_f64::<LittleEndian>()?;
            let y2 = cursor.read_f64::<LittleEndian>()?;
            let flipped = line_flags & FLAG_INVERTED != 0;
            let left_extension = line_flags & FLAG_LEFT_EXTENSION != 0;
            let right_extension = line_flags & FLAG_RIGHT_EXTENSION != 0;
            let endpoints = (Vec2::new(x1, y1), Vec2::new(x2, y2));
            if line_flags & FLAG_RED != 0 {
                track_builder.line_group().add_acceleration_line(
                    id,
                    endpoints,
                    flipped,
                    left_extension,
                    right_extension,
                )?;
            } else {
                track_builder.line_group().add_standard_line(
                    id,
                    endpoints,
                    flipped,
                    left_extension,
                    right_extension,
                )?;
            }
        }

        Ok(())
    }),
    write: Box::new(|cursor, track| {
        cursor.write_u32::<LittleEndian>(
            (track.line_group().acceleration_lines().len()
                + track.line_group().standard_lines().len()) as u32,
        )?;

        for line in track.line_group().standard_lines() {
            let mut line_flags: u8 = 0;
            if line.flipped() {
                line_flags |= FLAG_INVERTED;
            }
            if line.left_extension() {
                line_flags |= FLAG_LEFT_EXTENSION;
            }
            if line.right_extension() {
                line_flags |= FLAG_RIGHT_EXTENSION;
            }

            cursor.write_u32::<LittleEndian>(line.id())?;
            cursor.write_u8(line_flags)?;
            cursor.write_f64::<LittleEndian>(line.x1())?;
            cursor.write_f64::<LittleEndian>(line.y1())?;
            cursor.write_f64::<LittleEndian>(line.x2())?;
            cursor.write_f64::<LittleEndian>(line.y2())?;
        }

        for line in track.line_group().acceleration_lines() {
            let mut line_flags: u8 = 0;
            line_flags |= FLAG_RED;
            if line.flipped() {
                line_flags |= FLAG_INVERTED;
            }
            if line.left_extension() {
                line_flags |= FLAG_LEFT_EXTENSION;
            }
            if line.right_extension() {
                line_flags |= FLAG_RIGHT_EXTENSION;
            }

            cursor.write_u32::<LittleEndian>(line.id())?;
            cursor.write_u8(line_flags)?;
            cursor.write_f64::<LittleEndian>(line.x1())?;
            cursor.write_f64::<LittleEndian>(line.y1())?;
            cursor.write_f64::<LittleEndian>(line.x2())?;
            cursor.write_f64::<LittleEndian>(line.y2())?;
        }

        Ok(())
    }),
});
