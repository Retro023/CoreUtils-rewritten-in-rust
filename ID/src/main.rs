use colored::Colorize;
use std::env;
use std::fs;
// structs for the user and groups
#[derive(Debug)]
struct User {
    name: String,
    uid: u32,
    gid: u32,
}

#[derive(Debug)]
struct Group {
    name: String,
    gid: u32,
    members: Vec<String>,
}

//read group file and get groups
fn all_groups() -> Vec<Group> {
    let mut groups = Vec::new();
    if let Ok(content) = fs::read_to_string("/etc/group") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let members = if parts[3].is_empty() {
                    vec![]
                } else {
                    parts[3].split(',').map(|s| s.to_string()).collect()
                };
                if let Ok(gid) = parts[2].parse() {
                    groups.push(Group {
                        name: parts[0].to_string(),
                        gid,
                        members,
                    });
                }
            }
        }
    }
    groups
}
// get the group name
fn group_name(gid: u32, groups: &[Group]) -> String {
    for g in groups {
        if g.gid == gid {
            return g.name.clone();
        }
    }
    "unknown".to_string()
}
// get the current user
fn current_user() -> Option<User> {
    let username = env::var("USERNAME")
        .or_else(|_| env::var("LOGNAME"))
        .unwrap_or_default();
    if username.is_empty() {
        return None;
    }

    let passwd = fs::read_to_string("/etc/passwd").ok()?;
    for line in passwd.lines() {
        if line.starts_with(&format!("{}:", username)) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                return Some(User {
                    name: parts[0].to_string(),
                    uid: parts[2].parse().ok()?,
                    gid: parts[3].parse().ok()?,
                });
            }
        }
    }
    None
}

// main
fn main() {
    let user = match current_user() {
        Some(u) => u,
        _none => {
            eprintln!("Failed to determine current user.");
            return;
        }
    };

    let groups = all_groups();

    let user_groups: Vec<&Group> = groups
        .iter()
        .filter(|g| g.gid == user.gid || g.members.contains(&user.name))
        .collect();

    let group_list = user_groups
        .iter()
        .map(|g| format!("{}({})", g.name.yellow(), g.gid.to_string().purple()))
        .collect::<Vec<_>>()
        .join(",");

    println!(
        "{}={}({}) {}={}({}) {}={}",
        // syntax highlited ID
        "uid".bright_blue(),
        user.uid.to_string().purple(),
        user.name.cyan(),
        "gid".bright_blue(),
        user.gid.to_string().purple(),
        group_name(user.gid, &groups).cyan(),
        "groups".bright_blue(),
        group_list
    );
}
