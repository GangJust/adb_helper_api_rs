use jni::{
    objects::{JObject, JString, JValue},
    JNIEnv,
};

use crate::helper::{get_result, get_string};

/// 屏幕截图
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_screenshot<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
) -> JObject<'local> {
    let serial_no = get_string(&mut env, &serial_no);

    let result = adb::screenshot(&serial_no);

    let mimetype = env.new_string(&result.mimetype).unwrap();
    let width = result.width;
    let height = result.height;
    let data = env.byte_array_from_slice(&result.data).unwrap();

    let args = [
        JValue::from(&mimetype),
        JValue::Long(width),
        JValue::Long(height),
        JValue::from(&data),
    ];

    let result = env.new_object("adb/entity/Screenshot", "(Ljava/lang/String;JJ[B)V", &args);

    get_result(result)
}
