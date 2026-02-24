use crate::{
    cli::list::ListCli,
    directories,
    helpers::{format_name, ExitErr},
    projects, templates,
    tmux::session_exists,
};

pub fn list_handler(args: ListCli) {
    let projects = projects::parse_project_config();
    for project in projects {
        if args.running && !session_exists(&project.name).unwrap_or(false) {
            continue;
        }

        println!(
            "{}",
            format_name(args.format_project.as_deref(), &project.name)
        );
    }

    let templates = templates::parse_template_config();
    for template in templates {
        let is_hidden = template.hidden.unwrap_or(false);
        if is_hidden && !args.all {
            continue;
        }
        if args.running && !session_exists(&template.name).unwrap_or(false) {
            continue;
        }

        println!(
            "{}",
            format_name(args.format_template.as_deref(), &template.name)
        );
    }

    let dirs = directories::parse_directory_config().exit_err(1);
    let dirs = dirs
        .names()
        .filter(|name| !args.running || session_exists(*name).unwrap_or(false));
    for name in dirs {
        println!("{}", format_name(args.format_directory.as_deref(), name));
    }
}
