use chrono::Local;
use sysinfo::{CpuExt, System, SystemExt};
use crate::{models::UsageReport, files_generator};
use files_generator::creat_usage_report;


pub fn start_usage_monitor() {
    let mut sys = System::new();
    let report_start =  Local::now();
    let mut cpu_usage:Vec<f32> = vec![];
    let mut ram_usage:Vec<u64> = vec![];
    
    rayon::spawn(move || loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        cpu_usage.push(sys.global_cpu_info().cpu_usage());
        ram_usage.push(sys.used_memory());  

        let report_end = Local::now();
        creat_usage_report(UsageReport {
            start_time: report_start.naive_local(),
            end_time: report_end.naive_local(),
            cpu_usage: cpu_usage.clone(),
            ram_usage: ram_usage.clone(),
        });

        std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    });
}