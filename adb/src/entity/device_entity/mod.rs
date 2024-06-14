use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::RegexUtils;

/// 设备
#[derive(Serialize, Deserialize, Debug)]
pub struct Device {
    //设备序列
    pub serial_no: String,
    //设备状态
    pub state: String,
    //设备产品
    pub product: String,
    //设备型号
    pub model: String,
    //设备型号
    pub device: String,
    //传输ID
    pub transport_id: String,
    //设备参数
    pub props: HashMap<String, String>,
}

impl Device {
    /// 解析设备
    pub fn parse<F>(content: String, call_props_content: F) -> Self
    where
        F: Fn(&String) -> String,
    {
        let regex_str =
            r"(.*?)\s+(.*?)\s+product:(.*?)\s+model:(.*?)\s+device:(.*?)\s+transport_id:(\d+)";
        let cps = RegexUtils::captures(&content, regex_str);
        return match cps {
            Some(it) => {
                let serial_no = RegexUtils::captures_value(&it, 1);
                let state = RegexUtils::captures_value(&it, 2);
                let product = RegexUtils::captures_value(&it, 3);
                let model = RegexUtils::captures_value(&it, 4);
                let device = RegexUtils::captures_value(&it, 5);
                let transport_id = RegexUtils::captures_value(&it, 6);
                let props_content = call_props_content(&serial_no);
                let props = Self::parse_props(props_content);

                Self {
                    serial_no,
                    state,
                    product,
                    model,
                    device,
                    transport_id,
                    props,
                }
            }
            None => Self {
                serial_no: String::new(),
                state: String::new(),
                product: String::new(),
                model: String::new(),
                device: String::new(),
                transport_id: String::new(),
                props: HashMap::new(),
            },
        };
    }

    /// 解析设备参数
    fn parse_props(content: String) -> HashMap<String, String> {
        let mut props = HashMap::new();
        let lines = content.split("\n");
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let kv: Vec<&str> = line.split(":").collect();
            if kv.len() == 2 {
                let key = kv[0].trim().to_string();
                let value = kv[1].trim().to_string();
                props.insert(key, value);
            }
        }
        return props;
    }
}
