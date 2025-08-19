use crate::{
    formats::lrb::{ModHandler, mod_flags},
    track::Vec2,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use once_cell::sync::Lazy;

// X: f64 = the X coordinate of the start offset
// Y: f64 = the Y coordinate of the start offset (remember +Y is down)

pub(in crate::formats::lrb) static STARTOFFSET: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA | mod_flags::PHYSICS,
    read: Box::new(|cursor, track_builder| {
        let x = cursor.read_f64::<LittleEndian>()?;
        let y = cursor.read_f64::<LittleEndian>()?;
        track_builder.metadata().start_position(Vec2::new(x, y));

        Ok(())
    }),
    write: Box::new(|cursor, track| {
        let start_position = track
            .metadata()
            .start_position()
            .unwrap_or(Vec2::new(0.0, 0.0));
        cursor.write_f64::<LittleEndian>(start_position.x())?;
        cursor.write_f64::<LittleEndian>(start_position.y())?;
        Ok(())
    }),
});
