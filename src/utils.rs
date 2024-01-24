use std::ops::Sub as _;

use chrono::{Duration, Local};
use tabled::{builder::Builder, settings::Style};

use crate::model;

pub fn sort_commit_data(commit_data: Vec<model::CommitData>) -> String {
    let tabled_data = vec![
        "Author".to_string(),
        "Repo Id".to_string(),
        "Project Id".to_string(),
        "Date".to_string(),
        "Change Count".to_string(),
    ];

    let mut t_builder = Builder::new();
    t_builder.push_record(tabled_data);

    for cd in commit_data {
        let data = vec![
            cd.author,
            cd.repo_id,
            cd.pid,
            cd.date,
            (cd.change.add + cd.change.edit + cd.change.delete).to_string(),
        ];
        t_builder.push_record(data)
    }

    let date = Local::now();
    let diff = Duration::days(7);

    let end_date = date.sub(diff);

    let end_date_formatted = date.format("%m/%d/%Y %H:%M:%S %p").to_string();
    let start_date_formatted = end_date.format("%m/%d/%Y %H:%M:%S %p").to_string();

    let header_text = format!(
        "Scan taken between: {end_date} to {start_date}",
        end_date = end_date_formatted,
        start_date = start_date_formatted
    );

    let table = t_builder.build().with(Style::markdown()).to_string();

    let new_str = format!("> {}\n\n", header_text) + &table;

    new_str
}
