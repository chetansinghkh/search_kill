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

fn main() {
    println!("搜索想要结束的进程关键字,软件会列出编号和详细信息,你可以选择结束进程!");
    print!("请输入关键字: ");
    // 确保提示先被输出（stdout 可能会被缓冲），否则在某些运行环境下会等到读入完成或者程序退出才看到提示
    io::stdout().flush().expect("刷新 stdout 失败");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("读取输入失败");
    let keyword = input.trim();
    let processes = find_processes_by_keyword(keyword);
    for (pid, info) in &processes {
        println!("pid: {}, Info: {}", pid, info);
    }

    if (&processes).is_empty() {
        println!("没有找到进程");
        return;
    }

    println!("请输入要结束的进程编号: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    if input.is_empty() {
        return;
    }
    let pids: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap_or(-1))
        .collect();
    if pids.is_empty() {
        return;
    }
    //解析出无效和存在的pid
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
        println!("无效的编号: {:?}", invalid_pids);
    }
    if valid_pids.is_empty() {
        println!("没有有效的编号");
    }
    for pid in &valid_pids {
        println!("结束进程: {}", pid);
        Command::new("taskkill")
            .args(&["/F", "/PID", pid.to_string().as_str()])
            .output()
            .expect("failed to execute process");
    }
    println!("结束进程完成");
}
