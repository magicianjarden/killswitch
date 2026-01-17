use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize, Clone)]
pub struct PortInfo {
    pub pid: u32,
    pub process_name: String,
    pub port: u16,
    pub protocol: String,
    pub state: String,
    pub local_address: String,
}

#[tauri::command]
pub fn get_ports() -> Result<Vec<PortInfo>, String> {
    #[cfg(target_os = "macos")]
    {
        get_ports_macos()
    }
    #[cfg(target_os = "windows")]
    {
        get_ports_windows()
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}

#[cfg(target_os = "macos")]
fn get_ports_macos() -> Result<Vec<PortInfo>, String> {
    let output = Command::new("lsof")
        .args(["-i", "-P", "-n"])
        .output()
        .map_err(|e| format!("Failed to execute lsof: {}", e))?;

    if !output.status.success() {
        return Err("lsof command failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut ports: Vec<PortInfo> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        let process_name = parts[0].to_string();
        let pid: u32 = match parts[1].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let protocol = parts[7].to_string();
        let name_field = parts[8];

        // Parse address and port
        let (local_address, port) = if let Some(arrow_pos) = name_field.find("->") {
            let local_part = &name_field[..arrow_pos];
            parse_address_port(local_part)
        } else {
            parse_address_port(name_field)
        };

        let state = if parts.len() > 9 {
            parts[9].trim_matches(|c| c == '(' || c == ')').to_string()
        } else {
            "".to_string()
        };

        // Create unique key to avoid duplicates
        let key = format!("{}:{}:{}", pid, port, protocol);
        if seen.contains(&key) || port == 0 {
            continue;
        }
        seen.insert(key);

        ports.push(PortInfo {
            pid,
            process_name,
            port,
            protocol,
            state,
            local_address,
        });
    }

    ports.sort_by(|a, b| a.port.cmp(&b.port));
    Ok(ports)
}

#[cfg(target_os = "macos")]
fn parse_address_port(addr: &str) -> (String, u16) {
    if let Some(colon_pos) = addr.rfind(':') {
        let address = addr[..colon_pos].to_string();
        let port_str = &addr[colon_pos + 1..];
        let port: u16 = port_str.parse().unwrap_or(0);
        (address, port)
    } else {
        (addr.to_string(), 0)
    }
}

#[cfg(target_os = "windows")]
fn get_ports_windows() -> Result<Vec<PortInfo>, String> {
    // Get netstat output
    let netstat_output = Command::new("netstat")
        .args(["-ano"])
        .output()
        .map_err(|e| format!("Failed to execute netstat: {}", e))?;

    if !netstat_output.status.success() {
        return Err("netstat command failed".to_string());
    }

    // Get process names via tasklist
    let tasklist_output = Command::new("tasklist")
        .args(["/FO", "CSV", "/NH"])
        .output()
        .map_err(|e| format!("Failed to execute tasklist: {}", e))?;

    let mut pid_to_name: std::collections::HashMap<u32, String> = std::collections::HashMap::new();

    if tasklist_output.status.success() {
        let tasklist_str = String::from_utf8_lossy(&tasklist_output.stdout);
        for line in tasklist_str.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 2 {
                let name = parts[0].trim_matches('"').to_string();
                if let Ok(pid) = parts[1].trim_matches('"').parse::<u32>() {
                    pid_to_name.insert(pid, name);
                }
            }
        }
    }

    let netstat_str = String::from_utf8_lossy(&netstat_output.stdout);
    let mut ports: Vec<PortInfo> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for line in netstat_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }

        let protocol = parts[0].to_uppercase();
        if protocol != "TCP" && protocol != "UDP" {
            continue;
        }

        let local_addr = parts[1];
        let state = if protocol == "TCP" && parts.len() > 3 {
            parts[3].to_string()
        } else {
            "".to_string()
        };

        let pid_idx = if protocol == "TCP" { 4 } else { 3 };
        let pid: u32 = if parts.len() > pid_idx {
            parts[pid_idx].parse().unwrap_or(0)
        } else {
            continue;
        };

        let (local_address, port) = parse_windows_address(local_addr);

        let key = format!("{}:{}:{}", pid, port, protocol);
        if seen.contains(&key) || port == 0 {
            continue;
        }
        seen.insert(key);

        let process_name = pid_to_name
            .get(&pid)
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        ports.push(PortInfo {
            pid,
            process_name,
            port,
            protocol,
            state,
            local_address,
        });
    }

    ports.sort_by(|a, b| a.port.cmp(&b.port));
    Ok(ports)
}

#[cfg(target_os = "windows")]
fn parse_windows_address(addr: &str) -> (String, u16) {
    if let Some(colon_pos) = addr.rfind(':') {
        let address = addr[..colon_pos].to_string();
        let port_str = &addr[colon_pos + 1..];
        let port: u16 = port_str.parse().unwrap_or(0);
        (address, port)
    } else {
        (addr.to_string(), 0)
    }
}

#[tauri::command]
pub fn kill_process(pid: u32) -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute kill: {}", e))?;

        if output.status.success() {
            Ok(format!("Process {} terminated", pid))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to kill process {}: {}", pid, stderr))
        }
    }
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("taskkill")
            .args(["/F", "/PID", &pid.to_string()])
            .output()
            .map_err(|e| format!("Failed to execute taskkill: {}", e))?;

        if output.status.success() {
            Ok(format!("Process {} terminated", pid))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to kill process {}: {}", pid, stderr))
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        Err("Unsupported platform".to_string())
    }
}
