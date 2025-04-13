use std::{env, process::Command};

#[derive(Debug)]
struct Url {
    base_url: String,
    args: Vec<String>,
    tag: String,
}

impl Url {
    fn new(arguments: Vec<String>) -> Self {
        let mut base_url = String::new();
        let mut tag = String::new();
        let mut args: Vec<String> = arguments;

        if args.len() == 1 {
            eprintln!("Need url or arguments");
        } else if args.len() == 2 {
            base_url = args.get(1).unwrap().to_string();
            args = vec!["".to_string()];
        } else if args.get(1).unwrap().contains("-") {
            tag = args.get(1).unwrap().to_string();
            args.remove(0);
            args.remove(0);
            args = args;
        } else {
            eprintln!("...syntax error...");
        }
        Url {
            base_url,
            args,
            tag,
        }
    }

    fn make_url(&self, base_url: &str) -> String {
        let mut url = String::new();
        println!("... {}", url);
        if self.tag.is_empty() {
            url = format!("{}", &self.base_url);
        } else {
            let url_args = self.args.join("+");
            url = format!("{}{}", base_url, url_args);
        }
        if url.contains("http") {
            return url;
        } else {
            eprintln!("Url not found");
            return format!("Invalid url: {}", url);
        }
    }

    fn browse(&self, base_url: &str) {
        Command::new("xdg-open")
            .arg(&self.make_url(base_url))
            .output()
            // .spawn()
            .expect("Error running the command...");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).unwrap().as_str() {
        "-g" => {
            let google = Url::new(args);
            google.browse("https://google.com/search?q=");
        }
        "-y" => {
            let youtube = Url::new(args);
            youtube.browse("https://www.youtube.com/results?search_query=");
        }
        "-gs" => {
            let scholar = Url::new(args);
            scholar.browse("https://scholar.google.com/scholar?q=");
        }
        "-gh" => {
            let github = Url::new(args);
            github.browse("https://github.com/search?q=");
        }
        "help" | "-h" => {
            help();
        }
        _ => {
            let website = Url::new(args);
            website.browse(website.base_url.as_str());
        }
    }
}

fn help() {
    println!(
        r#"
    Usage: <TAG> <ARGS>
    Example: -g why rust is best?
    -----------------------------
    [ just use url to visit with no tags ]
    Use tags for specific websites:
    -y  -> Youtube
    -g  -> Google Search
    -gs -> Google Scholar
    -gh -> Github
    "#
    )
}
