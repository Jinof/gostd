#![allow(non_snake_case)]

use std::sync::atomic::{AtomicI32};
use std::sync::atomic::Ordering;

use gostd_builtin::{int32, int64, float64};
use std::arch::{global_asm};

// Package sync implement golang sync package.
struct Mutex {
    state: AtomicI32,
    sema: semaphore::Semaphore<int32>,
}

impl Mutex {
    fn lockSlow(&self) {
        let mut waitStartTime: int64 = 0;
        let starving: bool = false;
        let mut awoke: bool = false;
        let mut iter: int32 = 0;
        let mut old  = self.state.load(Ordering::Acquire);
        loop {
            if old&(mutextLocked|mutextStarving) == mutextLocked && runtime_canSpin(iter) {
                if !awoke && old&mutextWoken == 0 && old>>mutextWaitShift != 0 &&
                    self.state.compare_exchange_weak(old, old|mutextWoken, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                        awoke = true;
                    }
                    runtime_doSpin();
                    iter += 1;
                    old = self.state.load(Ordering::Acquire);
                    continue;
            }
            let mut new = old;
            if old&mutextStarving == 0 {
                new |= mutextLocked
            }
            if old&(mutextLocked|mutextStarving) != 0 {
                new += 1 << mutextWaitShift;
            }
            if starving && old&mutextLocked != 0 {
                new |= mutextStarving
            }
            if awoke {
                if new&mutextLocked == 0 {
                    // TODO: relace panic! to throw after impl throw.
                    panic!("sync: inconsistent mutex state");
                }
                new &= !mutextWoken;
            }
            if self.state.compare_exchange(old, new, Ordering::Acquire, Ordering::Relaxed).is_ok() {
                if old&(mutextLocked|mutextStarving) == 0{
                    break;
                }

                let queueLifo: bool = waitStartTime != 0;
                if waitStartTime == 0 {
                    // TODO: replace impl to runtime_nanotime.
                    waitStartTime = gostd_time::Now().Nanosecond() as i64;
                }
                _ = queueLifo;
                // TODO impl runtime package.
            }
        }
    }

    fn TryLock() -> bool {
     false
    }
}

// runtime_canSpin TODO: add func logic after impl package:runtime
// fake logic: return true when canSpin is called for the first time
fn runtime_canSpin(iter: int32) -> bool {
    if iter == 0 {
        true;
    }
    false
}

// runtime_doSpin use 
fn runtime_doSpin() {
    
}



// TODO: move procyield to runtime when finish runtime package
/* golang plan9 asm: procyield
TEXT runtime·procyield(SB),NOSPLIT,$0-0
	MOVL	cycles+0(FP), AX
again:
	PAUSE
	SUBL	$1, AX
	JNZ	again
	RET
*/
global_asm!(r#"
.global procyeild
procyeild:
    MOVL %edi, %rbx
again:
    PAUSE
    SUBL $1, %rbx
    JNZ again
    RET
"#);

extern {
    fn procyeild();
}


pub trait Locker {
    fn Lock(&self);
    fn UnLock(&self);
}

const mutextLocked: int32 = 1 << 0;
const mutextWoken: int32 = 1 << 1;
const mutextStarving: int32 = 1 << 2; 
const mutextWaitShift: int32 = 3;
const starvationThresholdNs: float64 = 1e6;

impl Locker for Mutex {
    fn Lock(&self) {
        if self.state.compare_exchange(0, mutextLocked, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_ok() {
            return
        }

        self.lockSlow()
    }

    fn UnLock(&self) {
        
    }
}