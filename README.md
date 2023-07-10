# dmesg_heartbeat

This is a toy Linux kernel module that prints `🫀` to the console (aka `dmesg`),
so that you can know if the kernel logging is working. Maybe you want to check
whether your logging pipeline works end to end, maybe you just want to learn
how these things work, maybe you just want emojis there. It's your funeral.

The module is implemented in both C and Rust. I started with Rust, but then
I had to re-implement it in C because the Rust version kept crashing.

You'll need linux-headers installed to build either one.

## C version

To build the module:

```
make -C c
```

To insert the module:

```
sudo insmod c/dmesg_heartbeat.ko
```

You should see these pop up in dmesg:

```
ivan@vm:~$ sudo dmesg -T | tail -n3
[Fri May 12 04:42:29 2023] 🫀
[Fri May 12 04:42:39 2023] 🫀
[Fri May 12 04:42:49 2023] 🫀
```

To remove the module:

```
sudo rmmod dmesg_heartbeat
```

The C version is very simple and it keeps the timer in a static variable
at a fixed address, so very little can go wrong (and nothing should).

## Rust version

On Ubuntu Lunar Lobster things might work a little easier:

* https://discourse.ubuntu.com/t/ubuntu-kernel-is-getting-rusty-in-lunar/34977

I'm using Linux v6.5-rc1 on Debian and I needed a few patches for that:

* [`8fac97511408`](https://github.com/bobrik/linux/commit/1f8bdceaca91) rust: enable allocator_api to allow Box usage
* [`a053ba6b56c8`](https://github.com/bobrik/linux/commit/6bee396aee4c) rust: include in deb package for linux-headers

If you are running on aarch64, you'll also need the following (in order):

* [`dd50b8163346`](https://github.com/bobrik/linux/commit/dd50b8163346) arm64: rust: Enable Rust support for AArch64
* [`02d425fa78f6`](https://github.com/bobrik/linux/commit/02d425fa78f6) arm64: rust: Enable PAC support for Rust
* [`0c078d5cca69`](https://github.com/bobrik/linux/commit/0c078d5cca69) arm64: rust: add missing BINDGEN_TARGET_arm64 in rust/Makefile

You will need to use a specific version of Rust, the same one the kernel was
built with. For Linux v6.5-rc1 that's v1.68.2. Same goes for bindgen v0.56.0.
To make this happen with already installed Rust via `rustup`:

```
rustup toolchain add 1.68.2-aarch64-unknown-linux-gnu
rustup default 1.68.2-aarch64-unknown-linux-gnu
```

Adjust the command above if you have a different host architecture.

```
cargo install --locked --version 0.56.0 bindgen
```

You'll also need `rust-src` component:

```
rustup component add rust-src
```

If you try building the module:

```
make -C rust
```

There will be an error, because for some reason it tries to create a file
in the system header directory. You'll just have to let it:

```
sudo chown $(id -u) /usr/src/linux-headers-$(uname -r)/
```

It should build successfully after that.

To insert the module:

```
sudo insmod c/dmesg_heartbeat.ko
```

You should see these pop up in dmesg:

```
ivan@vm:~/projects/printk_heartbeat$ sudo dmesg -T | tail -n3
[Sat May 13 02:39:09 2023] dmesg_heartbeat: 🫀
[Sat May 13 02:39:14 2023] dmesg_heartbeat: 🫀
[Sat May 13 02:39:19 2023] dmesg_heartbeat: 🫀
```

There's also the name of the module present, which Rust macro adds for some
reason when calling `pr_info!()` macro. In C `pr_info` doesn't do this.

To remove the module:

```
sudo rmmod dmesg_heartbeat
```

The Rust version has a wrapper for a `Box`ed `bindings::timer_list`.
It is critical that the timer is stored within a `Box`, because otherwise
it can and will be moved around in memory by Rust and you will get
some nasty kernel complaints and crashes due to mangled memory.

Ideally there should be a `Pin` in addition to `Box`, but `Box` works okay:

* https://doc.rust-lang.org/std/pin/index.html
