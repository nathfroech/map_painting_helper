On Windows you need to create `mise.local.toml` with something like that:
```toml
[env]
CARGO_BUILD_TARGET = "x86_64-pc-windows-gnu"
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "C:\\msys64\\mingw64\\bin\\gcc.exe"
_.path = ["C:\\msys64\\mingw64\\bin"]
```
