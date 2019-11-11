#![no_std]

use core::sync::atomic::{AtomicBool, Ordering};

static IS_COMPLETE: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn mark_complete() {
    IS_COMPLETE.store(true, Ordering::Release);
}

#[inline(always)]
pub fn is_complete() -> bool {
    IS_COMPLETE.load(Ordering::Acquire)
}

#[inline(always)]
pub fn block_until_complete() {
    while !is_complete() {}
}
