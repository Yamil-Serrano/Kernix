mod service_setter;
use service_setter::set_service;

fn main() {
    // List services
    let services_to_disable = [
        "DiagTrack",            // Telemetry (Connected User Experiences and Telemetry)
        "dmwappushservice",     // Telemetry (Device Management Wireless Application Protocol)
        "TrkWks",               // Distributed Link Tracking Client
        "Fax",                  // Fax Service
        "MapsBroker",           // Downloaded Maps Manager
        "lfsvc",                // Geolocation Service
        "XblAuthManager",       // Xbox Live Authentication Manager
        "XblGameSave",          // Xbox Live Game Save
        "CDPUserSvc",           // Connected Devices Platform User Service
        "SSDPSRV",              // SSDP Discovery Service
        "upnphost",             // UPnP Device Host Service
        "Windows Insider Service", // Windows Insider Service
        "Enterprise App Management Service", // Enterprise App Management Service
        "AppReadiness",      // Optional! Some games might use it. Uncomment if you're sure.
    ];

    // true = Enable, false = Disable
    let enable = false;

    for s in services_to_disable {
        set_service(s, enable);
    }
}
