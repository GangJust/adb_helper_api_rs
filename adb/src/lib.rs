pub mod entity;
pub mod expansion;
pub mod utils;

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use entity::{Activity, AppDesc, Device, FileDesc, Screenshot, Version};
use utils::ShellUtils;

/// 启动adb服务
pub fn start_server() -> String {
    let args = vec!["start-server"];
    ShellUtils::shell_to_string("adb", args)
}

/// 停止adb服务
pub fn kill_server() -> String {
    let args = vec!["kill-server"];
    ShellUtils::shell_to_string("adb", args)
}

/// 获取adb版本
pub fn version() -> Version {
    let args = vec!["version"];
    let content = ShellUtils::shell_to_string("adb", args);
    Version::parse(content)
}

/// 连接设备
pub fn connect<T: AsRef<str>>(ip: &T) -> String {
    let args = vec!["connect", ip.as_ref()];
    ShellUtils::shell_to_string("adb", args)
}

/// 获取附加设备列表
pub fn get_devices() -> Vec<Device> {
    let args = vec!["devices", "-l"];
    let content = ShellUtils::shell_to_string("adb", args);
    let mut splits = content.trim().split("\n");

    let first = splits.next().unwrap(); //跳过 `List of devices attached`

    if first.starts_with("* daemon not running;") {
        //如果是首次运行，需要跳过三行
        splits.next().unwrap(); //跳过 `* daemon not running; starting now at tcp:5037`
        splits.next().unwrap(); //跳过 `* daemon started successfully`
    }

    //获取设备属性
    let prop_call = |serial_no: &String| {
        let args = vec!["-s", serial_no, "shell", "getprop"];
        ShellUtils::shell_to_string("adb", args)
    };

    return splits
        .map(|item_str| {
            //解析设备信息
            Device::parse(item_str.to_string(), prop_call)
        })
        .filter(|item| !item.serial_no.is_empty())
        .collect::<Vec<Device>>();
}

/// 获取活动信息
pub fn get_activity<T: AsRef<str>>(serial_no: &T) -> Activity {
    let args = vec![
        "-s",
        serial_no.as_ref(),
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
pub fn get_packages<T: AsRef<str>>(serial_no: &T, is_system: bool) -> Vec<String> {
    let args = if is_system {
        vec![
            "-s",
            serial_no.as_ref(),
            "shell",
            "pm",
            "list",
            "packages",
            "-s",
        ]
    } else {
        vec![
            "-s",
            serial_no.as_ref(),
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
pub fn get_app_desc<T: AsRef<str>>(serial_no: &T, package_name: &str) -> AppDesc {
    let package_name = package_name.as_ref();

    //基本信息
    let args = vec![
        "-s",
        serial_no.as_ref(),
        "shell",
        "dumpsys",
        "package",
        package_name,
    ];
    let content = ShellUtils::shell_to_string("adb", args);

    //安装路径
    let args = vec![
        "-s",
        serial_no.as_ref(),
        "shell",
        "pm",
        "path",
        package_name,
    ];
    let install_path_result = ShellUtils::shell_to_string("adb", args);
    let install_path = install_path_result.trim().replace("package:", "");

    //文件大小
    let args = vec![
        "-s",
        serial_no.as_ref(),
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
pub fn list_files<T: AsRef<str>>(serial_no: &T, path: &str) -> Vec<FileDesc> {
    let safe_path = format!("'{}'", &path); //防止路径中出现空格，外部包裹一个引号
    let args = vec!["-s", serial_no.as_ref(), "shell", "ls", "-lai", &safe_path];
    let content = ShellUtils::shell_to_string("adb", args);
    FileDesc::parse(&content, &path)
}

/// 获取文件类型
pub fn get_file_kind<T: AsRef<str>>(serial_no: &T, path: &str) -> String {
    let safe_path = format!("'{}'", &path); //防止路径中出现空格，外部包裹一个引号
    let args = vec!["-s", serial_no.as_ref(), "shell", "file", "-L", &safe_path];
    let content = ShellUtils::shell_to_string("adb", args);
    match content.trim().ends_with("directory") {
        true => "directory".to_string(),
        false => "file".to_string(), //其他情况都认为是文件
    }
}

/// 推送文件
pub fn push_file<T: AsRef<str>>(serial_no: &T, local_path: &str, remote_path: &str) -> String {
    // let safe_local_path = format!("'{}'", local_path); //防止路径中出现空格，外部包裹一个引号
    // let safe_remote_path = format!("'{}'", remote_path); //防止路径中出现空格，外部包裹一个引号
    let args = vec![
        "-s",
        serial_no.as_ref(),
        "push",
        // &safe_local_path,
        // &safe_remote_path,
        local_path,
        remote_path,
    ];
    ShellUtils::shell_to_string("adb", args)
}

/// 拉取文件
pub fn pull_file<T: AsRef<str>>(serial_no: &T, remote_path: &str, local_path: &str) -> String {
    // let safe_local_path = format!("'{}'", local_path); //防止路径中出现空格，外部包裹一个引号
    // let safe_remote_path = format!("'{}'", remote_path); //防止路径中出现空格，外部包裹一个引号
    let args = vec![
        "-s",
        serial_no.as_ref(),
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
pub fn install_apk<T: AsRef<str>>(serial_no: &T, apk_path: &str) -> String {
    let args = vec!["-s", serial_no.as_ref(), "install", "-r", apk_path];
    ShellUtils::shell_to_string("adb", args)
}

/// 卸载apk
/// package_name 包名
pub fn uninstall_apk<T: AsRef<str>>(serial_no: &T, package_name: &str) -> String {
    let args = vec!["-s", serial_no.as_ref(), "uninstall", package_name];
    ShellUtils::shell_to_string("adb", args)
}

/// 截图 (返回二进制数据流)
pub fn screenshot<T: AsRef<str>>(serial_no: &T) -> Screenshot {
    let args = vec!["-s", serial_no.as_ref(), "exec-out", "screencap", "-p"];
    let result = ShellUtils::shell("adb", args);

    let none = Screenshot {
        mimetype: "".to_string(),
        width: 0,
        height: 0,
        data: Vec::new(),
    };

    match result {
        Err(_) => none,
        Ok(out) => {
            let data = out.stdout;
            let imgae_info = imageinfo::ImageInfo::from_raw_data(&data);
            match imgae_info {
                Err(_) => none,
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
pub fn get_layout<T: AsRef<str>>(serial_no: &T) -> String {
    let args = vec![
        "-s",
        serial_no.as_ref(),
        "exec-out",
        "uiautomator",
        "dump",
        "/dev/tty",
    ];
    ShellUtils::shell_to_string("adb", args)
}

/// 同步执行shell命令
/// cmd 命令
/// su 是否使用su模式 (root)
pub fn shell_syn<T: AsRef<str>>(serial_no: &T, cmd: &str, su: bool) -> String {
    let args = if su {
        vec!["-s", serial_no.as_ref(), "shell", "su", "-c", cmd]
    } else {
        vec!["-s", serial_no.as_ref(), "shell", cmd]
    };
    ShellUtils::shell_to_string("adb", args)
}

/// logcat forward
/// port 端口
pub fn logcat<T: AsRef<str>>(serial_no: &T, port: i32) {
    let bind_ip = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&bind_ip)
        .expect(format!("Could not bind to address: {}", &bind_ip).as_str());

    //listener once
    let (mut stream, addr) = listener.accept().expect("Connection failed.");
    println!("Connection established: {}", addr);

    //forwards the logcat output to the client
    let args = vec!["-s", serial_no.as_ref(), "logcat"];
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
