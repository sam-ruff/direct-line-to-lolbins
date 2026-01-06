use libloading::{Library, Symbol};
use std::env;
use std::path::PathBuf;

type GreetFn = extern "C" fn();

fn get_user_profile_dll_path() -> Option<PathBuf> {
    env::var("USERPROFILE")
        .ok()
        .map(|profile| PathBuf::from(profile).join("greeting.dll"))
}

fn get_user_profile_app_dll_path() -> Option<PathBuf> {
    env::var("USERPROFILE")
        .ok()
        .map(|profile| PathBuf::from(profile).join("GreetingApp").join("greeting.dll"))
}

fn get_safe_dll_path() -> PathBuf {
    // Admin-protected location - requires elevated privileges to write
    PathBuf::from(r"C:\Program Files\GreetingApp\greeting.dll")
}

fn load_library(path: &impl AsRef<std::path::Path>) -> Result<Library, libloading::Error> {
    unsafe { Library::new(path.as_ref()) }
}

fn call_greet(lib: &Library) -> Result<(), libloading::Error> {
    unsafe {
        let greet: Symbol<GreetFn> = lib.get(b"greet")?;
        greet();
        Ok(())
    }
}

fn try_load_and_greet(path: &PathBuf, description: &str) -> bool {
    if !path.exists() {
        return false;
    }

    println!("Loading DLL from {}: {}", description, path.display());
    match load_library(path) {
        Ok(lib) => {
            if let Err(e) = call_greet(&lib) {
                eprintln!("Failed to call greet: {}", e);
            }
            true
        }
        Err(e) => {
            eprintln!("Failed to load: {}", e);
            false
        }
    }
}

fn main() {
    // Search order demonstrates DLL hijack vulnerability:
    // 1. User profile root (most easily hijacked)
    // 2. User profile app directory (user-writable)
    // 3. Program Files (admin-protected, safe)

    if let Some(path) = get_user_profile_dll_path()
        && try_load_and_greet(&path, "user profile")
    {
        return;
    }

    if let Some(path) = get_user_profile_app_dll_path()
        && try_load_and_greet(&path, "user profile app directory")
    {
        return;
    }

    let safe_path = get_safe_dll_path();
    if !try_load_and_greet(&safe_path, "safe location") {
        eprintln!("No DLL found in any search location");
        std::process::exit(1);
    }
}
