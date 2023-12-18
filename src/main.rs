#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::http::ContentType;
use rocket::State;

const INDEX: &'static str = include_str!("../static/index.html");
const STYLE: &'static str = include_str!("../static/simple.min.css");
const FAVICON: &'static [u8; 1_150] = include_bytes!("../static/favicon.ico");

#[derive(Debug, PartialEq, FromFormField)]
enum IPVersion {
    V4,
    V6,
}

#[derive(Debug, PartialEq, FromForm)]
struct CountryRequest<'a> {
    country: &'a str,
    name: &'a str,
    timeout: &'a str,
    version: IPVersion,
}
fn process_cidr_block(blocks: &str, country_request: CountryRequest) -> String {
    let CountryRequest {
        country,
        name,
        timeout,
        version,
    } = country_request;
    let header: String = match version {
        IPVersion::V4 => "/ip firewall address-list".into(),
        IPVersion::V6 => "/ipv6 firewall address-list".into(),
    };
    let mut commands = vec![header];
    let add_commands = blocks.trim().split('\n').map(|s| {
        format!(
            ":do {{ add address={} list={} timeout={} comment={} }} on-error={{}}",
            s, name, timeout, country
        )
    });
    commands.extend(add_commands);
    commands.join("\n")
}

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, INDEX)
}

#[get("/favicon.ico")]
async fn favicon() -> (ContentType, &'static [u8]) {
    (ContentType::Icon, FAVICON)
}

#[get("/simple.min.css")]
async fn style() -> (ContentType, &'static str) {
    (ContentType::CSS, STYLE)
}

#[get("/api/v0/list?<list..>")]
async fn list(list: CountryRequest<'_>, client: &State<Client>) -> Option<String> {
    let ver = match list.version {
        IPVersion::V4 => "ipv4",
        IPVersion::V6 => "ipv6",
    };
    let url = format!(
        "https://raw.githubusercontent.com/herrbischoff/country-ip-blocks/master/{}/{}.cidr",
        ver,
        list.country.to_lowercase()
    );

    match client.get(&url).send().await {
        Ok(body) => {
            debug!("Loaded {} country", &list.country);
            body.text()
                .await
                .ok()
                .as_deref()
                .map(|block| process_cidr_block(block, list))
        }
        Err(error) => {
            error!("Failed to obtain {} country: {:?}", &list.country, error);
            None
        }
    }
}

#[launch]
fn rocket() -> _ {
    let client = Client::new();

    rocket::build()
        .manage(client)
        .mount("/", routes![index, list, favicon, style])
}
