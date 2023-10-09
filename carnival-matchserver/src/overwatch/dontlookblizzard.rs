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
            PAGE_PROTECTION_FLAGS, MEM_COMMIT, MEM_PRIVATE,
        },
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::WindowsAndMessaging::GetWindowThreadProcessId,
};

const PAGE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);

#[derive(Debug)]
pub struct ScanResult<T> {
    pub address: u64,
    pub value: T
}

impl ScanResult<String> {
    pub fn new(address: u64, value: String) -> Self {
        Self { address, value }
    }
}


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

    pub unsafe fn find_str(&self, input: &str, size: usize) -> Vec<ScanResult::<String>> {
        let mut ret: Vec<ScanResult::<String>> = Vec::new();
        let input_bytes: &[u8] = input.as_bytes();
        let pages = self.page_info();
        let filtered_pages = pages
            .iter()
            .filter(|page| {
                page.Protect & PAGE_MASK != PAGE_PROTECTION_FLAGS(0)
                    && page.State == MEM_COMMIT
                    && page.Type == MEM_PRIVATE
                    // && VALID_REGION_SIZES.contains(&page.RegionSize)
                    && page.PartitionId == 0
                    && page.AllocationProtect == PAGE_PROTECTION_FLAGS(4)
            })
            .collect::<Vec<_>>();

         println!("Filtered page count: {}", filtered_pages.len());

        // TODO: Split this up when I find fucks to give (I will do it tomorrow)
        filtered_pages.iter().for_each(|sub_range| {
            let read_res = self.yoink_bytes(sub_range.BaseAddress as _, sub_range.RegionSize);
            if !read_res.is_none() {
                read_res.unwrap()
                    .windows(input_bytes.len())
                    .enumerate()
                    .for_each(|(offset, window)| {
                        if window == input_bytes {
                            let location = sub_range.BaseAddress as u64 + offset as u64;
                            if let Some(read_str) = self.read_str(location, size) {
                                println!("{:X}: {}", location, read_str);
                                let scan_result = ScanResult::new(location, read_str);
                                ret.push(scan_result);
                            }
                        }
                    });
                }
            });
            ret
        }

    pub unsafe fn retain_valid(&self, prev_scan: &mut Vec<ScanResult::<String>>) {
        prev_scan.retain(|prev_result| {
            if let Some(read) = self.read_str(prev_result.address, prev_result.value.len()) {
                read == prev_result.value
            } else {
                false
            }
        });
    }

    // TODO
    pub unsafe fn retain_static(&self, prev_scan: &mut Vec<(u64, String)>) {}
}
