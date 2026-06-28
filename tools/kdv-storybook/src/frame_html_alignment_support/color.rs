pub(super) fn color_distance(left: u32, right: u32) -> u32 {
    channel_distance(left, right, 16)
        + channel_distance(left, right, 8)
        + channel_distance(left, right, 0)
}

fn channel_distance(left: u32, right: u32, shift: u32) -> u32 {
    let left_channel = ((left >> shift) & 0xff) as i32;
    let right_channel = ((right >> shift) & 0xff) as i32;
    left_channel.abs_diff(right_channel)
}
