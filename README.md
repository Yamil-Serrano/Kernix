# Kernix

**Kernix** is a lightweight Windows service optimizer, designed to safely disable unnecessary services that consume CPU and memory resources. It focuses on improving system performance for developers and gamers without breaking essential functionality.

> **Note:** Kernix only disables services that are generally considered safe to disable. However, always ensure you have a system restore point before making changes. Kernix is designed for Windows 10/11 systems. Results may vary depending on your specific Windows version and configuration.

## Features

- **Safe Service Optimization**: Disables telemetry and non-essential services while preserving critical system functionality
- **Gamer/Developer Focused**: Maintains services needed for gaming, programming, and multimedia

## Currently Supported Services for Disabling

Kernix safely disables the following services:

| Service Name | Description |
|--------------|-------------|
| `DiagTrack` | Connected User Experiences and Telemetry |
| `dmwappushservice` | Device Management Wireless Application Protocol |
| `TrkWks` | Distributed Link Tracking Client |
| `Fax` | Fax Service |
| `MapsBroker` | Downloaded Maps Manager |
| `lfsvc` | Geolocation Service |
| `XblAuthManager` | Xbox Live Authentication Manager |
| `XblGameSave` | Xbox Live Game Save |
| `CDPUserSvc` | Connected Devices Platform User Service |
| `SSDPSRV` | SSDP Discovery Service |
| `upnphost` | UPnP Device Host Service |
| `Windows Insider Service` | Windows Insider Program Service |
| `Enterprise App Management Service` | Enterprise Application Management |

## Services Preserved

Kernix intentionally preserves these essential services:
- Bluetooth services (`BthServ`) - for Bluetooth devices
- Network services - for Wi-Fi and internet connectivity
- Windows Media Player services - for multimedia playback
- Critical system services (Firewall, Security, Updates)

## Installation

1. **Install Rust** (if you haven't already):
   ```bash
   Download from https://rustup.rs/
   Or use winget on Windows:
   winget install Rustlang.Rust.MSVC

## Clone the repository
  ```bash
  git clone https://github.com/Yamil-Serrano/Kernix
  cd Kernix
```

## Run the project
  ```bash
  cargo run
  ```

## Safety Notes

❗Is recommended to always create a system restore point before running system optimization tools

❗Kernix only disables services that are generally safe to disable, but individual system configurations may vary

❗Some games or applications may require specific services: Choose wisely which services and logs to disable

## License
This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.


## Contact
If you have any questions or suggestions, feel free to reach out to me:
GitHub: [Neowizen](https://github.com/Yamil-Serrano)
