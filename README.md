# KVM

Rust interface to the KVM Hypervisor
[![Build Status](https://travis-ci.org/dschatzberg/kvm.svg?branch=master)]

[[Documentation](https://dschatzberg.github.io/kvm/)]

General documentation for KVM can be found in the
[kernel Docuementation tree](https://kernel.org/doc/Documentation/virtual/kvm/api.txt)
and through
[this LWN article by Josh Triplett](https://lwn.net/Articles/658511/)
## What is KVM?

Kernel-based Virtual Machine (KVM) is a Linux hypervisor which
provides an interface the hardware virtualization extensions of a
machine. In particular, using KVM, a userspace process can set up a
guest VM's address space, provide/receive I/O, and run a Virtual CPU.

## How is it different from QEMU?

QEMU is a userspace process which can use KVM to construct virtual
machines. QEMU is responsible for emulating a full suite of hardware
devices in order to execute a complete operating system. KVM only
provides the means to execute in a hardware enforced "sandbox." User
processes are responsible for the booting and device emulation.

## Requirements
Rust >= 1.2.0

## License

Apache 2.0
