use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDesc {
    pub inode: String,
    pub permission_str: String,
    pub link_count: String,
    pub owner: String,
    pub group: String,
    pub size: String,
    pub modification_date: String,
    pub modification_time: String,
    pub name: String,
    pub kind: String,
    pub path: String,
}

impl FileDesc {
    /// 解析文件信息
    pub fn parse(content: &str, path: &str) -> Vec<Self> {
        content
            .split("\n")
            .map(|item| {
                let splits = item.split_whitespace().collect::<Vec<&str>>();
                if splits.len() < 9 {
                    return None;
                }

                let inode = splits[0];
                let permission_str = splits[1];
                let link_count = splits[2];
                let owner = splits[3];
                let group = splits[4];
                let size = splits[5];
                let modification_date = splits[6];
                let modification_time = splits[7];
                let name = splits[8..].join(" "); // 文件名可能包含空格, 向后合并剩余部分
                let path = match path.strip_suffix("/") {
                    Some(it) => format!("{}/", it),
                    None => format!("{}/", path),
                };
                let kind = if permission_str.starts_with("d") {
                    "directory"
                } else if permission_str.starts_with("l") {
                    "link"
                } else {
                    "file"
                };

                Some(Self {
                    inode: inode.to_string(),
                    permission_str: permission_str.to_string(),
                    link_count: link_count.to_string(),
                    owner: owner.to_string(),
                    group: group.to_string(),
                    size: size.to_string(),
                    modification_date: modification_date.to_string(),
                    modification_time: modification_time.to_string(),
                    name: name.to_string(),
                    kind: kind.to_string(),
                    path,
                })
            })
            .filter(|item| item.is_some())
            .map(|item| item.unwrap())
            .collect::<Vec<Self>>()
    }
}
