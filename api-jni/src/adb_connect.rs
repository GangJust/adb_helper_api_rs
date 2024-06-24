use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

use crate::helper::get_string;

/// 获取adb版本
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_connect<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    ip: JString<'local>,
) -> JString<'local> {
    let ip = get_string(&mut env, &ip);

    let result = adb::connect(&ip);

    env.new_string(result).unwrap()
}
