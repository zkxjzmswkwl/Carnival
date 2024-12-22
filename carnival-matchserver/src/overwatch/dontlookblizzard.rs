use core::time;
use std::{
    collections::HashMap,
    ffi::c_void,
    mem::{size_of, MaybeUninit},
    sync::mpsc::{self, Sender},
    thread,
    time::Instant,
};

use windows::Win32::{
    Foundation::HANDLE,
    System::{
        Diagnostics::Debug::ReadProcessMemory,
        Memory::{
            VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_PROTECTION_FLAGS, PAGE_TYPE,
            VIRTUAL_ALLOCATION_TYPE,
        },
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::WindowsAndMessaging::GetWindowThreadProcessId,
};

const PAGE_PROTECTION_MASK: PAGE_PROTECTION_FLAGS =
    PAGE_PROTECTION_FLAGS(0x4 | 0x80 | 0x40 | 0x20 | 0x10);
const PAGE_TYPE_MASK: PAGE_TYPE = PAGE_TYPE(131072u32 | 16777216u32);
const STATE_STRINGS: &[&'static str] = &["2.6.1.1 - 116944", "Jump into", "tinder watch", "Add AI"];

// *mut ::core::ffi::c_void is not able to be passed across threads,
// so we need to make our own type that is.
// Luckily, both BaseAddress and AllocationBase are just u64s.
#[derive(Clone, Copy)]
pub struct THREADSAFE_MEMORY_BASIC_INFO {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}

pub struct ProcessMemory {
    pub pages: Vec<THREADSAFE_MEMORY_BASIC_INFO>,
    pub is_filtered: bool
}

impl ProcessMemory {
    pub unsafe fn new(tank: &Tank) -> Self {
        Self {
            pages: tank.page_info(),
            is_filtered: false
        }
    }
}

impl THREADSAFE_MEMORY_BASIC_INFO {
    pub fn translate(info: &MEMORY_BASIC_INFORMATION) -> Self {
        Self {
            BaseAddress: info.BaseAddress as u64,
            AllocationBase: info.AllocationBase as u64,
            AllocationProtect: info.AllocationProtect,
            PartitionId: info.PartitionId,
            RegionSize: info.RegionSize,
            State: info.State,
            Protect: info.Protect,
            Type: info.Type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScanResult<T> {
    pub address: u64,
    pub value: T,
    pub byte_count: usize,
}

impl<T> ScanResult<T> {
    pub fn new(address: u64, value: T, byte_count: usize) -> Self {
        Self {
            address,
            value,
            byte_count,
        }
    }
}

impl ScanResult<String> {
    // TODO: implement for generics when I have patience to look at rust generics etc.
    pub fn rescan(&self, overwatch: &Tank) -> Option<ScanResult<String>> {
        if let Some(read_val) = overwatch.read_str(self.address, self.byte_count) {
            Some(ScanResult::<String>::new(
                self.address,
                read_val,
                self.byte_count,
            ))
        } else {
            None
        }
    }
}

// TODO: implement this when I have patience to look at rust generics etc.
// pub struct CachedScan<T>(Vec<ScanResult::<T>>);
#[derive(Clone, Debug)]
pub struct CachedScan(Vec<ScanResult<String>>);

impl CachedScan {
    pub fn new(scan_results: Vec<ScanResult<String>>) -> Self {
        Self(scan_results)
    }

    pub fn rescan(&self, overwatch: &Tank) -> CachedScan {
        let mut ret: Vec<ScanResult<String>> = Vec::new();
        self.0.iter().for_each(|cached_result| {
            if let Some(new_result) = cached_result.rescan(overwatch) {
                ret.push(ScanResult::<String>::new(
                    cached_result.address,
                    new_result.value,
                    cached_result.byte_count,
                ));
            }
        });
        log::info!("Rescan result len: {}", ret.len());
        CachedScan::new(ret)
    }
}

#[derive(Clone)]
pub struct Tank {
    proc_handle: HANDLE,
    pub cached_scans: HashMap<String, CachedScan>,
}

impl Tank {
    pub fn new() -> Self {
        let mut pid: u32 = 0;
        let mut handle;
        unsafe {
            let pid_ret =
                GetWindowThreadProcessId(super::get_hwnd().unwrap(), Some(&mut pid as *mut u32));
            handle = OpenProcess(
                PROCESS_VM_READ | PROCESS_QUERY_INFORMATION,
                false,
                pid as u32,
            )
            .unwrap();
            println!("{:#?}, {:#?}", pid_ret, pid);
        }

        Self {
            proc_handle: handle,
            cached_scans: HashMap::new(),
        }
    }

    pub fn _filter_engine_rev(&self) -> usize {
        let cached_scan = self.cached_scans.get("engine_revision");
        if cached_scan.is_none() {
            log::info!("engine_revision is not present in cached_scans.");
            return 5000;
        };
        let cached_scan = cached_scan.unwrap();
        let rescanned = cached_scan.rescan(self);
        rescanned
            .0
            .iter()
            .filter(|predicate| {
                println!("{}", predicate.value);
                predicate.value == "Jump into a game against"
            })
            .count()
    }

    // TODO: This doesn't belong on the tank impl.
    pub fn filter_fps_gt_60(&self) -> usize {
        let cached_scan = self.cached_scans.get("fps_counter");
        if cached_scan.is_none() {
            log::info!("{} is not present in cached_scans.", "fps_counter");
            return 5000;
        };
        let cached_scan = cached_scan.unwrap();
        let rescanned = cached_scan.rescan(self);

        rescanned
            .0
            .iter()
            // Need to pass a closure for this, it won't be the same for every scan, since types can vary.
            // Nevermind rust makes that absolutely cock and ball cbf.
            .filter(|predicate| {
                let p_value_trimmed = predicate.value.replace(" ", "");
                if !p_value_trimmed.contains("FPS:") {
                    return false;
                }

                let split: Vec<&str> = p_value_trimmed.split(":").collect();
                let frame_rate_str = split.last().unwrap();
                if let Ok(frame_rate) = frame_rate_str.parse::<u32>() {
                    // log::info!("{}", frame_rate);
                    frame_rate > 61
                } else {
                    false
                }
            })
            .count()
    }

    pub fn yoink_bytes(&self, location: u64, len: usize) -> Option<Vec<u8>> {
        let mut bytes = vec![0; len];
        unsafe {
            if let Err(_) = ReadProcessMemory(
                self.proc_handle,
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

    pub unsafe fn page_info(&self) -> Vec<THREADSAFE_MEMORY_BASIC_INFO> {
        let mut base = 0;
        let mut regions = Vec::new();
        let mut info = MaybeUninit::uninit();

        loop {
            let written = VirtualQueryEx(
                self.proc_handle,
                Some(base as *const c_void),
                info.as_mut_ptr(),
                size_of::<THREADSAFE_MEMORY_BASIC_INFO>(),
            );
            if written == 0 {
                break regions;
            }

            let info = info.assume_init();
            base = info.BaseAddress as usize + info.RegionSize;

            // Aim is to ignore 0x7FFF* pages.
            // if base > 0x30000000000 {
            //     log::info!("Disregarding pages allocated after 0x{:X}", base);
            //     break regions;
            // }

            // println!("Page @ 0x{:X}", base);
            regions.push(THREADSAFE_MEMORY_BASIC_INFO::translate(&info));
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
            }
            None => None,
        }
    }

    pub fn filter_pages(&self, pages: &mut Vec<THREADSAFE_MEMORY_BASIC_INFO>) {
        pages.retain(|page| {
            page.Protect & PAGE_PROTECTION_MASK != PAGE_PROTECTION_FLAGS(0)
                && page.State == MEM_COMMIT
                && page.Type & PAGE_TYPE_MASK != PAGE_TYPE(0)
                && page.BaseAddress < 0x30000000000
            // && VALID_REGION_SIZES.contains(&page.RegionSize)
            // && page.PartitionId == 0
        });
    }

    // TODO: remove me and refactor find_str to take more types than just &str.
    pub unsafe fn scan_i32(&self, input: i32) -> Vec<ScanResult<i32>> {
        // Need to convert &[u8] to u/i32. Manually I guess?
        fn to_i32_little_endian(out: &[u8]) -> i32 {
            ((out[0] as i32) << 24) +
            ((out[1] as i32) << 16) +
            ((out[2] as i32) << 8)  +
            ((out[3] as i32) << 0)
        }
        let mut ret: Vec<ScanResult<i32>> = Vec::new();
        let mut pages = self.page_info();
        self.filter_pages(&mut pages);

        log::info!("Filtered page count: {}", pages.len());

        for page in pages {
            let read_res = self.yoink_bytes(page.BaseAddress as _, page.RegionSize);

            if read_res.is_none() {
                continue;
            }
            for (offset, window) in read_res.unwrap().windows(4).enumerate() {
                if to_i32_little_endian(window) == input {
                    let location = page.BaseAddress as u64 + offset as u64;
                    if let Some(read_val) = self.yoink_bytes(location, 4) {
                        let read_val_slice = to_i32_little_endian(&read_val[0..4]);
                        // log::info!(
                        //     "{:X}: {:#?} ({})",
                        //     location,
                        //     read_val_slice,
                        //     4
                        // );
                        let scan_result = ScanResult::new(location, read_val_slice, 4);
                        ret.push(scan_result);
                    }
                }
            }
        }
        ret
       
    }

    pub unsafe fn find_str(&self, input: &str, size: usize) -> Vec<ScanResult<String>> {
        let mut ret: Vec<ScanResult<String>> = Vec::new();
        let input_bytes: &[u8] = input.as_bytes();

        let mut pages = self.page_info();
        self.filter_pages(&mut pages);

        log::info!("Filtered page count: {}", pages.len());

        for page in pages {
            let read_res = self.yoink_bytes(page.BaseAddress as _, page.RegionSize);

            if read_res.is_none() {
                continue;
            }
            for (offset, window) in read_res.unwrap().windows(input_bytes.len()).enumerate() {
                if window == input_bytes {
                    let location = page.BaseAddress as u64 + offset as u64;
                    if let Some(read_str) = self.read_str(location, size) {
                        log::info!(
                            "{:X}: {} ({})",
                            location,
                            read_str,
                            read_str.as_bytes().len()
                        );
                        let scan_result = ScanResult::new(location, read_str, size);
                        ret.push(scan_result);
                    }
                }
            }
        }
        ret
    }

    pub unsafe fn _retain_valid(&self, prev_scan: &mut Vec<ScanResult<String>>) {
        prev_scan.retain(|prev_result| {
            if let Some(read) = self.read_str(prev_result.address, prev_result.value.len()) {
                read == prev_result.value
            } else {
                false
            }
        });
    }

    // TODO
    pub unsafe fn _retain_static(&self, _prev_scan: &mut Vec<(u64, String)>) {}

    // Thread count observations:
    //  - 50
    //      - Very high cpu
    //  - 30
    //      - Very high cpu
    //  - 15
    //      - High cpu
    //  - 7
    //      - Considerable-ish cpu, not bad. 25%~ improvement in speed over 5 threads on my machine.
    //  - 5
    //      - Good but 4.3s avg on my machine for finding engine revision.
    // TODO(Carter):
    //  - Call this once for multiple strings.
    //  - Don't spawn new threads every call, reuse old ones (Send them instructions)
    //  - Profile it properly
    //  - Goal is to scan all relevant pages for ~8 strings in <= 1s.
    pub fn turboscan(&self, process_memory: &mut ProcessMemory, input: &str, break_at_count: usize) -> Vec<ScanResult<String>> {
        fn worker(
            sender: Sender<Vec<ScanResult<String>>>,
            pages: &[THREADSAFE_MEMORY_BASIC_INFO],
            input: &str,
            size: usize,
            tank: &Tank,
            break_at_count: usize,
        ) {
            let mut ret: Vec<ScanResult<String>> = Vec::new();
            let input_bytes: &[u8] = input.as_bytes();

            'outter: for page in pages {
                let page_bytes = tank.yoink_bytes(page.BaseAddress as _, page.RegionSize);
                if page_bytes.is_none() {
                    continue;
                }
                for (offset, window) in page_bytes.unwrap().windows(input_bytes.len()).enumerate() {
                    let location = page.BaseAddress as u64 + offset as u64;
                    if window == input_bytes {
                        if let Some(read_str) = tank.read_str(location, size) {
                            // log::info!("[{:#?}]\t{:X}: {} ({})", thread::current().id(), location, read_str, read_str.as_bytes().len());
                            let scan_result = ScanResult::new(location, read_str, size);
                            ret.push(scan_result);
                            if ret.len() >=  break_at_count {
                                break 'outter;
                            }
                        }
                    }
                }
            }

            match sender.send(ret) {
                Ok(_) => {}
                Err(e) => log::error!("{e}"),
            }
        }

        let mut all_results: Vec<ScanResult<String>> = Vec::new();
        unsafe {
            // For cross-thread communication
            // We only need one receiver, for the main thread.
            // But we'll need to clonse the sender for as many threads as we spawn.
            let (sender, receiver) = mpsc::channel::<Vec<ScanResult<String>>>();

            // How many pages each thread will be responsible for.
            // without handling the remainder, we have the chance of a few pages.
            let pages_per_thread = process_memory.pages.len() / 16;
            // println!("pages_per_thread: {}\ntotal pages: {}", pages_per_thread, pages.len());

            let mut pages_assigned = 0;
            let mut pool = Vec::<thread::JoinHandle<()>>::new();

            for _ in 0..16 {
                let page_slice = &process_memory.pages[pages_assigned..pages_assigned + pages_per_thread];
                pages_assigned += pages_per_thread;
                let cloned_sender = sender.clone();
                let cloned_tank = self.clone();
                let cloned_input = input.to_string().clone();
                let mut cloned_slice = Vec::with_capacity(pages_per_thread);
                cloned_slice.extend_from_slice(&page_slice);

                let test = thread::spawn(move || {
                    worker(
                        cloned_sender,
                        &cloned_slice,
                        &cloned_input,
                        cloned_input.len(),
                        &cloned_tank,
                        break_at_count,
                    );
                });
                pool.push(test);
            }

            let start = Instant::now();
            for thr in pool.into_iter() {
                if let Ok(scan_result) = receiver.recv_timeout(time::Duration::from_secs(3)) {
                    all_results.extend(scan_result);
                }
                thr.join().unwrap();
            }
            println!("Scanned {} pages for '{}' value in {}. Found {} results.", process_memory.pages.len(), input, start.elapsed().as_secs_f32(), all_results.len());
        }
        all_results
    }
}
