extern crate sys_info;

pub mod rust_top {

    use sys_info::*;

    pub struct CpuInfo {
        pub num: u32,
        pub speed: u64
    }

    impl CpuInfo {
        pub fn new() -> Result<CpuInfo, Error> {
            cpu_num().and_then(|num| {
                cpu_speed().map(|speed|
                    CpuInfo {
                        num: num,
                        speed: speed
                    }
                )
            })
        }
    }

    pub struct LoadInfo {
        pub one: f64,
        pub five: f64,
        pub fiveteen: f64
    }

    impl LoadInfo {
        pub fn new() -> Result<LoadInfo, Error> {
            loadavg().map(|loadavg|
                LoadInfo {
                    one: loadavg.one,
                    five: loadavg.five,
                    fiveteen: loadavg.fifteen
                }
            )
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
        pub fn new() -> Result<MemInfo, Error> {
            mem_info().map(|mem|
                MemInfo{
                    total: mem.total,
                    free: mem.free,
                    avail: mem.avail,
                    buffers: mem.buffers,
                    cached: mem.cached
                }
            )
        }
    }

    pub struct DiskInfo {
        pub total: u64,
        pub free: u64
    }

    impl DiskInfo {
        pub fn new() -> Result<DiskInfo, Error> {
            disk_info().map(|disk|
                DiskInfo {
                    total: disk.total,
                    free: disk.free
                }
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_cpu_info() {
            let cpu_info = CpuInfo::new().unwrap();
            assert_eq!(true, cpu_info.num > 0);
            assert_eq!(true, cpu_info.speed > 0);
        }

        #[test]
        fn test_disk_info() {
            let disk_info = DiskInfo::new().unwrap();
            assert_eq!(true, disk_info.free > 0);
            assert_eq!(true, disk_info.total > 0);
        }

        #[test]
        fn test_mem_info() {
            let mem_info = MemInfo::new().unwrap();
            assert_eq!(true, mem_info.free > 0);
            assert_eq!(true, mem_info.total > 0);
        }

        #[test]
        fn test_load_info() {
            let load_info = LoadInfo::new().unwrap();
            assert_eq!(true, load_info.free > 0);
            assert_eq!(true, load_info.total > 0);
        }
    }
}

