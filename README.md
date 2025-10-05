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

I'm using Linux v6.17 on Debian and I needed a single patch for that:

* [`a053ba6b56c8`](https://github.com/bobrik/linux/commit/0c59f006527c) rust: include in deb package for linux-headers

On Ubuntu this stuff is packaged in `linux-lib-rust` (24.04 noble, 24.10 oracular)
or in `linux-headers` (starting with 25.04 plucky). For others you need to find
whatever package includes `libkernel.rmeta` on your distribution.

You will need to use a specific version of Rust, the same one the kernel was
built with. This is taken care of by `Makefile`, which automatically generates
the `rust-toolchain.toml` file to be used by `rustup`.

All you need to do to build the module:

```
make -C rust
```

To insert the module:

```
sudo insmod rust/dmesg_heartbeat.ko
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
