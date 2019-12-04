extern crate clap;
extern crate failure;
extern crate reqwest;
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

async fn call_get_request(url: &str) -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get("https://hyper.rs")
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);
    Ok(())
    // Ok(body)
}

fn download(url: &str) -> Result<(), failure::Error> {
    let url = url::Url::parse(url)?;
    println!("Call");
    call_get_request(url.as_str()).await?;
    println!("Called");
    // println!("html {:?}", html);
    // println!("{:?}", html.unwrap());
    Ok(())
}

fn main() {
    let matches = clap::App::new("atcoder-sample-downloader")
        .version("1.0")
        .author("Hitoshi Togasaki. <togasakitogatoga+github.com>")
        .about("Download sample test cases of AtCoder(https://atcoder.jp)")
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
        match download(matched.value_of("url").unwrap()) {
            Ok(()) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
