extern crate sys_info;

pub mod rust_top {

    use sys_info::*;

    pub struct CpuInfo {
        pub num: u32,
        pub speed: u64
    }

    impl CpuInfo {
        pub fn new() -> CpuInfo {
            CpuInfo {
                num: cpu_num().unwrap(),
                speed: cpu_speed().unwrap()
            }
        }
    }

    pub struct LoadInfo {
        pub one: f64,
        pub five: f64,
        pub fiveteen: f64
    }

    impl LoadInfo {
        pub fn new() -> LoadInfo {
            let load = loadavg().unwrap();
            LoadInfo {
                one: load.one,
                five: load.five,
                fiveteen: load.fifteen
            }
        }
    }

    pub struct MemInfo {
        pub total: u64,
        pub free: u64,
        pub avail: u64,
        pub buffers: u64,
        pub cached: u64
    }

    impl MemInfo {
        pub fn new() -> MemInfo {
            let mem = mem_info().unwrap();
            MemInfo{
                total: mem.total,
                free: mem.free,
                avail: mem.avail,
                buffers: mem.buffers,
                cached: mem.cached
            }
        }
    }

    pub struct SwapInfo {
        pub total: u64,
        pub free: u64
    }

    impl SwapInfo {
        pub fn new() -> SwapInfo {
            let mem = mem_info().unwrap();
            SwapInfo{
                total: mem.swap_total,
                free: mem.swap_free
            }
        }
    }

    pub struct DiskInfo {
        pub total: u64,
        pub free: u64
    }

    impl DiskInfo {
        pub fn new() -> DiskInfo {
            let disk = disk_info().unwrap();
            DiskInfo {
                total: disk.total,
                free: disk.free
            }
        }
    }

    pub struct SysInfo {
        pub os: String,
        pub cpu_info: CpuInfo,
        pub load_info: LoadInfo,
        pub mem_info: MemInfo,
        pub swap_info: SwapInfo,
        pub disk_info: DiskInfo
    }

    impl SysInfo {
        pub fn new() -> SysInfo {
            SysInfo {
                os: os_type().unwrap() + " " + &os_release().unwrap(),
                cpu_info: CpuInfo::new(),
                load_info: LoadInfo::new(),
                mem_info: MemInfo::new(),
                swap_info: SwapInfo::new(),
                disk_info: DiskInfo::new()
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let sys_info = SysInfo::new();
            assert_eq!(true, sys_info.cpu_info.num > 0);
            assert_eq!(true, sys_info.cpu_info.speed > 0);
        }
    }
}

