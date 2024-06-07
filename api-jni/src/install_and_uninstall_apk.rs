use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

use crate::helper::get_string;

/// 安装apk
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_installApk<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    apk_path: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let apk_path = get_string(&mut env, &apk_path);

    let result = adb::install_apk(&serial_no, &apk_path);

    env.new_string(result).unwrap()
}

/// 卸载apk
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_uninstallApk<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    package_name: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let package_name = get_string(&mut env, &package_name);

    let result = adb::uninstall_apk(&serial_no, &package_name);

    env.new_string(result).unwrap()
}
