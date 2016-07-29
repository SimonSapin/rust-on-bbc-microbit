// Very approximate, found experimentally.
const ITERS_PER_MILLISECOND: u32 = 1500;

pub fn wait_approx_ms(ms: u32) {
    for i in 0..ms {
        for j in 0..ITERS_PER_MILLISECOND {
            black_box((i, j));
        }
    }
}

// Copied from https://github.com/rust-lang/rust/blob/f5d79521a/src/libtest/lib.rs#L1204
fn black_box<T>(dummy: T) -> T {
    // we need to "use" the argument in some way LLVM can't
    // introspect.
    unsafe { asm!("" : : "r"(&dummy)) }
    dummy
}
