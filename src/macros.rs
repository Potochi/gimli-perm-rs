
#[macro_export]
macro_rules! gimli_idx {
    ($r: expr, $c: expr) => {{
        $r * GIMLI_COLS + $c
    }};
}

#[macro_export]
macro_rules! gimli_get {
    ($st: expr, $r: expr, $c: expr) => {{
        $st.state[gimli_idx!($r, $c)]
    }};
}

#[macro_export]
macro_rules! simd_rotate {
    ($s: expr, $shm: expr) => {{
        $s.shl(Simd::<u32, 4>::splat($shm)) | $s.shr(Simd::<u32, 4>::splat(32 - $shm as u32))
    }};
}
