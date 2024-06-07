use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};

use crate::helper::get_result;

/// 获取adb版本
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_version<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
) -> JObject<'local> {
    let adb_version = adb::version();

    let version_name = env.new_string(adb_version.version_name).unwrap();
    let version_code = env.new_string(adb_version.version_code).unwrap();
    let install_path = env.new_string(adb_version.install_path).unwrap();
    let running_os = env.new_string(adb_version.running_os).unwrap();

    let args = [
        JValue::from(&version_name),
        JValue::from(&version_code),
        JValue::from(&install_path),
        JValue::from(&running_os),
    ];

    let result = env.new_object(
        "adb/entity/Version",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
        &args,
    );

    get_result(result)
}
