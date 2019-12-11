extern crate clap;
extern crate dirs;
extern crate failure;
extern crate reqwest;
extern crate rpassword;
extern crate scraper;
extern crate selectors;
extern crate tokio;
extern crate url;

use itertools::Itertools;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, SET_COOKIE};
use selectors::Element;
use std::collections::HashMap;
use std::io::{Read, Write};

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

async fn call_get_request(url: &str) -> Result<reqwest::Response, reqwest::Error> {
    let req = reqwest::Client::builder()
        .build()
        .unwrap()
        .get(url)
        .send()
        .await?;
    Ok(req)
}
async fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let req = call_get_request(url).await?;
    let html = req.text().await?;
    Ok(html)
}

fn parse_html(html: &str) -> scraper::Html {
    let document = scraper::Html::parse_document(html);
    document
}

fn get_csrf_token(html: &str) -> Option<String> {
    let document: scraper::Html = parse_html(&html);
    let csrf_token = parse_token(&document);
    csrf_token
}
fn parse_token(document: &scraper::Html) -> Option<String> {
    let selector = scraper::Selector::parse(r#"input[name="csrf_token"]"#).unwrap();
    if let Some(element) = document.select(&selector).next() {
        if let Some(token) = element.value().attr("value") {
            return Some(token.to_string());
        }
    }
    return None;
}

fn get_cookie_headers(resp: &reqwest::Response) -> HeaderMap {
    let mut cookie_headers = HeaderMap::new();
    let cookies = resp.cookies().collect::<Vec<_>>();

    cookies.iter().for_each(|cookie| {
        cookie_headers.insert(
            COOKIE,
            HeaderValue::from_str(&format!("{}={}", &cookie.name(), &cookie.value())).unwrap(),
        );
    });
    cookie_headers
}

fn get_username_and_password() -> (String, String) {
    println!("Please input Your username and password");
    let username = rpassword::read_password_from_tty(Some("Username > ")).unwrap();
    let password = rpassword::read_password_from_tty(Some("Password > ")).unwrap();
    (username, password)
}

fn save_cookie_in_local(cookies_str: &str) -> Result<(), failure::Error> {
    let path = dirs::home_dir().unwrap().join(".atcoder-sample-downloader");
    //create $HOME/.atcoder-sample-downloader
    std::fs::create_dir(path.clone())?;
    //create cookie.jar under this directory
    std::fs::File::create(path.join("cookie.jar"))?.write_all(cookies_str.as_bytes());
    Ok(())
}
async fn login(url: &str) -> Result<(), failure::Error> {
    let url = url::Url::parse(url)?;
    let resp = call_get_request(url.as_str()).await?;

    //necessary information and parameters to login AtCoder
    let cookie_headers = get_cookie_headers(&resp);
    let html = resp.text().await?;
    let csrf_token = get_csrf_token(&html).unwrap();
    let (username, password) = get_username_and_password();
    let mut params = HashMap::new();
    params.insert("username", username);
    params.insert("password", password);
    params.insert("csrf_token", csrf_token);

    //make a post request and try to login
    let resp: reqwest::Response = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap()
        .post(url)
        .headers(cookie_headers)
        .form(&params)
        .send()
        .await?;

    let cookies_str = resp
        .cookies()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .join(";");
    save_cookie_in_local(&cookies_str);
    let html = resp.text().await?;
    println!("{}", html);
    Ok(())
}

fn create_sample_test_files(test_cases: &[(String, String)]) -> Result<(), failure::Error> {
    for (idx, (input, output)) in test_cases.iter().enumerate() {
        //e.g sample_input_1.txt sample_output_1.txt
        let input_file_name = format!("sample_input_{}.txt", idx + 1);
        let mut input_file = std::fs::File::create(input_file_name)?;
        input_file.write_all(input.as_bytes())?;

        let output_file_name = format!("sample_output_{}.txt", idx + 1);
        let mut output_file = std::fs::File::create(output_file_name)?;
        output_file.write_all(output.as_bytes())?;
    }
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
    create_sample_test_files(&sample_test_cases)?;
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
    //Download
    atcoder-sample-donwloader download https://atcoder.jp/contests/agc035/tasks/agc035_a
    //Login
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
