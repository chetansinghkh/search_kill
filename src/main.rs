use std::io::{self, Write};
use std::process::Command;
use std::str;

#[cfg(target_os = "windows")]
fn find_processes_by_keyword(keyword: &str) -> Vec<(i32, String)> {
    let output = Command::new("cmd")
        .args(&["/C", "tasklist"])
        .output()
        .expect("failed to execute process");

    if output.status.success() == false {
        eprintln!("Failed to retrieve process list");
        return vec![];
    }

    let stdout = str::from_utf8(&output.stdout).expect("failed to convert stdout to string");
    let mut result = Vec::new();
    for line in stdout.lines().skip(3) {
        if line.contains(keyword) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let pid = parts[1].parse::<i32>().unwrap_or(-1);
                let process_name = parts[0].to_string();
                result.push((pid, process_name));
            }
        }
    }
    return result;
}

#[cfg(target_os = "windows")]
fn set_console_encoding() {
    use std::process::Command;
    // Set console encoding to UTF-8
    Command::new("cmd")
        .args(&["/C", "chcp", "65001"])
        .output()
        .expect("Failed to set console encoding");
}

fn main() {
    set_console_encoding();
    println!("Search for processes by keyword. The program will list PIDs and details. You can choose to terminate processes!");
    print!("Enter keyword: ");
    // Ensure prompt is output first (stdout may be buffered)
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read input");
    let keyword = input.trim();
    let processes = find_processes_by_keyword(keyword);
    for (pid, info) in &processes {
        println!("pid: {}, Info: {}", pid, info);
    }

    if (&processes).is_empty() {
        println!("No processes found");
        return;
    }

    println!("Enter process PID(s) to terminate (enter 'all' to terminate all matching processes): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if input.is_empty() {
        return;
    }
    let input_trimmed = input.trim();
    
    // Check if "all" was entered
    let valid_pids: Vec<i32> = if input_trimmed.eq_ignore_ascii_case("all") {
        // If "all" is entered, use all found process PIDs
        processes.iter().map(|(pid, _)| *pid).collect()
    } else {
        // Otherwise parse the entered PIDs
        let pids: Vec<i32> = input_trimmed
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap_or(-1))
            .collect();
        if pids.is_empty() {
            return;
        }
        // Separate valid and invalid PIDs
        let mut valid_pids = Vec::new();
        let mut invalid_pids = Vec::new();
        for pid in pids {
            if pid == -1 {
                invalid_pids.push(pid);
                continue;
            }
            if processes.iter().any(|(p, _)| *p == pid) {
                valid_pids.push(pid);
            } else {
                invalid_pids.push(pid);
            }
        }
        if !invalid_pids.is_empty() {
            println!("Invalid PID(s): {:?}", invalid_pids);
        }
        valid_pids
    };
    
    if valid_pids.is_empty() {
        println!("No valid PIDs");
        return;
    }
    for pid in &valid_pids {
        println!("Terminating process: {}", pid);
        Command::new("taskkill")
            .args(&["/F", "/PID", pid.to_string().as_str()])
            .output()
            .expect("failed to execute process");
    }
    println!("Process termination completed");
}
