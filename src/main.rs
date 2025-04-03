use battery::Manager;
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt, ComponentExt};

fn get_battery_health() -> Option<String> {
    let manager = Manager::new().ok()?;
    let batteries = manager.batteries().ok()?;

    for battery in batteries {
        let bat = battery.ok()?;
        let current_capacity = bat.energy_full().value; // Current max capacity (mWh)
        let design_capacity = bat.energy_full_design().value; // Original capacity (mWh)
        let cycle_count = bat.cycle_count().unwrap_or(0); // Charge-discharge cycles
        let health = (current_capacity / design_capacity) * 100.0; // Health percentage

        return Some(format!(
            "🔋 Battery Health: {:.1}% ({} cycles)\n\
             📉 Current Max Capacity: {:.2}Wh\n\
             🏭 Design Capacity: {:.2}Wh",
            health, cycle_count, 
            current_capacity / 1000.0,  // Convert mWh to Wh
            design_capacity / 1000.0   // Convert mWh to Wh
        ));
    }

    None
}

fn get_graphics_info(sys: &System) -> String {
    // Try to get GPU information (sysinfo has limited GPU support)
    if let Some(gpu) = sys.components().iter().find(|c| c.label().contains("GPU")) {
        format!("🎮 Graphics: {} | Temp: {:.1}°C", gpu.label(), gpu.temperature())
    } else {
        "🎮 Graphics: Information not available".to_string()
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU Info
    let cpu_info = sys
        .processors()
        .get(0)
        .map(|cpu| format!("💻 CPU: {} | Usage: {:.1}%", cpu.brand(), cpu.cpu_usage()));

    // Detailed RAM Info
    let total_ram = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0); // Convert to GB
    let used_ram = (sys.total_memory() - sys.available_memory()) as f64 / (1024.0 * 1024.0 * 1024.0);
    let ram_percent = (used_ram / total_ram) * 100.0;
    let ram_info = format!(
        "🛠️ RAM: {:.2}/{:.2}GB used ({:.1}%)\n\
         📊 Max RAM Capacity: {:.2}GB",
        used_ram, total_ram, ram_percent,
        sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0)
    );

    // Detailed Disk Info
    let mut disk_info = String::new();
    for disk in sys.disks() {
        let total_space = disk.total_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let available_space = disk.available_space() as f64 / (1024.0 * 1024.0 * 1024.0);
        let used_space = total_space - available_space;
        let disk_percent = (used_space / total_space) * 100.0;
        
        disk_info.push_str(&format!(
            "\n💾 Disk [{}]: {:.2}/{:.2}GB used ({:.1}%)\n\
             📊 Total Capacity: {:.2}GB | Free: {:.2}GB",
            disk.name().to_string_lossy(),
            used_space, total_space, disk_percent,
            total_space, available_space
        ));
    }

    // Uptime
    let uptime = sys.uptime();
    let hours = uptime / 3600;
    let minutes = (uptime % 3600) / 60;
    let uptime_info = format!("⏳ Uptime: {}h {}m", hours, minutes);

    // Battery Health
    let battery_health = get_battery_health().unwrap_or("🔋 Battery: Not Detected".to_string());

    // Graphics Info
    let graphics_info = get_graphics_info(&sys);

    // Display Everything
    println!("\n🔍 System Health Report:");
    if let Some(cpu) = cpu_info {
        println!("{}", cpu);
    }
    println!("{}", ram_info);
    println!("{}", disk_info);
    println!("{}", graphics_info);
    println!("{}", uptime_info);
    println!("{}", battery_health);
}