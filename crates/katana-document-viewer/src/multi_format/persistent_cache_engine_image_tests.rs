use super::{image_uuid, uuid_from_commands};

#[test]
fn linked_image_uuid_is_parsed_after_other_commands_and_invalid_metadata_is_explicit() {
    let mut commands = [0_u8; 32];
    commands[4..8].copy_from_slice(&8_u32.to_le_bytes());
    commands[8..12].copy_from_slice(&0x1b_u32.to_le_bytes());
    commands[12..16].copy_from_slice(&24_u32.to_le_bytes());
    commands[16..].fill(7);
    assert_eq!(Some([7; 16]), uuid_from_commands(&commands).ok());
    commands[16..].fill(0);
    assert!(uuid_from_commands(&commands).is_err());
    commands[16..].fill(7);
    assert!(uuid_from_commands(&commands[..7]).is_err());
    assert!(uuid_from_commands(&commands[..8]).is_err());
    commands[4..8].fill(0);
    assert!(uuid_from_commands(&commands).is_err());
    commands[4..8].copy_from_slice(&33_u32.to_le_bytes());
    assert!(uuid_from_commands(&commands).is_err());
    commands[..4].copy_from_slice(&0x1b_u32.to_le_bytes());
    commands[4..8].copy_from_slice(&8_u32.to_le_bytes());
    assert!(uuid_from_commands(&commands).is_err());
    assert!(image_uuid(std::ptr::null()).is_err());
}
