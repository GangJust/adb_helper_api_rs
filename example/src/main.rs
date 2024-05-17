fn main() {
    // let result = adb_core::version();
    // dbg!("{:?}", &result);

    let devices = adb_core::get_devices();
    // dbg!("{:?}", &devices);

    let first_device = devices.first().expect("No device attached");
    // let activity = adb_core::get_activity(first_device);
    // dbg!("{:?}", &activity);

    // let packages = adb_core::get_packages(first_device, false);
    // dbg!("{:?}", &packages);

    // let desc = adb_core::get_app_desc(first_device, &"com.coolapk.market");
    // dbg!("{:?}", &desc);

    // let files = adb_core::list_files(first_device, "/storage/emulated/0/新建_文件 夹/");
    // let files = adb_core::list_files(first_device, "/");
    // println!("{}", serde_json::to_string_pretty(&files).unwrap());

    // let local_path = "C:\\Users\\HMS\\Desktop\\测试 空格\\com.coolapk.market.apk";
    // let remote_path = "/storage/emulated/0/新建_文件 夹/com.coolapk.market.apk";
    // let result = adb_core::push_file(&first_device, local_path, remote_path);
    // dbg!("{:?}", &result);

    // let remote_path = "/storage/emulated/0/新建_文件 夹/com.coolapk.market.apk";
    // let local_path = "C:\\Users\\HMS\\Desktop\\测试 空格\\com.coolapk.market.apk";
    // let result = adb_core::pull_file(&first_device, remote_path, local_path);
    // dbg!("{:?}", &result);

    // let apk_path = "C:/Users/HMS/Desktop/测试 空格/com.coolapk.market.apk";
    // let result = adb_core::install_apk(&first_device, apk_path);
    // dbg!("{:?}", &result);

    // let result = adb_core::shell(
    //     &first_device,
    //     // r"ls -lai '/storage/emulated/0/新建_文件\ 夹/'",
    //     r"chmod 777 /data/local/tmp/test",
    //     true,
    // );
    // println!("{}", &result);

    // let result = adb_core::uninstall_apk(&first_device, "com.coolapk.market");
    // dbg!("{:?}", &result);

    // let data = adb_core::screenshot(&first_device);
    // println!("{:?}", data);

    // let layout = adb_core::get_layout(&first_device);
    // println!("{}", layout);

    // adb_core::logcat(&first_device, 5656); //and then run `cargo run -p logcat-client`
}
