pub struct Changes {
    pub add: i32,
    pub delete: i32,
    pub edit: i32,
}

pub struct CommitData {
    pub author: String,
    pub repo_id: String,
    pub pid: String,
    pub date: String,
    pub change: Changes,
}

impl CommitData {
    pub fn new() -> CommitData {
        return CommitData {
            author: "".to_string(),
            repo_id: "".to_string(),
            pid: "".to_string(),
            date: "".to_string(),
            change: Changes {
                add: 0,
                delete: 0,
                edit: 0,
            },
        };
    }

    pub fn set_author(mut self, author: String) -> CommitData {
        self.author = author;
        self
    }

    pub fn set_repo_id(mut self, repo_id: String) -> CommitData {
        self.repo_id = repo_id;
        self
    }

    pub fn set_pid(mut self, pid: String) -> CommitData {
        self.pid = pid;
        self
    }

    pub fn set_date(mut self, date: String) -> CommitData {
        self.date = date;
        self
    }

    pub fn set_changes(mut self, add: i32, delete: i32, edit: i32) -> CommitData {
        self.change.add = add;
        self.change.delete = delete;
        self.change.edit = edit;
        self
    }
}

#[derive(Debug)]
pub struct ContentData {
    data: String,
}

impl ContentData {
    pub fn new(content: String) -> ContentData {
        ContentData { data: content }
    }

    pub fn dispaly(self) -> String {
        return format!("{{\"content\":\"{}\"}}", self.data);
    }
}
