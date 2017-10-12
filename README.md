# SputnikVM Callback Wrapper

[![Build Status](https://travis-ci.org/ethereumproject/sputnikvm-callback.svg?branch=master)](https://travis-ci.org/ethereumproject/sputnikvm-callback)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE)
[![Cargo](https://img.shields.io/crates/v/sputnikvm-callback.svg)](https://crates.io/crates/sputnikvm-callback)

This callback wrapper allows you to invoke the SputnikVM library using
a callback-like style instead of the RequireError style.

Note that however, the RequireError style is more flexible and works
better when doing concurrent programming. So this is a SputnikVM
[Contrib
Project](https://github.com/ethereumproject/sputnikvm/wiki/Contrib-Projects).

## Get Started

Add `sputnikvm-callback` to your dependency and implement the
`sputnikvm_callback::Callback` trait. After that, you can first create
a normal VM, and then use:

```
let callback_vm = CallbackVM::new(vm, callback);
```

After that, invoke `fire` on the `callback_vm` would no longer needs
to deal with `RequireError`.
