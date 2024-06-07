use jni::{
    objects::{JObject, JString},
    JNIEnv,
};

use crate::helper::get_string;

/// 获取布局
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_getLayout<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);

    let result = adb::get_layout(&serial_no);

    env.new_string(&result).unwrap()
}
