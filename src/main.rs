extern crate clap;
extern crate failure;
extern crate reqwest;
extern crate tokio;
extern crate url;

enum SubCommand {
    Download,
    Login,
}
impl SubCommand {
    fn value(&self) -> String {
        match *self {
            SubCommand::Download => "download".to_string(),
            SubCommand::Login => "login".to_string(),
        }
    }
}

async fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::Client::new().get(url).send().await?;
    let html = res.text().await?;
    Ok(html)
}

async fn download(url: &str) -> Result<(), failure::Error> {
    let url = url::Url::parse(url)?;
    let html = get_html(url.as_str()).await?;
    println!("{}", html);
    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = clap::App::new("atcoder-sample-downloader")
        .version("1.0")
        .author("Hitoshi Togasaki. <togasakitogatoga+github.com>")
        .about(
            "Download sample test cases of AtCoder problem
Example:
    //Download sample test cases
    atcoder-sample-donwloader download https://atcoder.jp/contests/agc035/tasks/agc035_a
    //Login AtCoder and save your session in your local
    atcoder-sample-donwloader login",
        )
        .subcommand(
            clap::SubCommand::with_name(&SubCommand::Download.value())
                .about("Download sample test cases in your local")
                .arg(
                    clap::Arg::with_name("url")
                        .help("A URL of AtCoder problem")
                        .required(true),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name(&SubCommand::Login.value())
                .about("Login AtCoder and save session in your local"),
        )
        .get_matches();

    //run sub commands
    if let Some(ref matched) = matches.subcommand_matches(&SubCommand::Download.value()) {
        match download(matched.value_of("url").unwrap()).await {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
