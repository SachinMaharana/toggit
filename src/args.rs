use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "toggit",
    about = "toggle your github repository private or public"
)]
pub struct Cli {
    #[structopt(short, long)]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub cmd: Toggit,
}
#[derive(Debug, StructOpt)]
#[structopt()]
pub enum Toggit {
    #[structopt(about = "Initialize toggit")]
    Init,
    #[structopt(about = "toggle a repo to private or public")]
    Toggle {
        #[structopt(name = "repo", required = true)]
        repo: String,
    },
    #[structopt(about = "get the current visibility of a repo")]
    Visible {
        #[structopt(name = "visible", required = true)]
        repo: String,
    },
}

pub fn get_cli() -> Cli {
    Cli::from_args()
}
