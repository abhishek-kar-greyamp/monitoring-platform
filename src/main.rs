use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use config::{Config, File};

pub mod model;
pub mod utils;

use utils::sort_commit_data;

use model::CommitData;

mod logger;

pub mod api_client;

fn main() {
    logger::LOGGER.init_logger();

    let source = File::from(Path::new("./config.yml"));
    let config = Config::builder().add_source(source).build();
    let mut pat: String = String::new();
    let mut org_url: String = String::new();

    match config {
        Ok(c) => {
            match c.get("PAT") {
                Ok(p) => pat = p,
                Err(_) => {
                    // logger::Logger.log_error("PAT not found in config");
                    logger::LOGGER.log_error("PAT not found in config file")
                }
            }
            match c.get("ORG_URL") {
                Ok(v) => org_url = v,
                Err(_) => {
                    logger::LOGGER.log_error("Org URL not found in config");
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }

    logger::LOGGER.log_info("Config successfully Read");

    let api_client_inst = api_client::ApiClient::new(pat.to_string(), org_url.to_string());
    let api_client_clone = api_client_inst.clone();
    let prjts = api_client_inst.get_projects();

    let mut project_list: Vec<String> = vec![];

    match prjts {
        Ok(projects) => {
            if projects["value"].is_array() {
                match projects["value"].as_array() {
                    Some(project_arr) => {
                        for project in project_arr {
                            let id = project["id"].as_str().unwrap();
                            project_list.push(id.to_string())
                        }
                    }
                    None => panic!("Not Projects Found."),
                }
            }
        }
        Err(_) => logger::LOGGER.log_info("Config successfully Read"),
    }

    let mut handles = vec![];
    let repo_list = Arc::new(Mutex::new(vec![]));
    let api_client_mutex = Arc::new(Mutex::new(api_client_clone));

    for pid in project_list {
        let rp = Arc::clone(&repo_list);
        let ac = Arc::clone(&api_client_mutex);
        let handle = thread::spawn(move || {
            let res_ac = ac.lock().unwrap();
            let pid_clone = pid.clone();

            let res = res_ac.clone().get_repositories(pid);

            match res {
                Ok(repos) => {
                    if repos["value"].is_array() {
                        match repos["value"].as_array() {
                            Some(repo_array) => {
                                for repo in repo_array {
                                    let id = repo["id"].as_str().unwrap();
                                    let name = repo["name"].as_str().unwrap();
                                    let mut repo_list = rp.lock().unwrap();
                                    let mut hm: HashMap<String, String> = HashMap::new();
                                    hm.insert("id".to_string(), id.to_string());
                                    hm.insert("name".to_string(), name.to_string());
                                    hm.insert("pid".to_string(), pid_clone.to_string());
                                    repo_list.push(hm)
                                }
                            }
                            None => {
                                let p = pid_clone.clone();
                                logger::LOGGER
                                    .log_error(&format!("No repos found for Project Id: {}", p))
                            }
                        }
                    }
                    logger::LOGGER.log_info(&format!("Repositories computed for {}", pid_clone))
                }
                Err(_) => println!("Error"),
            }
        });
        handles.push(handle)
    }
    for handle in handles {
        handle.join().unwrap()
    }

    let mut commit_data: Vec<CommitData> = vec![];

    for repos in &*repo_list.lock().unwrap() {
        match repos.get("id") {
            Some(id) => {
                let pid = repos.get("pid").unwrap();
                let res = api_client_mutex
                    .lock()
                    .unwrap()
                    .clone()
                    .get_commits(pid.to_string(), id.to_string());
                match res {
                    Ok(res) => {
                        let count = res["count"].as_i64().unwrap();
                        if count > 0 {
                            let value_arr = res["value"].as_array().unwrap();
                            for commit in value_arr {
                                let cd = CommitData::new();

                                let author = commit["author"].as_object().unwrap();
                                let email =
                                    author.get("email").unwrap().as_str().unwrap().to_string();
                                let date =
                                    author.get("date").unwrap().as_str().unwrap().to_string();

                                let change = commit["changeCounts"].as_object().unwrap();
                                let add = change.get("Add").unwrap().as_i64().unwrap() as i32;
                                let edit = change.get("Edit").unwrap().as_i64().unwrap() as i32;
                                let delete = change.get("Delete").unwrap().as_i64().unwrap() as i32;

                                let mod_cd = cd
                                    .set_author(email)
                                    .set_date(date)
                                    .set_pid(pid.to_string())
                                    .set_changes(add, delete, edit);
                                commit_data.push(mod_cd);
                            }
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            None => todo!(),
        }
    }

    let data = sort_commit_data(commit_data);
    let res = Arc::clone(&api_client_mutex)
        .lock()
        .unwrap()
        .clone()
        .write_wiki(data);
    match res {
        Ok(_) => logger::LOGGER.log_info("Export Completed"),
        Err(e) => {
            let msg = format!("{:?}", e);
            logger::LOGGER.log_error(&msg)
        }
    }
}
