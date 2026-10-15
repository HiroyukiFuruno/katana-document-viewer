use super::CacheKey;
use std::io;

#[cfg(test)]
#[path = "persistent_cache_engine_image_tests.rs"]
mod tests;

unsafe extern "C" {
    fn _dyld_get_image_header(image_index: u32) -> *const u8;
}

pub(super) fn digest() -> io::Result<String> {
    // dyldが保持する起動imageのUUIDは、pathの置換とASLRの影響を受けない。
    let header = unsafe { _dyld_get_image_header(0) };
    Ok(CacheKey::digest(&image_uuid(header)?))
}

fn invalid_image() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        "running Mach-O image has no valid UUID",
    )
}

fn image_uuid(header: *const u8) -> io::Result<[u8; 16]> {
    // macOSの対応archは64bitで、dyldのheaderとload commandsはprocessの間有効。
    let header = unsafe { header.cast::<[u32; 8]>().as_ref() }.ok_or_else(invalid_image)?;
    let commands = unsafe {
        std::slice::from_raw_parts(
            (header as *const [u32; 8]).add(1).cast::<u8>(),
            header[5] as usize,
        )
    };
    uuid_from_commands(commands)
}

fn uuid_from_commands(mut commands: &[u8]) -> io::Result<[u8; 16]> {
    while commands.len() >= 8 {
        let kind = u32::from_le_bytes([commands[0], commands[1], commands[2], commands[3]]);
        let size =
            u32::from_le_bytes([commands[4], commands[5], commands[6], commands[7]]) as usize;
        if size < 8 || size > commands.len() {
            return Err(invalid_image());
        }
        if kind == 0x1b {
            if size < 24 {
                return Err(invalid_image());
            }
            let mut uuid = [0; 16];
            uuid.copy_from_slice(&commands[8..24]);
            if uuid == [0; 16] {
                return Err(invalid_image());
            }
            return Ok(uuid);
        }
        commands = &commands[size..];
    }
    Err(invalid_image())
}
