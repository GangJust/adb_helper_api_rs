use jni::{
    objects::{JObject, JString, JValue},
    sys::jboolean,
    JNIEnv,
};

use crate::helper::{get_result, get_string, ArrayList};

/// 获取包名列表
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_getPackages<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    is_system: jboolean,
) -> JObject<'local> {
    let serial_no = get_string(&mut env, &serial_no);

    let packages = adb::get_packages(&serial_no, is_system == 1);

    let array_list = ArrayList::new(&mut env);
    for ele in &packages {
        let package = env.new_string(ele).unwrap();
        ArrayList::add(&mut env, &array_list, &package);
    }

    array_list
}

/// 通过包名获取App信息
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_getAppDesc<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    package_name: JString<'local>,
) -> JObject<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let package_name = get_string(&mut env, &package_name);

    let app_desc = adb::get_app_desc(&serial_no, &package_name);

    let package_name = env.new_string(&app_desc.package_name).unwrap();
    let launcher_activity = env.new_string(&app_desc.launcher_activity).unwrap();
    let primary_cpu_abi = env.new_string(&app_desc.primary_cpu_abi).unwrap();
    let version_name = env.new_string(&app_desc.version_name).unwrap();
    let version_code = env.new_string(&app_desc.version_code).unwrap();
    let min_sdk = env.new_string(&app_desc.min_sdk).unwrap();
    let target_sdk = env.new_string(&app_desc.target_sdk).unwrap();
    let time_stamp = env.new_string(&app_desc.time_stamp).unwrap();
    let first_install_time = env.new_string(&app_desc.first_install_time).unwrap();
    let last_update_time = env.new_string(&app_desc.last_update_time).unwrap();
    let sign_version = env.new_string(&app_desc.sign_version).unwrap();
    let data_dir = env.new_string(&app_desc.data_dir).unwrap();
    let external_data_dir = env.new_string(&app_desc.external_data_dir).unwrap();
    let install_path = env.new_string(&app_desc.install_path).unwrap();
    let size = env.new_string(&app_desc.size).unwrap();
    let is_system = app_desc.is_system;

    let args = [
        JValue::from(&package_name),
        JValue::from(&launcher_activity),
        JValue::from(&primary_cpu_abi),
        JValue::from(&version_name),
        JValue::from(&version_code),
        JValue::from(&min_sdk),
        JValue::from(&target_sdk),
        JValue::from(&time_stamp),
        JValue::from(&first_install_time),
        JValue::from(&last_update_time),
        JValue::from(&sign_version),
        JValue::from(&data_dir),
        JValue::from(&external_data_dir),
        JValue::from(&install_path),
        JValue::from(&size),
        JValue::from(is_system),
    ];

    let result = env.new_object(
        "adb/entity/AppDesc",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Z)V",
        &args,
    );

    get_result(result)
}
