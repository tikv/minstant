#[inline]
pub const fn timespec_to_u64(secs: u64, nsecs: u32) -> u64 {
    (secs << 32) | ((nsecs as u64 * 0x225C17D05) >> 31)
}

#[inline]
pub const fn nanos_to_u64(nanos: u64) -> u64 {
    let secs = nanos / 1_000_000_000;
    timespec_to_u64(secs, (nanos - secs * 1_000_000_000) as u32)
}

#[inline]
pub const fn millis_to_u64(millis: u64) -> u64 {
    // is that magic?
    ((millis * 0x4189374C) >> 8) - 0x64
}

#[inline]
pub const fn secs_to_u64(secs: u64) -> u64 {
    secs << 32
}
