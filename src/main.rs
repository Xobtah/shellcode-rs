#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod utils;

use core::ffi::c_void;
use windows_sys::core::{PCSTR, PSTR};
use windows_sys::Win32::Foundation::{FARPROC, HMODULE};
use windows_sys::Win32::Networking::WinSock::{
    AF_INET, IPPROTO_TCP, SOCKADDR, SOCKADDR_IN, SOCKET, SOCK_STREAM, WSADATA, WSAPROTOCOL_INFOA,
};
use windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE;
use windows_sys::Win32::System::Diagnostics::Debug::IMAGE_NT_HEADERS64;
use windows_sys::Win32::System::Kernel::LIST_ENTRY;
use windows_sys::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY};
use windows_sys::Win32::System::Threading::{
    PEB, PEB_LDR_DATA, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA,
};
use windows_sys::Win32::System::WindowsProgramming::LDR_DATA_TABLE_ENTRY;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: u8, n: usize) -> *mut u8 {
    let end = s.add(n);
    let mut p = s;
    while p < end {
        *p = c;
        p = p.add(1);
    }
    s
}

// type LoadLibraryAFn = extern "system" fn(lpLibFileName: PCSTR) -> HMODULE;
// type GetProcAddressFn = extern "system" fn(hModule: HMODULE, lpProcName: PCSTR) -> FARPROC;
// type GetModuleHandleAFn = extern "system" fn(lpModuleName: PCSTR) -> HMODULE;

type WinExecFn = extern "system" fn(lpCmdLine: PCSTR, uCmdShow: u32) -> u32;

// type GetStdHandleFn = extern "system" fn(nStdHandle: u32) -> windows_sys::Win32::Foundation::HANDLE;
// type WriteConsoleAFn = extern "system" fn(
//     hConsoleOutput: windows_sys::Win32::Foundation::HANDLE,
//     lpBuffer: *const c_void,
//     nNumberOfCharsToWrite: u32,
//     lpNumberOfCharsWritten: *mut u32,
//     lpReserved: *mut c_void,
// ) -> windows_sys::Win32::Foundation::BOOL;

// #[no_mangle]
// unsafe fn _start() {
// let k32_h = get_module_handle(obfstr::wide!("KERNEL32.DLL\0")).unwrap();
// let LoadLibraryA = get_function::<LoadLibraryAFn>(k32_h, "LoadLibraryA\0").unwrap();
// let ws2_h = LoadLibraryA("ws2_32.dll\0".as_ptr());

// let WinExec = get_function::<WinExecFn>(k32_h, "WinExec\0").unwrap();
// let GetStdHandle = get_function::<GetStdHandleFn>(k32_h, "GetStdHandle\0").unwrap();
// let WriteConsoleA = get_function::<WriteConsoleAFn>(k32_h, "WriteConsoleA\0").unwrap();

// // Hello, world!
// let hello = "Hello, world !\n\0";
// WriteConsoleA(
//     GetStdHandle(STD_OUTPUT_HANDLE),
//     hello.as_ptr() as *const c_void,
//     hello.len() as u32,
//     core::ptr::null_mut(),
//     core::ptr::null_mut(),
// );

// // Spawn calc.exe
// WinExec("calc.exe\0".as_ptr(), 0);
// }

type WSAStartupFn = extern "system" fn(wVersionRequested: u16, lpWSAData: *mut WSADATA) -> i32;
// type WSACleanupFn = extern "system" fn() -> i32;
// type WSASocketAFn = extern "system" fn(
//     af: i32,
//     type_: i32,
//     protocol: i32,
//     lpProtocolInfo: *mut WSAPROTOCOL_INFOA,
//     g: u32,
//     dwFlags: u32,
// ) -> SOCKET;
// type htonsFn = extern "system" fn(hostshort: u16) -> u16;
// type inet_addrFn = extern "system" fn(cp: *const i8) -> u32;
// type connectFn = extern "system" fn(s: SOCKET, name: *const SOCKADDR, namelen: i32) -> i32;
// type closesocketFn = extern "system" fn(s: SOCKET) -> i32;
// type CreateProcessAFn = extern "system" fn(
//     lpApplicationName: PCSTR,
//     lpCommandLine: PSTR,
//     lpProcessAttributes: *mut c_void,
//     lpThreadAttributes: *mut c_void,
//     bInheritHandles: i32,
//     dwCreationFlags: u32,
//     lpEnvironment: *mut c_void,
//     lpCurrentDirectory: *mut c_void,
//     lpStartupInfo: *mut STARTUPINFOA,
//     lpProcessInformation: *mut PROCESS_INFORMATION,
// ) -> i32;

#[no_mangle]
unsafe fn _start() {
    let k32_h = get_module_handle(obfstr::wide!("KERNEL32.DLL\0"));
    get_function::<WinExecFn>(k32_h, "WinExec\0")("calc.exe\0".as_ptr(), 0);
}

unsafe fn get_module_handle(module_name: &[u16]) -> HMODULE {
    let peb_ptr: *const PEB;
    #[cfg(target_pointer_width = "32")]
    core::arch::asm!("mov eax, fs:[0x30]", out("eax") peb_ptr);
    #[cfg(target_pointer_width = "64")]
    core::arch::asm!("mov rax, gs:[0x60]", out("rax") peb_ptr);
    let ldr_ptr: *const PEB_LDR_DATA = (*peb_ptr).Ldr;
    let module_list: LIST_ENTRY = (*ldr_ptr).InMemoryOrderModuleList;
    let mut module_ptr: *const LIST_ENTRY = (*(module_list.Flink as *const LIST_ENTRY)).Flink;

    while module_ptr != module_list.Flink as *const LIST_ENTRY {
        // TODO "2 * core::mem::size_of::<c_void>()" for 32-bit
        let module_base: *const LDR_DATA_TABLE_ENTRY =
            (module_ptr as usize - 0x10usize) as *const LDR_DATA_TABLE_ENTRY;
        if contains((*module_base).FullDllName.Buffer, module_name.as_ptr()) {
            return (*module_base).DllBase as HMODULE;
        }
        module_ptr = (*module_ptr).Flink;
    }
    -1
}

unsafe fn get_function<T: Sized>(module_handle: HMODULE, function_name: &str) -> T {
    core::mem::transmute_copy::<FARPROC, T>(&get_proc_address(module_handle, function_name))
}

unsafe fn get_proc_address(module_handle: HMODULE, function_name: &str) -> FARPROC {
    let dos_header: IMAGE_DOS_HEADER = *(module_handle as *const IMAGE_DOS_HEADER);
    let nt_headers64: IMAGE_NT_HEADERS64 =
        *((module_handle + dos_header.e_lfanew as isize) as *const IMAGE_NT_HEADERS64);
    let export_dir: IMAGE_EXPORT_DIRECTORY = *((module_handle
        + nt_headers64.OptionalHeader.DataDirectory[0].VirtualAddress as isize)
        as *const IMAGE_EXPORT_DIRECTORY);
    let export_name_table_ptr: *const u32 =
        (module_handle + export_dir.AddressOfNames as isize) as *const u32;

    for i in 0..export_dir.NumberOfNames {
        let export_name_ptr: *const u8 =
            (module_handle + *export_name_table_ptr.offset(i as isize) as isize) as *const u8;
        if equal_pointers(export_name_ptr, function_name.as_ptr()) {
            let address_of_ordinal_table_ptr: *const u16 =
                (module_handle + export_dir.AddressOfNameOrdinals as isize) as *const u16;
            let ordinal = *address_of_ordinal_table_ptr.offset(i as isize);
            let address_of_func_table_ptr: *const u32 =
                (module_handle + export_dir.AddressOfFunctions as isize) as *const u32;
            let func_rva = *address_of_func_table_ptr.offset(ordinal as isize);
            return Some(core::mem::transmute(module_handle + func_rva as isize));
        }
    }
    None
}

/*
 *  Utils
 */
unsafe fn slicify<'a, T: PartialEq + Default>(ptr: *const T) -> &'a [T] {
    core::slice::from_raw_parts(
        ptr,
        (0..)
            .take_while(|&i| *ptr.offset(i) != T::default())
            .count(),
    )
}

unsafe fn equal_pointers<T: PartialEq + Default>(st: *const T, nd: *const T) -> bool {
    equal_slices(slicify(st), slicify(nd))
}

unsafe fn contains<T: PartialEq + Default>(context: *const T, subject: *const T) -> bool {
    let s_slc = slicify(subject);
    slicify(context)
        .windows(s_slc.len())
        .any(|w| equal_slices(w, s_slc))
}

fn equal_slices<T: PartialEq>(context: &[T], subject: &[T]) -> bool {
    if context.len() != subject.len() {
        return false;
    }
    context.iter().zip(subject.iter()).all(|(a, b)| a == b)
}

// unsafe fn cprint() {
//     let k32_h = get_module_handle(obfstr::wide!("KERNEL32.DLL\0"));
//
//     let msg = "Coucou ;)\n\0";
//     let GetStdHandle = get_function::<GetStdHandleFn>(k32_h, "GetStdHandle\0");
//     let WriteConsoleA = get_function::<WriteConsoleAFn>(k32_h, "WriteConsoleA\0");
//     WriteConsoleA(
//         GetStdHandle(STD_OUTPUT_HANDLE),
//         msg.as_ptr() as *const c_void,
//         msg.len() as u32,
//         core::ptr::null_mut(),
//         core::ptr::null_mut(),
//     );
// }
