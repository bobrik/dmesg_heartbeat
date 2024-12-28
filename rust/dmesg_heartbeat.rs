//! Print heartbeat into dmesg on a timer.

use core::ptr::null_mut;
use kernel::{bindings, prelude::*};

const INTERVAL_SECONDS: u64 = 5;

module! {
    type: RustOutOfTree,
    name: "dmesg_heartbeat",
    author: "Ivan Babrou <dmesg_heartbeat@ivan.computer>",
    description: "Print heartbeat into dmesg on a timer",
    license: "GPL",
}

struct RustOutOfTree {
    #[allow(dead_code)]
    timer: Timer,
}

struct Timer {
    inner: KBox<bindings::timer_list>,
}

impl Timer {
    fn new() -> Self {
        let inner = KBox::<bindings::timer_list>::new_uninit(GFP_KERNEL).unwrap();
        let mut inner = unsafe { inner.assume_init() };

        unsafe {
            bindings::init_timer_key(
                &mut *inner as *mut _,
                Some(Self::timer_callback),
                0,
                null_mut(),
                null_mut(),
            );
        }

        Self { inner }
    }

    fn setup(&mut self) {
        let inner = &mut *self.inner as *mut _;
        Self::arm(inner);
    }

    fn arm(timer: *mut bindings::timer_list) {
        let jiffies = unsafe { bindings::jiffies };
        let expiration = jiffies + bindings::CONFIG_HZ as u64 * INTERVAL_SECONDS;

        unsafe {
            bindings::mod_timer(timer, expiration);
        }
    }

    extern "C" fn timer_callback(timer: *mut bindings::timer_list) {
        pr_info!("🫀\n");
        Self::arm(timer);
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let inner = &mut *self.inner as *mut _;

        unsafe {
            bindings::timer_delete_sync(inner);
        }
    }
}

/// # Safety
///
/// Trust me bro, it's as safe as it gets here.
unsafe impl Sync for Timer {}
unsafe impl Send for Timer {}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let mut timer = Timer::new();
        timer.setup();

        Ok(RustOutOfTree { timer })
    }
}
