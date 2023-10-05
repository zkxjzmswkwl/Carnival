use std::{
    ffi::{c_ulonglong, c_void},
    mem::{size_of, MaybeUninit},
};

use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::ReadProcessMemory,
        Memory::{
            VirtualQueryEx, MEMORY_BASIC_INFORMATION,
            PAGE_PROTECTION_FLAGS,
        },
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::WindowsAndMessaging::GetWindowThreadProcessId,
};

const PAGE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32 | 128u32 | 4u32 | 8u32);

pub struct Tank(HANDLE);

impl Tank {
    pub unsafe fn new() -> Self {
        let mut pid: u32 = 0;
        let pid_ret =
            GetWindowThreadProcessId(super::get_hwnd().unwrap(), Some(&mut pid as *mut u32));
        println!("{:#?}, {:#?}", pid_ret, pid);
        Self(
            OpenProcess(
                PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                false,
                pid as u32,
            )
            .unwrap(),
        )
    }

    /// 2B19E912488 (Dynamic)
    pub fn yoink_bytes(&self, location: u64, len: usize) -> Option<Vec<u8>> {
        let mut bytes = vec![0; len];
        unsafe {
            if let Err(e) = ReadProcessMemory(
                self.0,
                location as _,
                bytes.as_mut_ptr() as _,
                bytes.len() as _,
                None,
            ) {
                None
            } else {
                Some(bytes)
            }
        }
    }

    unsafe fn page_info(&self) -> Vec<MEMORY_BASIC_INFORMATION> {
        let mut base = 0;
        let mut regions = Vec::new();
        let mut info = MaybeUninit::uninit();

        loop {
            let written = VirtualQueryEx(
                self.0,
                Some(base as *const c_void),
                info.as_mut_ptr(),
                size_of::<MEMORY_BASIC_INFORMATION>(),
            );
            if written == 0 {
                break regions;
            }

            let info = info.assume_init();
            base = info.BaseAddress as usize + info.RegionSize;
            regions.push(info);
        }
    }

    pub fn read_str(&self, location: u64, len: usize) -> Option<String> {
        let bytes = vec![0; len];
        match self.yoink_bytes(location, bytes.len()) {
            Some(out) => {
                if let Ok(to_str) = String::from_utf8(out) {
                    return Some(to_str);
                }
                None
            },
            None => None,
        }
    }

    pub unsafe fn find_str(&self, input: &str) -> Vec<(u64, String)> {
        let mut ret: Vec<(u64, String)> = Vec::new();
        let input_bytes: &[u8] = input.as_bytes();
        let pages = self.page_info();
        let range = pages
            .iter()
            .filter(|page| page.Protect & PAGE_MASK != PAGE_PROTECTION_FLAGS(0))
            .collect::<Vec<_>>();

        // TODO: Split this up when I find fucks to give (I will do it tomorrow)
        range.iter().for_each(|sub_range| {
            let read_res = self.yoink_bytes(sub_range.BaseAddress as _, sub_range.RegionSize);
            if !read_res.is_none() {
                read_res.unwrap()
                    .windows(input_bytes.len())
                    .enumerate()
                    .for_each(|(offset, window)| {
                        if window == input_bytes {
                            let location = sub_range.BaseAddress as u64 + offset as u64;
                            if let Some(read_str) = self.read_str(location, input_bytes.len()) {
                                ret.push((location, read_str));
                            }
                        }
                    });
                }
            });
            ret
        }

    pub unsafe fn retain_valid(&self, prev_scan: &mut Vec<(u64, String)>) {
        prev_scan.retain(|prev_result| {
            if let Some(read) = self.read_str(prev_result.0, prev_result.1.len()) {
                read == prev_result.1
            } else {
                false
            }
        });
    }

    // TODO
    pub unsafe fn retain_static(&self, prev_scan: &mut Vec<(u64, String)>) {}
}
