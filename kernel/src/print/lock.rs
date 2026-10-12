use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

/// x86_64 CPUの割り込み状態
#[derive(Clone, Copy)]
pub struct InterruptState {
    enabled: bool,
}

/// 現在の割り込み状態を取得する
#[inline]
pub fn interrupt_state() -> InterruptState {
    let flags: u64;

    unsafe {
        core::arch::asm!(
            "pushfq",
            "pop {}",
            out(reg) flags,
            options(nomem, preserves_flags)
        );
    }

    InterruptState {
        enabled: (flags & (1 << 9)) != 0,
    }
}

/// CPUの割り込みを無効化する
#[inline]
pub fn disable_interrupts() {
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}

/// CPUの割り込みを有効化する
#[inline]
pub fn enable_interrupts() {
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// 保存した割り込み状態を復元する
#[inline]
pub fn restore_interrupts(state: InterruptState) {
    if state.enabled {
        enable_interrupts();
    } else {
        disable_interrupts();
    }
}

/// カーネル用スピンロック
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

/// SpinLockによって排他アクセスを保証する。
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// 新しいSpinLockを作成する
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// 通常のSpinLockを取得する
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinLockGuard {
            lock: self,
            interrupt_state: None,
        }
    }

    /// 割り込みを停止してSpinLockを取得する
    ///
    /// Guardが破棄されると、取得前の割り込み状態へ戻る。
    pub fn lock_irqsave(&self) -> SpinLockGuard<'_, T> {
        let state = interrupt_state();

        disable_interrupts();

        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinLockGuard {
            lock: self,
            interrupt_state: Some(state),
        }
    }
}

/// SpinLockのGuard
pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
    interrupt_state: Option<InterruptState>,
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);

        if let Some(state) = self.interrupt_state {
            restore_interrupts(state);
        }
    }
}
