# System Health

A command-line tool to display comprehensive system health information including battery status, RAM usage, and system resources.

## Features

- 🔋 Detailed battery health information (health percentage, cycle count, capacity)
- 💾 RAM usage and capacity monitoring
- 💻 CPU usage and information
- 🖴 Disk usage and storage information
- 🎮 Basic graphics card information
- ⏳ System uptime

## Installation

```bash
cargo install system_health
```

## Usage

Simply run the command:

```bash
system_health
```

### Example Output

```
🔍 System Health Report:
💻 CPU: Intel(R) Core(TM) i7-9750H | Usage: 25.5%
🛠️ RAM: 8.5/16.0GB used (53.1%)
📊 Max RAM Capacity: 16.0GB

💾 Disk [/dev/sda]: 256.5/512.0GB used (50.1%)
📊 Total Capacity: 512.0GB | Free: 255.5GB

🎮 Graphics: NVIDIA GeForce RTX 2060 | Temp: 45.2°C
⏳ Uptime: 2h 15m

🔋 Battery Health: 95.2% (42 cycles)
📉 Current Max Capacity: 56.8Wh
🏭 Design Capacity: 59.7Wh
```

## Requirements

- Rust 1.70 or higher
- Linux, macOS, or Windows

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 