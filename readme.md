Attempts to call a C library from Rust
<br><br>

### branch `main`
This is my preferred method. A Rust crate wraps the C library.  Other Rust crates can then include the Rust wrapper crate, and not have to worry about any C libraries.
... in theory.

The Rust crate wrapping the C library (`libstm32_cube_programmer_sys`) builds ok. Its tests run ok.
The Rust crate calling the Rust crate which wraps the C library (`caller`) does not build, but reports:
```
    = note: LINK : fatal error LNK1181: cannot open input file '.\drivers\CubeProgrammer_API.lib'
```

- In `build.rs` I initially mis-wrote the C library name, and `libstm32_cube_programmer_sys` did not build. Correcting the library name allowed `libstm32_cube_programmer_sys` to build successfully. So it seems like `libstm32_cube_programmer_sys` does open the C library.
- I tried adding the path to the `drivers` folder to my `PATH`.
- I tried listing the absolute path to the C library:
```
println!("cargo:rustc-link-lib=C:/[blah blah]/drivers/CubeProgrammer_API");
```
I could not find how to feed in the path correctly, without Rust reporting:
```
    error: renaming of the library `C` was specified, however this crate contains no `#[link(...)]` attributes referencing this library.
```
<br><br>

### branch `all_in_one`
In the `main` branch it seemed like maybe the problem was that `libstm32_cube_programmer_sys` could find the C library but `caller` could not. So I tried discarding the separate Rust crate, and having a single Rust crate which both wraps the C library and calls the C functions.

This time I get the following error, plus a bonus warning:
```
    = note: caller.59pofysds2mkvvjr.rcgu.o : error LNK2019: unresolved external symbol disconnect referenced in function _ZN6caller4main17ha79648c0a9e86ed0E
    .\drivers\CubeProgrammer_API.lib : warning LNK4272: library machine type 'x86' conflicts with target machine type 'x64'
```
<br><br>

### branch `link_search`
I searched a lot on the internet and found lots of different ways to call a C library from Rust. One way is to use `link-search` rather than `link-lib`. This surely only makes things harder for the compiler because you make it do more work. But I am stuck and need to try different things!

This time I get the following error, plus the bonus warning:
```
    = note: caller.59pofysds2mkvvjr.rcgu.o : error LNK2019: unresolved external symbol __imp_disconnect referenced in function _ZN6caller4main17ha79648c0a9e86ed0E
    .\drivers\CubeProgrammer_API.lib : warning LNK4272: library machine type 'x86' conflicts with target machine type 'x64'
```
