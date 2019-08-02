use clap::ArgMatches;

pub struct CollectArgs {
    pub branch: String,
    pub matcher: String,
    pub path: String,
    pub points_path: Option<String>
}

pub struct TotalArgs {
    pub paths: Option<Vec<String>>,
    pub points_path: Option<String>
}

pub fn parse_collect_args(matches: &ArgMatches) -> CollectArgs {
    let branch = matches.value_of("branch").unwrap_or_default().to_string();
    let matcher = matches.value_of("matcher").unwrap_or_default().to_string();
    let path = matches.value_of("filepath").unwrap_or_default().to_string();
    let points_path = match matches.value_of("points") {
        Some(points_path) => Some(points_path.to_string()),
        None => None
    };

    CollectArgs {
        branch,
        matcher,
        path,
        points_path
    }
}

pub fn parse_total_args(matches: &ArgMatches) -> TotalArgs {

    let paths = match matches.values_of("in") {
        Some(paths) => {
            paths.map(|str| str.to_string())
                .collect::<Vec<String>>()
        },
        None => Vec::new()
    };

    let points_path = match matches.value_of("points") {
        Some(points_path) => Some(points_path.to_string()),
        None => None
    };

    TotalArgs {
        paths: if paths.len() > 0 { Some(paths) } else { None },
        points_path
    }
}
