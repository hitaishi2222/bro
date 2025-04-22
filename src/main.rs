use std::{env, process};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "query", required = true)]
    query: Vec<String>,

    #[arg(short, long, help = "Google search", required = false)]
    google: bool,

    #[arg(
        short = 'S',
        long,
        help = "Sci-Hub PDF search",
        aliases = [ "scihub", "sci-hub" ],
        required = false
    )]
    sci_hub: bool,

    #[arg(
        short = 's',
        long,
        help = "Google Scholar search",
        aliases = [ "google-scholar", "googlescholar" ],
        required = false
    )]
    google_scholar: bool,

    #[arg(
        short = 'G',
        long,
        help = "Github Repositories search",
        aliases = ["git-hub"],
        required = false
    )]
    github: bool,

    #[arg(
        short, 
        long,
        help = "Youtube search",
        aliases = ["you-tube"],
        required = false
    )]
    youtube: bool,

    #[arg(
        short,
        long="rd",
        help = "Rust documentation std search",
        aliases = ["rust-doc", "rustdoc"],
        required = false
    )]
    rust_doc: bool,
}

impl Cli {
    fn make_url(&self, base_url: &str) -> String {
        if self.query.contains(&"http".to_string()) {
            return self.query.join("");
        }
        format!("{}{}", base_url, self.query.join("+"))
    }

    fn browse(&self, base_url: &str) {
        let url = self.make_url(base_url);
        let os = env::consts::OS;

        if os == "windows" {
            process::Command::new("cmd")
                .args(&["/C", "start", "", &url])
                .spawn()
                .expect("Error running the command on Windows...");
        } else {
            let cmd = if os == "linux" {
                "xdg-open"
            } else if os == "macos" {
                "open"
            } else {
                "echo"
            };

            process::Command::new(cmd)
                .arg(&url)
                .spawn()
                .expect("Error running the command...");
        }
    }
}


fn main() {
    let args = Cli::parse();
    if args.query.is_empty() {
        eprintln!(">> No input detected: use -h or --help");
    } else if args.google {
        args.browse("https://google.com/search?q=");
    } else if args.google_scholar {
        args.browse("https://scholar.google.com/scholar?q=");
    } else if args.github {
        args.browse("https://github.com/search?q=");
    } else if args.youtube {
        args.browse("https://www.youtube.com/results?search_query=");
    } else if args.rust_doc {
        args.browse("https://doc.rust-lang.org/stable/std/?search=");
    } else if args.sci_hub {
        args.browse("https://pismin.com/");
    }
    else {
        if args.query.join("").contains(&"htt".to_string()) {
            args.browse("");
        } else {
            eprintln!("Not a valid URL")
        }
    }
}
