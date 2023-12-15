#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::State;

#[derive(Debug, PartialEq, FromForm)]
struct CountryRequest<'a> {
    country: &'a str,
    name: &'a str,
    timeout: &'a str,
}
fn process_cidr_block(blocks: &str, country_request: CountryRequest) -> String {
    let CountryRequest {
        country,
        name,
        timeout,
    } = country_request;
    let add_commands = blocks.trim().split('\n').map(|s| {
        format!(
            ":do {{ add address={} list={} timeout={} comment={} }} on-error={{}}",
            s, name, timeout, country
        )
    });
    let mut commands = vec![String::from("/ip firewall address-list")];
    commands.extend(add_commands);
    commands.join("\n")
}

#[get("/api/v0/list?<list..>")]
async fn list(list: CountryRequest<'_>, client: &State<Client>) -> Option<String> {
    let url = format!(
        "https://raw.githubusercontent.com/herrbischoff/country-ip-blocks/master/ipv4/{}.cidr",
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

    rocket::build().manage(client).mount("/", routes![list])
}
