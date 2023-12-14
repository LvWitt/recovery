use crate::{files_generator, models::UsageReport};
use chrono::Local;
use files_generator::creat_usage_report;
use sysinfo::{CpuExt, System, SystemExt};

pub fn start_usage_monitor() {
    let mut sys = System::new();
    let report_start = Local::now();
    let mut cpu_usage: Vec<f32> = vec![];
    let mut ram_usage: Vec<f32> = vec![];

    rayon::spawn(move || loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        let usedm = sys.used_memory() as f32;
        let totalm = sys.total_memory() as f32;

        cpu_usage.push(sys.global_cpu_info().cpu_usage() / 100.00);
        ram_usage.push(usedm / totalm);
       //println!("{:?}", ram_usage);
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
