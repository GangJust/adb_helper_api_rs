use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

use crate::helper::get_string;

/// 拉取文件
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_pullFile<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    remote_path: JString<'local>,
    local_path: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let remote_path = get_string(&mut env, &remote_path);
    let local_path = get_string(&mut env, &local_path);

    let result = adb::pull_file(&serial_no, &remote_path, &local_path);

    env.new_string(result).unwrap()
}

/// 推送文件
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_pushFile<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    local_path: JString<'local>,
    remote_path: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let local_path = get_string(&mut env, &local_path);
    let remote_path = get_string(&mut env, &remote_path);

    let result = adb::push_file(&serial_no, &local_path, &remote_path);

    env.new_string(result).unwrap()
}
