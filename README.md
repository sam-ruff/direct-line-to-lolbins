# DLL Hijacking Demonstration

An educational example demonstrating DLL search order hijacking on Windows.

The loader checks for `greeting.dll` in this order:
- `%USERPROFILE%\GreetingApp\greeting.dll` (user-writable, hijackable)
- `C:\Program Files\GreetingApp\greeting.dll` (admin-protected, safe)

## Building

### Prerequisites (Linux cross-compilation)

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
```

### Build

Build the DLLs separately since they share the same output name:

```bash
cargo build --release --target x86_64-pc-windows-gnu -p safe-dll
cp target/x86_64-pc-windows-gnu/release/greeting.dll safe-greeting.dll

cargo build --release --target x86_64-pc-windows-gnu -p dodgy-dll
cp target/x86_64-pc-windows-gnu/release/greeting.dll dodgy-greeting.dll

cargo build --release --target x86_64-pc-windows-gnu -p loader
cp target/x86_64-pc-windows-gnu/release/loader.exe .
```

Copy `loader.exe`, `safe-greeting.dll`, and `dodgy-greeting.dll` to the Windows machine.

## Demonstration

### Setup (as Administrator)

Install the safe DLL to the protected location:

```powershell
mkdir "C:\Program Files\GreetingApp"
cp safe-greeting.dll "C:\Program Files\GreetingApp\greeting.dll"
```

### Safe scenario

Run the loader with no DLL in the user profile locations:

```powershell
.\loader.exe
```

Output:
```
Loading DLL from safe location: C:\Program Files\GreetingApp\greeting.dll
hello from a safe binary
```

### Hijack scenario

Place the dodgy DLL in the user's app directory:

```powershell
mkdir $env:USERPROFILE\GreetingApp
cp dodgy-greeting.dll $env:USERPROFILE\GreetingApp\greeting.dll
.\loader.exe
```

Output:
```
Loading DLL from user profile app directory: C:\Users\<username>\GreetingApp\greeting.dll
Hello, direct line?
```

### Cleanup

```powershell
rmdir $env:USERPROFILE\GreetingApp -Recurse
```
