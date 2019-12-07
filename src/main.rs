extern crate clap;
extern crate failure;
extern crate reqwest;
extern crate scraper;
extern crate selectors;
extern crate tokio;
extern crate url;

use itertools::Itertools;
use selectors::Element;
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

fn parse_html(html: &str) -> scraper::Html {
    let document = scraper::Html::parse_document(html);
    document
}

async fn login(url: &str) -> Result<(), failure::Error> {
    let url = url::Url::parse(url)?;
    println!("{}", url);
    Ok(())
}

fn parse_sample_cases(document: &scraper::Html) -> Result<Vec<(String, String)>, failure::Error> {
    let task_statement_selector = scraper::Selector::parse(r#"div[id="task-statement"]"#).unwrap();
    let pre_selector = scraper::Selector::parse("pre").unwrap();
    let h3_selector = scraper::Selector::parse("h3").unwrap();
    let input_h3_text = vec!["入力例", "Sample Input"];
    let output_h3_text = vec!["出力例", "Sample Output"];

    let mut input_cases = vec![];
    let mut output_cases = vec![];
    if let Some(task_statement) = document.select(&task_statement_selector).next() {
        for pre in task_statement.select(&pre_selector) {
            if let Some(pre_parent) = pre.parent_element() {
                if let Some(h3) = pre_parent.select(&h3_selector).next() {
                    let h3_text = h3.text().collect::<String>();
                    let input = input_h3_text.iter().any(|&x| h3_text.contains(x));
                    let output = output_h3_text.iter().any(|&x| h3_text.contains(x));
                    let text = pre.text().collect::<String>();
                    if input {
                        input_cases.push(text);
                    } else if output {
                        output_cases.push(text);
                    }
                }
            }
        }
    } else {
        //return error
    }
    // make cases unique to remove extra duplicated language cases
    let input_cases: Vec<String> = input_cases.into_iter().unique().collect();
    let output_cases: Vec<String> = output_cases.into_iter().unique().collect();
    println!("Found {} test cases", input_cases.len());
    let sample_test_cases: Vec<(String, String)> = input_cases
        .into_iter()
        .zip(output_cases)
        .enumerate()
        .map(|(idx, (input, output))| {
            println!("=== Sample Test Case {} ===", idx + 1);
            println!("Input:\n{}\nOutput:\n{}", input, output);
            println!("===========================");
            (input, output)
        })
        .collect();
    Ok(sample_test_cases)
}

async fn download(url: &str) -> Result<(), failure::Error> {
    let url = url::Url::parse(url)?;
    let html = get_html(url.as_str()).await?;
    let document = parse_html(&html);
    let sample_test_cases = parse_sample_cases(&document)?;
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
                .about("Login AtCoder and save session in your local")
                .arg(clap::Arg::with_name("url").help("A login URL of AtCoder")),
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
    if let Some(ref matched) = matches.subcommand_matches(&SubCommand::Login.value()) {
        match login(
            matched
                .value_of("url")
                .unwrap_or("https://atcoder.jp/login"),
        )
        .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
