use clap::ArgMatches;

pub struct CollectArgs<'a> {
    pub branch: &'a str,
    pub matcher: &'a str,
    pub path: &'a str,
    pub points_path: Option<&'a str>
}

pub struct TotalArgs<'a> {
    pub path: Option<&'a str>,
    pub points_path: Option<&'a str>
}

pub fn parse_collect_args<'a>(matches: &'a ArgMatches) -> CollectArgs<'a> {
    let mut branch = "master";
    if let Some(requested_branch) = matches.value_of("branch") {
        branch = requested_branch;
    }

    let mut matcher = "v1";
    if let Some(requested_matcher) = matches.value_of("matcher") {
        matcher = requested_matcher;
    }

    let mut path = ".";
    if let Some(requested_path) = matches.value_of("filepath") {
        path = requested_path;
    }

    let points_path = matches.value_of("points");

    CollectArgs {
        branch,
        matcher,
        path,
        points_path
    }
}



pub fn parse_total_args<'a>(matches: &'a ArgMatches) -> TotalArgs<'a> {
    let path = matches.value_of("in");
    let points_path = matches.value_of("points");

    TotalArgs {
        path,
        points_path
    }
}