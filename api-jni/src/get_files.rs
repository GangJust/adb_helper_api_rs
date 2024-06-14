use adb::entity::FileDesc;
use jni::{
    objects::{JObject, JString, JValue},
    JNIEnv,
};

use crate::helper::{get_result, get_string, JArrayList};

/// 获取文件列表
#[no_mangle]
pub extern "C" fn Java_adb_Adb_getFiles<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    path: JString<'local>,
) -> JObject<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let path = get_string(&mut env, &path);

    let files = adb::list_files(&serial_no, &path);

    let list = JArrayList::new(&mut env);
    for ele in files {
        let file_desc = new_file_desc(&mut env, &ele);
        JArrayList::add(&mut env, &list, &file_desc);
    }

    list
}

fn new_file_desc<'local>(env: &mut JNIEnv<'local>, desc: &FileDesc) -> JObject<'local> {
    let inode = env.new_string(&desc.inode).unwrap();
    let permission_str = env.new_string(&desc.permission_str).unwrap();
    let link_count = env.new_string(&desc.link_count).unwrap();
    let owner = env.new_string(&desc.owner).unwrap();
    let group = env.new_string(&desc.group).unwrap();
    let size = env.new_string(&desc.size).unwrap();
    let datetime = env.new_string(&desc.datetime).unwrap();
    let name = env.new_string(&desc.name).unwrap();
    let kind = env.new_string(&desc.kind).unwrap();
    let path = env.new_string(&desc.path).unwrap();

    let args = vec![
        JValue::from(&inode),
        JValue::from(&permission_str),
        JValue::from(&link_count),
        JValue::from(&owner),
        JValue::from(&group),
        JValue::from(&size),
        JValue::from(&datetime),
        JValue::from(&name),
        JValue::from(&kind),
        JValue::from(&path),
    ];

    let result = env.new_object(
        "adb/entity/FileDesc", 
    "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", 
    &args
    );

    get_result(result)
}

/// 获取文件类型
#[no_mangle]
pub extern "C" fn Java_adb_Adb_getFileKind<'local>(
    mut env: JNIEnv<'local>,
    _thiz: JObject<'local>,
    serial_no: JString<'local>,
    path: JString<'local>,
) -> JString<'local> {
    let serial_no = get_string(&mut env, &serial_no);
    let path = get_string(&mut env, &path);

    let result = adb::get_file_kind(&serial_no, &path);

    env.new_string(&result).unwrap()
}
