# Docs

SyncA contains a documentation processor that allows you to generate 
two versions of documentation: for synchronous and asynchronous versions.

## Issues

- [Support multi-mod in docs processor](https://github.com/sgr-team/rs_synca/issues/5)

## Blocks

To create a synchronous or asynchronous block, use [synca::async] and [synca::sync].

```rust
#[synca::synca(feature = "tokio")] 
mod my_mod {
  /// # My struct
  /// 
  /// [synca::sync]
  /// My sync block
  /// [/synca::sync]
  /// [synca::async]
  /// Async block of docs.
  /// Multiline
  /// [/synca::async]
  pub struct MyStruct { }
}
```

## Match

To replace part of a string use [synca::match].

```rust
#[synca::synca(feature = "tokio")] 
mod my_mod {
  /// # My struct
  /// 
  /// Featured substring: "[synca::match]My async doc|Sync version[/synca::match]"
  pub struct MyStruct { }
}
```