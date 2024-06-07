use adb::entity::{Device, DeviceProp};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};

use crate::helper::{get_result, ArrayList};

/// 获取附加设备列表
#[no_mangle]
pub unsafe extern "C" fn Java_adb_Adb_getDevices<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
) -> JObject<'local> {
    let devices = adb::get_devices();

    // 遍历设备列表, 构造ArrayList
    let device_list = ArrayList::new(&mut env);
    for ele in &devices {
        let device = new_device(&mut env, &ele);
        ArrayList::add(&mut env, &device_list, &device);
    }

    return device_list;
}

// 构建设备对象
fn new_device<'local>(env: &mut JNIEnv<'local>, device: &Device) -> JObject<'local> {
    let serial_no = env.new_string(&device.serial_no).unwrap();
    let state = env.new_string(&device.state).unwrap();
    let product = env.new_string(&device.product).unwrap();
    let model = env.new_string(&device.model).unwrap();
    let device_name = env.new_string(&device.device).unwrap();
    let transport_id = env.new_string(&device.transport_id).unwrap();

    // 遍历属性, 构造ArrayList
    let prop_list = ArrayList::new(env);
    for ele in &device.props {
        let prop = new_device_prop(env, &ele);
        ArrayList::add(env, &prop_list, &prop);
    }

    let args = [
        JValue::from(&serial_no),
        JValue::from(&state),
        JValue::from(&product),
        JValue::from(&model),
        JValue::from(&device_name),
        JValue::from(&transport_id),
        JValue::from(&prop_list),
    ];

    let result = env.new_object(
        "adb/entity/Device",
        "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/List;)V",
        &args,
    );

    get_result(result)
}

// 构建设备属性对象
fn new_device_prop<'local>(env: &mut JNIEnv<'local>, device_prop: &DeviceProp) -> JObject<'local> {
    let key = env.new_string(&device_prop.name).unwrap();
    let value = env.new_string(&device_prop.value).unwrap();

    let args = [JValue::from(&key), JValue::from(&value)];

    let result = env.new_object(
        "adb/entity/DeviceProp",
        "(Ljava/lang/String;Ljava/lang/String;)V",
        &args,
    );

    get_result(result)
}
