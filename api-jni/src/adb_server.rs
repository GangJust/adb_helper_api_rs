use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

/// 启动adb服务
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_startServer<'local>(
    env: JNIEnv<'local>,
    _thiz: JObject<'local>,
) -> JString<'local> {
    let result = adb::start_server();
    env.new_string(&result).unwrap()
}

/// 停止adb服务
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_killServer<'local>(
    env: JNIEnv<'local>,
    _thiz: JObject<'local>,
) -> JString<'local> {
    let result = adb::kill_server();
    env.new_string(&result).unwrap()
}
