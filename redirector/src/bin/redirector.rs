use actix_web::{
    get, middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::{DateTime, Utc};
use clap::Parser;
use env_logger::Env;
use log::{error, info, warn};
use maxminddb::{
    geoip2::{Asn, Country},
    Reader,
};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

struct AppState {
    reqwest: reqwest::Client,
    maxmind_countries: Option<Reader<Vec<u8>>>,
    maxmind_asn: Option<Reader<Vec<u8>>>,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, value_parser, default_value_t = String::from("127.0.0.1"))]
    host: String,

    #[clap(short, long, value_parser, default_value_t = 5000)]
    port: u16,
}

#[derive(Serialize, Debug)]
struct Click {
    ip: String,
    created_at: DateTime<Utc>,

    network: Option<String>,
    campaignid: Option<String>,
    adgroup: Option<String>,
    ad: Option<String>,
    device: Option<String>,
    devicemodel: Option<String>,
    placement: Option<String>,
    gclid: Option<String>,
    keyword: Option<String>,
    url: String,

    country_iso_code: Option<String>,
    asn_organization: Option<String>,
    asn_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct Params {
    network: Option<String>,
    campaignid: Option<String>,
    adgroup: Option<String>,
    ad: Option<String>,
    device: Option<String>,
    devicemodel: Option<String>,
    placement: Option<String>,
    gclid: Option<String>,
    keyword: Option<String>,
    url: String,
}

#[get("/")]
async fn redirect(
    data: web::Data<AppState>,
    params: web::Query<Params>,
    req: HttpRequest,
) -> impl Responder {
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .expect("No IP address found")
        .to_owned();
    let now = Utc::now();

    let Params {
        network,
        campaignid,
        adgroup,
        ad,
        device,
        devicemodel,
        placement,
        gclid,
        keyword,
        url,
    } = params.into_inner();

    let parsed_ip: IpAddr = ip.parse().unwrap();

    let country_iso_code = if let Some(maxmind_countries) = &data.maxmind_countries {
        match maxmind_countries.lookup::<Country>(parsed_ip) {
            Ok(country) => country
                .country
                .and_then(|country| country.iso_code)
                .map(|iso_code| iso_code.to_owned()),
            Err(err) => {
                error!("MaxMindDBError: {}", err);

                None
            }
        }
    } else {
        None
    };

    let (asn_organization, asn_number) = if let Some(maxmind_asn) = &data.maxmind_asn {
        match maxmind_asn.lookup::<Asn>(parsed_ip) {
            Ok(asn) => (
                asn.autonomous_system_organization.map(|org| org.to_owned()),
                asn.autonomous_system_number,
            ),
            Err(err) => {
                error!("MaxMindDBError: {}", err);

                (None, None)
            }
        }
    } else {
        (None, None)
    };

    let info = Click {
        ip,
        created_at: now,
        network,
        campaignid,
        adgroup,
        ad,
        device,
        devicemodel,
        placement,
        gclid,
        keyword,
        url: url.clone(),
        country_iso_code,
        asn_number,
        asn_organization,
    };

    // TODO: We will just log bad results...
    let res = data
        .reqwest
        .post("http://db:8000/key/click")
        .basic_auth("root", Some("root"))
        .header("NS", "ppc")
        .header("DB", "ppc")
        .json(&info)
        .send()
        .await;

    match res {
        Ok(_) => (),
        Err(err) => error!("Saving data to SurrealDB failed: {}", err),
    }

    HttpResponse::Found()
        .insert_header(("Location", url))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = Args::parse();

    info!("Server starting on {}:{}", args.host, args.port);

    let reqwest = reqwest::Client::new();

    let maxmind_countries =
        match maxminddb::Reader::open_readfile("data/geolocation/GeoLite2-Country.mmdb") {
            Ok(reader) => Some(reader),
            Err(err) => {
                warn!("Maxmind Country DB load failed: {}", err);

                None
            }
        };

    let maxmind_asn = match maxminddb::Reader::open_readfile("data/geolocation/GeoLite2-ASN.mmdb") {
        Ok(reader) => Some(reader),
        Err(err) => {
            warn!("Maxmind ASN DB load failed: {}", err);

            None
        }
    };

    let state = web::Data::new(AppState {
        reqwest,
        maxmind_countries,
        maxmind_asn,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&state))
            .wrap(Logger::default())
            .service(redirect)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
