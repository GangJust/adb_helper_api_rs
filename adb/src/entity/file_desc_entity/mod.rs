use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDesc {
    pub inode: String,
    pub permission_str: String,
    pub link_count: String,
    pub owner: String,
    pub group: String,
    pub size: String,
    pub datetime: String,
    pub name: String,
    pub kind: String,
    pub path: String,
}

impl FileDesc {
    /// 解析文件信息
    pub fn parse(content: &str, path: &str) -> Vec<Self> {
        let path = match path.strip_suffix("/") {
            Some(it) => format!("{}/", it),
            None => format!("{}/", path),
        };

        content
            .split("\n")
            .filter(|item| !item.trim().is_empty())
            .map(|item| {
                let item = item.trim();

                // ls: *: No such file or directory
                if item.ends_with("No such file or directory") {
                    return Some(Self {
                        inode: "".to_string(),
                        permission_str: "".to_string(),
                        link_count: "".to_string(),
                        owner: "".to_string(),
                        group: "".to_string(),
                        size: "".to_string(),
                        datetime: "".to_string(),
                        name: item.to_string(),
                        kind: "no-such".to_string(),
                        path: path.clone(),
                    });
                }

                // ls: *: Permission denied
                if item.ends_with("Permission denied") {
                    return Some(Self {
                        inode: "".to_string(),
                        permission_str: "".to_string(),
                        link_count: "".to_string(),
                        owner: "".to_string(),
                        group: "".to_string(),
                        size: "".to_string(),
                        datetime: "".to_string(),
                        name: item.to_string(),
                        kind: "no-permission".to_string(),
                        path: path.clone(),
                    });
                }

                let splits = item.split_whitespace().collect::<Vec<&str>>();
                if splits.len() >= 8 {
                    let inode = splits[0];
                    let permission_str = splits[1];
                    let link_count = splits[2];
                    let owner = splits[3];
                    let group = splits[4];
                    let size = splits[5];
                    let is_no_oermission = splits[6] == "?";
                    let datetime = if is_no_oermission {
                        splits[6].to_string() //无权限时，日期时间为第7列
                    } else {
                        splits[6..8].join(" ") //合并日期和时间
                    };
                    let name = if is_no_oermission {
                        splits[7..].join(" ") // 文件名可能包含空格, 向后合并剩余部分
                    } else {
                        splits[8..].join(" ") // 文件名可能包含空格, 向后合并剩余部分
                    };
                    let kind = match &permission_str[0..1] {
                        "l" => "link".to_string(),
                        "d" => "directory".to_string(),
                        _ => "file".to_string(),
                    };

                    Some(Self {
                        inode: inode.to_string(),
                        permission_str: permission_str.to_string(),
                        link_count: link_count.to_string(),
                        owner: owner.to_string(),
                        group: group.to_string(),
                        size: size.to_string(),
                        datetime: datetime.to_string(),
                        name: name.to_string(),
                        kind: kind.to_string(),
                        path: path.clone(),
                    })
                } else {
                    None
                }
            })
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect::<Vec<Self>>()
    }
}
