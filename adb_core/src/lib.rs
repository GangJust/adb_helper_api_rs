pub mod entity;
pub mod expansion;
pub mod utils;

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use entity::{Activity, AppDesc, Device, FileDesc, Screenshot, Version};
use utils::ShellUtils;

/// 获取adb版本
pub fn version() -> Version {
    let args = vec!["version"];
    let content = ShellUtils::shell_to_string("adb", args);
    Version::parse(content)
}

/// 获取附加设备列表
pub fn get_devices() -> Vec<Device> {
    let args = vec!["devices", "-l"];
    let content = ShellUtils::shell_to_string("adb", args);
    let mut splits = content.trim().split("\n");

    splits.next(); //跳过第一个 `List of devices attached`

    let prop_call = |serial_no: &String| {
        //获取设备属性
        let args = vec!["-s", serial_no, "shell", "getprop"];
        ShellUtils::shell_to_string("adb", args)
    };

    return splits
        .map(|item_str| {
            //解析设备信息
            Device::parse(item_str.to_string(), prop_call)
        })
        .collect::<Vec<Device>>();
}

/// 获取活动信息
pub fn get_activity(device: &Device) -> Activity {
    let args = vec![
        "-s",
        &device.serial_no,
        "shell",
        "dumpsys",
        "activity",
        "activities",
    ];
    let content = ShellUtils::shell_to_string("adb", args);
    Activity::parse(content)
}

/// 获取包名列表
/// is_system 是否系统应用
pub fn get_packages(device: &Device, is_system: bool) -> Vec<String> {
    let args = if is_system {
        vec![
            "-s",
            &device.serial_no,
            "shell",
            "pm",
            "list",
            "packages",
            "-s",
        ]
    } else {
        vec![
            "-s",
            &device.serial_no,
            "shell",
            "pm",
            "list",
            "packages",
            "-3",
        ]
    };

    let content = ShellUtils::shell_to_string("adb", args);
    content
        .split("\n")
        .map(|item| item.trim().replace("package:", ""))
        .filter(|item| !item.is_empty())
        .collect::<Vec<String>>()
}

/// 通过包名获取App信息
/// package_name 包名
pub fn get_app_desc<T: AsRef<str>>(device: &Device, package_name: &T) -> AppDesc {
    let package_name = package_name.as_ref();

    //基本信息
    let args = vec![
        "-s",
        &device.serial_no,
        "shell",
        "dumpsys",
        "package",
        package_name,
    ];
    let content = ShellUtils::shell_to_string("adb", args);

    //安装路径
    let args = vec!["-s", &device.serial_no, "shell", "pm", "path", package_name];
    let install_path_result = ShellUtils::shell_to_string("adb", args);
    let install_path = install_path_result.trim().replace("package:", "");

    //文件大小
    let args = vec![
        "-s",
        &device.serial_no,
        "shell",
        "du",
        "-h",
        install_path.as_str(),
    ];
    let size_result = ShellUtils::shell_to_string("adb", args);
    let splits = size_result.split_whitespace().collect::<Vec<&str>>();
    let size = splits.first().unwrap_or(&"0").to_string();

    AppDesc::parse(content, package_name.to_string(), install_path, size)
}

/// 获取文件列表
/// path 路径
pub fn list_files(device: &Device, path: &str) -> Vec<FileDesc> {
    let safe_path = format!("'{}'", &path); //防止路径中出现空格，外部包裹一个引号
    let args = vec!["-s", &device.serial_no, "shell", "ls", "-lai", &safe_path];
    let content = ShellUtils::shell_to_string("adb", args);
    FileDesc::parse(&content, &path)
}

/// 推送文件
pub fn push_file(device: &Device, local_path: &str, remote_path: &str) -> String {
    // let safe_local_path = format!("'{}'", local_path); //防止路径中出现空格，外部包裹一个引号
    // let safe_remote_path = format!("'{}'", remote_path); //防止路径中出现空格，外部包裹一个引号
    let args = vec![
        "-s",
        &device.serial_no,
        "push",
        // &safe_local_path,
        // &safe_remote_path,
        local_path,
        remote_path,
    ];
    ShellUtils::shell_to_string("adb", args)
}

/// 拉取文件
pub fn pull_file(device: &Device, remote_path: &str, local_path: &str) -> String {
    // let safe_local_path = format!("'{}'", local_path); //防止路径中出现空格，外部包裹一个引号
    // let safe_remote_path = format!("'{}'", remote_path); //防止路径中出现空格，外部包裹一个引号
    let args = vec![
        "-s",
        &device.serial_no,
        "pull",
        // &safe_remote_path,
        // &safe_local_path,
        remote_path,
        local_path,
    ];
    ShellUtils::shell_to_string("adb", args)
}

/// 安装apk
/// apk_path apk路径
pub fn install_apk(device: &Device, apk_path: &str) -> String {
    let args = vec!["-s", &device.serial_no, "install", "-r", apk_path];
    ShellUtils::shell_to_string("adb", args)
    /* let output = Command::new("adb")
        .arg("install")
        .arg("-r")
        .arg(apk_path)
        .stdout(Stdio::piped()) // Redirect output
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            let stderr = String::from_utf8_lossy(&out.stderr).to_string();
            stderr + &stdout // Combine stdout and stderr
        }
        Err(err) => err.to_string(),
    } */
}

/// 卸载apk
/// package_name 包名
pub fn uninstall_apk(device: &Device, package_name: &str) -> String {
    let args = vec!["-s", &device.serial_no, "uninstall", package_name];
    ShellUtils::shell_to_string("adb", args)
}

/// 截图 (返回二进制数据流)
pub fn screenshot(device: &Device) -> Screenshot {
    let args = vec!["-s", &device.serial_no, "exec-out", "screencap", "-p"];
    let result = ShellUtils::shell("adb", args);
    match result {
        Err(_) => Screenshot {
            mimetype: "".to_string(),
            width: 0,
            height: 0,
            data: Vec::new(),
        },
        Ok(data) => {
            let data = data.stdout;
            match imageinfo::ImageInfo::from_raw_data(&data) {
                Err(_) => Screenshot {
                    mimetype: "".to_string(),
                    width: 0,
                    height: 0,
                    data: Vec::new(),
                },
                Ok(it) => Screenshot {
                    mimetype: it.mimetype.to_string(),
                    width: it.size.width,
                    height: it.size.height,
                    data,
                },
            }
        }
    }
}

/// 获取当前布局信息
pub fn get_layout(device: &Device) -> String {
    let args = vec![
        "-s",
        &device.serial_no,
        "exec-out",
        "uiautomator",
        "dump",
        "/dev/tty",
    ];
    ShellUtils::shell_to_string("adb", args)
}

/// 执行shell命令
/// cmd 命令
/// su 是否使用su模式 (root)
pub fn shell(device: &Device, cmd: &str, su: bool) -> String {
    let args = if su {
        vec!["-s", &device.serial_no, "shell", "su", "-c", cmd]
    } else {
        vec!["-s", &device.serial_no, "shell", cmd]
    };
    ShellUtils::shell_to_string("adb", args)
}

/// logcat forward
/// port 端口
pub fn logcat(device: &Device, port: i32) {
    let bind_ip = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&bind_ip)
        .expect(format!("Could not bind to address: {}", &bind_ip).as_str());

    //listener once
    let (mut stream, addr) = listener.accept().expect("Connection failed.");
    println!("Connection established: {}", addr);

    //forwards the logcat output to the client
    let args = vec!["-s", &device.serial_no, "logcat"];
    let mut child: std::process::Child = ShellUtils::shell_spawn("adb", args);

    match child.stdout.take() {
        None => {}
        Some(stdout) => {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    // dbg!(line.as_str());
                    match stream.write_all(line.as_bytes()) {
                        Err(_) => break,
                        Ok(_) => {
                            stream.write_all(b"\n").unwrap();
                            stream.flush().unwrap();
                        }
                    }
                }
            }
        }
    }

    //connection closed and kill the child process
    if let Ok(()) = child.kill() {
        println!("Child process killed.");
    }
}
