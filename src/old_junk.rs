// use std::mem::MaybeUninit;
// use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_ALL_ACCESS, PROCESS_VM_READ};
// use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
// use windows::Win32::System::ProcessStatus::{K32GetModuleFileNameExW, K32EnumProcessModulesEx, K32GetModuleFileNameExA, LIST_MODULES_ALL};
// use windows::Win32::Foundation::{ HINSTANCE };
// // create ISimpleAudioVolume
// use windows::Win32::Media::Audio::IAudioClient;
// use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
    
// let audio_sess_ctrl_2: IAudioSessionControl2 = unwrap_or_return!(unsafe { audio_client.GetService() }, "audio_sess_ctrl_2");
// let procc_id = unwrap_or_return!(unsafe {audio_sess_ctrl_2.GetProcessId()}, "procc_id");
// println!("{}", procc_id);



// // https://stackoverflow.com/a/14322736
// // this guy fucks https://stackoverflow.com/users/403671/simon-mourier
// // IWISH: let sess_procc_id = unwrap_or_return!(unsafe {  sess.GetProcessId() }, "sess_procc_id");
// // https://stackoverflow.com/a/4570213 -- guess I'm into giving credit, atm
// // todo ---v
// let sess_procc_id = unwrap_or_return!(unsafe { sess2.GetProcessId() }, "sess_procc_id");
// let procc_handle_res =  unsafe {  OpenProcess(PROCESS_ALL_ACCESS, true, sess_procc_id) };

// if n == 0 {
//     println!("---------------MUTING----------------");
    
// }
// if sess_name_str.contains("Firefox") {
//     println!("\nmuting FireFox\n");
//     // let simple_audio = unwrap_or_return!(unsafe {audio_session_mgr_2.GetSimpleAudioVolume(sess_grouping_ptr, 1)}, "simple_audio");
// }

// // https://docs.microsoft.com/en-us/windows/win32/psapi/enumerating-all-modules-for-a-process
// // https://github.com/mgostIH/process_list/blob/master/src/windows/processes.rs
// // 
// let procc_handle_res_ok = procc_handle_res.is_ok();
// if procc_handle_res_ok {
//     let proc_handle = procc_handle_res.unwrap();
//     // let enum_modules_maybe: MaybeUninit<[HINSTANCE; 1024]> = MaybeUninit::uninit();
//     let enum_modules: [HINSTANCE; 1024] = unsafe { std::mem::zeroed() };
//     let enum_modules_ptr: *mut HINSTANCE = (&enum_modules) as *const HINSTANCE as *mut HINSTANCE;
//     let enum_modules_bytes_needed: *mut u32 = 0 as *mut u32;
//     let cb = std::mem::size_of::<[HINSTANCE; 1024]>() as u32;
//     let enum_modules_success = unsafe { K32EnumProcessModulesEx(
//         proc_handle,
//         enum_modules_ptr,
//         cb,
//         enum_modules_bytes_needed,
//         LIST_MODULES_ALL
//     ) }.as_bool();

//     if !enum_modules_success {
//         println!("Failed to get modules for process handle: {}", io::Error::last_os_error());
//     } else {
//         let modules_retrieved: usize = (enum_modules_bytes_needed as u32 as usize) / std::mem::size_of::<HINSTANCE>();
//         println!("Modules retrieved {modules_retrieved}");
//         for x in 0..modules_retrieved {
//             let mut procc_fn_slice: Vec<u16> = vec![];
//             let modules = enum_modules[x];
//             let procc_file_name = unsafe { K32GetModuleFileNameExW(proc_handle, modules, &mut procc_fn_slice) };
//             let procc_name = unwrap_or_continue!(String::from_utf16(&procc_fn_slice), "procc_name");
//             println!("procc: {procc_name}");
//         }
//     }

// }

// println!("{}, ok?{}", sess_iden_str, procc_handle_res_ok);