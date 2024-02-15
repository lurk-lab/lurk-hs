# lurk-hs
WIP Haskell Wrappers for Lurk

# Build

Regenerate Haskell module with

```
$ nix develop
$ cargo build
$ cabal build
```

Example in src/lib.hs

```rust
#[hs_bindgen]
fn greetings(name: &str) {
    println!("Hello, {name}!");
}
```

generates in `src/LurkHs.hs`

```haskell
foreign import ccall safe "__c_greetings" greetings :: CString -> IO (())
```
