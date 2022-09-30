extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;

use axum::{extract::Path, http::StatusCode, routing::get, Json, Router};
use either::Either::{self, Left, Right};
use regex::{Captures, Regex};
use serde::Serialize;
use sheets4::{api::ValueRange, Sheets};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with the following routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/character/:sheet_key", get(character_data));

    // run our app
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let addr = format!("{}:{}", host, port).parse::<SocketAddr>().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

/// loading the character data from the passed in sheet
async fn character_data(
    Path(sheet_key): Path<String>,
) -> (StatusCode, Json<Option<PlayerCharacter>>) {
    tracing::debug!("Sheet Key: {:?}", sheet_key);

    // setting up connection to Google Spreadsheets
    let service_account_info = env::var("SERVICE_ACCOUNT_INFORMATION").unwrap_or_default();

    let secret = if let Ok(credentials) = oauth2::parse_service_account_key(&service_account_info) {
        tracing::debug!("parsed credentials successfully from env variable");
        credentials
    } else {
        tracing::debug!(
            "parsed credentials unsuccessfully from env variable, trying credentials.json now"
        );
        if let Ok(credentials) = oauth2::read_service_account_key("credentials.json").await {
            tracing::debug!("parsed credentials successfully from credentials.json");
            credentials
        } else {
            tracing::debug!("both parse attempts failed, giving up");
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(None));
        }
    };

    let authenticator = oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .expect("failed to create authenticator");

    let hub = Sheets::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        authenticator,
    );

    // loading the data
    let result_google_spreadsheet = if let Ok(result) = hub
        .spreadsheets()
        .values_batch_get(sheet_key.as_str())
        // Feld Charakter Name
        .add_ranges("D1")
        // Feld Spieler Name
        .add_ranges("N1")
        // Feld Version Sheet
        .add_ranges("AA1")
        // Feld Wesen
        .add_ranges("G4")
        // Feld Generation
        .add_ranges("AA4")
        // Feld Clan/Blutlinie
        .add_ranges("G5")
        // Feld Blutvorrat Subfeld Blut Pro Runde
        .add_ranges("AA5")
        // Feld Blutvorrat Subfeld Blutpool
        .add_ranges("AC5")
        // Feld Attribut Körperlich Wert
        .add_ranges("H8")
        // Feld Attribut Sozial Wert
        .add_ranges("R8")
        // Feld Geistig Wert
        .add_ranges("AB8")
        // Felder Körperliche Foki
        .add_ranges("D9:D11")
        // Felder Soziale Foki
        .add_ranges("N9:N11")
        // Felder Mentale Foki
        .add_ranges("X9:X11")
        // Feld Akademisches Wissen
        .add_ranges("H13")
        // Feld Akademisches Wissen Subfeld Spezialisierung
        .add_ranges("A14")
        // Feld Ausflüchte
        .add_ranges("H16")
        // Feld Ausweichen
        .add_ranges("H17")
        // Feld Computer
        .add_ranges("H18")
        // Feld Einschüchtern
        .add_ranges("H19")
        // Feld Empathie
        .add_ranges("H20")
        // Feld Fahren
        .add_ranges("H21")
        // Feld Führungsqualitäten
        .add_ranges("H22")
        // Feld Handgemenge
        .add_ranges("H23")
        // Feld Handwerk A
        .add_ranges("H24")
        // Feld Handwerk A Subfeld Spezialisierung
        .add_ranges("A25")
        // Feld Handwerk B
        .add_ranges("R13")
        // Feld Handwerk B Subfeld Spezialisierung
        .add_ranges("K14")
        // Feld Heimlichkeit
        .add_ranges("R15")
        // Feld Linguistik
        .add_ranges("R16")
        // Feld Linguistik Subfeld Spezialisierung
        .add_ranges("K17")
        // Feld Magiegespür
        .add_ranges("R19")
        // Feld Medizin
        .add_ranges("R20")
        // Feld Nachforschungen
        .add_ranges("R21")
        // Feld Nahkampf
        .add_ranges("R22")
        // Feld Naturwissenschaften A
        .add_ranges("R23")
        // Feld Naturwissenschaften A Subfeld Spezialisierung
        .add_ranges("K24")
        // Feld Naturwissenschaften B
        .add_ranges("R25")
        // Feld Naturwissenschaften B Subfeld Spezialisierung
        .add_ranges("K26")
        // Feld Okkultismus
        .add_ranges("AB13")
        // Feld Schusswaffen
        .add_ranges("AB14")
        // Feld Sicherheit
        .add_ranges("AB15")
        // Feld Sportlichkeit
        .add_ranges("AB16")
        // Feld Szenekenntnis
        .add_ranges("AB17")
        // Feld Tierkunde
        .add_ranges("AB18")
        // Feld Überleben
        .add_ranges("AB19")
        // Feld Vortrag A
        .add_ranges("AB20")
        // Feld Vortrag A Subfeld Spezialisierung
        .add_ranges("U21")
        // Feld Vortrag B
        .add_ranges("AB22")
        // Feld Vortrag B Subfeld Spezialisierung
        .add_ranges("U23")
        // Feld Übernatürliches Wissen
        .add_ranges("AB24")
        // Feld Übernatürliches Wissen Subfeld Spezialisierung
        .add_ranges("U25")
        // Felder In Clan Disziplinen
        .add_ranges("A29:H34")
        // Felder Out Of Clan Disziplinen"
        .add_ranges("K29:S34")
        // Felder Techniken
        .add_ranges("U29:U37")
        // Felder In Clan Ahnenkräfte
        .add_ranges("A36:A37")
        // Felder Out Of Clan Ahnenkräfte
        .add_ranges("K36:K37")
        // Feld Moralvorstellung Name
        .add_ranges("A41")
        // Feld Moralvorstellung Wert
        .add_ranges("H41")
        // Feld Fraktion Name
        .add_ranges("E42")
        // Felder Vorzüge/Schwächen
        .add_ranges("A44:H55")
        // Felder Backgrounds
        .add_ranges("K41:T55")
        .doit()
        .await
    {
        result.1.value_ranges.unwrap()
    } else {
        return (StatusCode::NOT_FOUND, Json(None));
    };

    // parsing the retrieved data
    let character_name = get_value_from_result_range(result_google_spreadsheet.get(0));
    let player_name = get_value_from_result_range(result_google_spreadsheet.get(1));
    let version_sheet = get_value_from_result_range(result_google_spreadsheet.get(2));
    let archetype = get_value_from_result_range(result_google_spreadsheet.get(3));
    let generation_raw = get_value_from_result_range(result_google_spreadsheet.get(4));
    let generation: Either<u8, String> = if let Ok(parsed) = generation_raw.parse() {
        Left(parsed)
    } else {
        Right(generation_raw)
    };
    let clan = get_value_from_result_range(result_google_spreadsheet.get(5));
    let blood_per_turn: u8 = get_value_from_result_range(result_google_spreadsheet.get(6))
        .parse()
        .unwrap();
    let blood_pool: u8 = get_value_from_result_range(result_google_spreadsheet.get(7))
        .parse()
        .unwrap();
    let attribut_physical_value = get_value_from_result_range(result_google_spreadsheet.get(8));
    let attribut_social_value = get_value_from_result_range(result_google_spreadsheet.get(9));
    let attribut_mental_value = get_value_from_result_range(result_google_spreadsheet.get(10));
    let attribut_physical_foci =
        get_value_from_result_column_range(result_google_spreadsheet.get(11));
    let attribut_social_foci =
        get_value_from_result_column_range(result_google_spreadsheet.get(12));
    let attribut_mental_foci =
        get_value_from_result_column_range(result_google_spreadsheet.get(13));
    let academics_value: u8 = get_value_from_result_range(result_google_spreadsheet.get(14))
        .parse()
        .unwrap();
    let academics_foci = get_skill_specialization(result_google_spreadsheet.get(15));
    let subterfuge_value: u8 = get_skill_value(result_google_spreadsheet.get(16));
    let dodge_value: u8 = get_skill_value(result_google_spreadsheet.get(17));
    let computer_value: u8 = get_skill_value(result_google_spreadsheet.get(18));
    let intimidation_value: u8 = get_skill_value(result_google_spreadsheet.get(19));
    let empathy_value: u8 = get_skill_value(result_google_spreadsheet.get(20));
    let drive_value: u8 = get_skill_value(result_google_spreadsheet.get(21));
    let leadership_value: u8 = get_skill_value(result_google_spreadsheet.get(22));
    let brawl_value: u8 = get_skill_value(result_google_spreadsheet.get(23));
    let craft_a_value: u8 = get_skill_value(result_google_spreadsheet.get(24));
    let craft_a_foci = get_skill_specialization(result_google_spreadsheet.get(25));
    let craft_b_value: u8 = get_skill_value(result_google_spreadsheet.get(26));
    let craft_b_foci = get_skill_specialization(result_google_spreadsheet.get(27));
    let stealth_value: u8 = get_skill_value(result_google_spreadsheet.get(28));
    let linguistics_value: u8 = get_skill_value(result_google_spreadsheet.get(29));
    let linguistics_foci = get_skill_specialization(result_google_spreadsheet.get(30));
    let awareness_value: u8 = get_skill_value(result_google_spreadsheet.get(31));
    let medicine_value: u8 = get_skill_value(result_google_spreadsheet.get(32));
    let investigation_value: u8 = get_skill_value(result_google_spreadsheet.get(33));
    let melee_value: u8 = get_skill_value(result_google_spreadsheet.get(34));
    let science_a_value: u8 = get_skill_value(result_google_spreadsheet.get(35));
    let science_a_foci = get_skill_specialization(result_google_spreadsheet.get(36));
    let science_b_value: u8 = get_skill_value(result_google_spreadsheet.get(37));
    let science_b_foci = get_skill_specialization(result_google_spreadsheet.get(38));
    let occult_value: u8 = get_skill_value(result_google_spreadsheet.get(39));
    let firearms_value: u8 = get_skill_value(result_google_spreadsheet.get(40));
    let security_value: u8 = get_skill_value(result_google_spreadsheet.get(41));
    let athletics_value: u8 = get_skill_value(result_google_spreadsheet.get(42));
    let streetwise_value: u8 = get_skill_value(result_google_spreadsheet.get(43));
    let animal_ken_value: u8 = get_skill_value(result_google_spreadsheet.get(44));
    let survival_value: u8 = get_skill_value(result_google_spreadsheet.get(45));
    let performance_a_value: u8 = get_skill_value(result_google_spreadsheet.get(46));
    let performance_a_foci = get_skill_specialization(result_google_spreadsheet.get(47));
    let performance_b_value: u8 = get_skill_value(result_google_spreadsheet.get(48));
    let performance_b_foci = get_skill_specialization(result_google_spreadsheet.get(49));
    let lore_value: u8 = get_skill_value(result_google_spreadsheet.get(50));
    let lore_foci = get_skill_specialization(result_google_spreadsheet.get(51));
    let in_clan_disciplines = get_disciplines(result_google_spreadsheet.get(52));
    let out_of_clan_disciplines = get_disciplines(result_google_spreadsheet.get(53));
    let techniques = get_value_from_result_column_range(result_google_spreadsheet.get(54));
    let in_clan_elder_powers =
        get_value_from_result_column_range(result_google_spreadsheet.get(55));
    let out_of_clan_elder_powers =
        get_value_from_result_column_range(result_google_spreadsheet.get(56));
    let morality_name = get_value_from_result_range(result_google_spreadsheet.get(57));
    let morality_value = get_value_from_result_range(result_google_spreadsheet.get(58));
    let faction_name = get_value_from_result_range(result_google_spreadsheet.get(59));
    let merits_and_flaws = result_google_spreadsheet.get(60);
    let merits = get_merits(merits_and_flaws);
    let flaws = get_flaws(merits_and_flaws);
    let backgrounds = get_backgrounds(result_google_spreadsheet.get(61));

    // creating the result struct
    let retrieved_character = PlayerCharacter {
        character_name,
        player_name,
        version_sheet,
        archetype,
        generation,
        clan,
        blood_per_turn,
        blood_pool,
        attributes: Attributes {
            physical: Attribute {
                value: attribut_physical_value.parse().unwrap(),
                foci: attribut_physical_foci,
            },
            social: Attribute {
                value: attribut_social_value.parse().unwrap(),
                foci: attribut_social_foci,
            },
            mental: Attribute {
                value: attribut_mental_value.parse().unwrap(),
                foci: attribut_mental_foci,
            },
        },
        skills: Skills {
            academics: Skill {
                value: academics_value,
                foci: academics_foci,
            },
            animal_ken: Skill {
                value: animal_ken_value,
                foci: None,
            },
            athletics: Skill {
                value: athletics_value,
                foci: None,
            },
            awareness: Skill {
                value: awareness_value,
                foci: None,
            },
            brawl: Skill {
                value: brawl_value,
                foci: None,
            },
            computer: Skill {
                value: computer_value,
                foci: None,
            },
            craft_a: Skill {
                value: craft_a_value,
                foci: craft_a_foci,
            },
            craft_b: Skill {
                value: craft_b_value,
                foci: craft_b_foci,
            },
            dodge: Skill {
                value: dodge_value,
                foci: None,
            },
            drive: Skill {
                value: drive_value,
                foci: None,
            },
            empathy: Skill {
                value: empathy_value,
                foci: None,
            },
            firearms: Skill {
                value: firearms_value,
                foci: None,
            },
            intimidation: Skill {
                value: intimidation_value,
                foci: None,
            },
            investigation: Skill {
                value: investigation_value,
                foci: None,
            },
            leadership: Skill {
                value: leadership_value,
                foci: None,
            },
            linguistics: Skill {
                value: linguistics_value,
                foci: linguistics_foci,
            },
            lore: Skill {
                value: lore_value,
                foci: lore_foci,
            },
            medicine: Skill {
                value: medicine_value,
                foci: None,
            },
            melee: Skill {
                value: melee_value,
                foci: None,
            },
            occult: Skill {
                value: occult_value,
                foci: None,
            },
            performance_a: Skill {
                value: performance_a_value,
                foci: performance_a_foci,
            },
            performance_b: Skill {
                value: performance_b_value,
                foci: performance_b_foci,
            },
            security: Skill {
                value: security_value,
                foci: None,
            },
            science_a: Skill {
                value: science_a_value,
                foci: science_a_foci,
            },
            science_b: Skill {
                value: science_b_value,
                foci: science_b_foci,
            },
            stealth: Skill {
                value: stealth_value,
                foci: None,
            },
            streetwise: Skill {
                value: streetwise_value,
                foci: None,
            },
            subterfuge: Skill {
                value: subterfuge_value,
                foci: None,
            },
            survival: Skill {
                value: survival_value,
                foci: None,
            },
        },
        powers: Powers {
            in_clan_disciplines,
            out_of_clan_disciplines,
            techniques,
            in_clan_elder_powers,
            out_of_clan_elder_powers,
        },
        morality: Morality {
            name: morality_name,
            value: morality_value.parse().unwrap(),
        },
        faction: faction_name,
        merits,
        flaws,
        backgrounds,
    };

    // returning the result
    (StatusCode::OK, Json(Some(retrieved_character)))
}

/// extracts the value from the selected value range
/// assumes that the selected range is one field
fn get_value_from_result_range(input: Option<&ValueRange>) -> String {
    input
        .unwrap()
        .clone()
        .values
        .unwrap_or(vec![vec!["-".to_string()]] as Vec<Vec<String>>)
        .get(0)
        .unwrap()
        .get(0)
        .unwrap()
        .clone()
}

/// extracts an vec from the selected column range
fn get_value_from_result_column_range(input: Option<&ValueRange>) -> Vec<String> {
    let max_data_size: usize = match input.unwrap().clone().values {
        Some(x) => x.len(),
        None => 1,
    };
    input
        .unwrap()
        .clone()
        .values
        .unwrap_or_else(|| vec![vec!["-".to_string()]])
        .get(0..max_data_size)
        .unwrap()
        .concat()
        .iter()
        .filter_map(|x| match x != "-" {
            true => Some(x.clone()),
            false => None,
        })
        .collect()
}

/// reads a numeric value for a skill
fn get_skill_value(input: Option<&ValueRange>) -> u8 {
    if let Ok(n) = input
        .unwrap()
        .clone()
        .values
        .unwrap()
        .get(0)
        .unwrap()
        .get(0)
        .unwrap()
        .parse()
    {
        n
    } else {
        0
    }
}

/// reads a skill specialization
fn get_skill_specialization(input: Option<&ValueRange>) -> Option<Vec<String>> {
    Some(
        get_value_from_result_range(input)
            .split(',')
            .map(|x| x.trim().to_string())
            .filter_map(|x| match x != "-" {
                true => Some(x),
                false => None,
            })
            .collect(),
    )
}

/// reads the list of disciplines
fn get_disciplines(input: Option<&ValueRange>) -> Vec<Discipline> {
    input
        .unwrap()
        .clone()
        .values
        .unwrap()
        .get(0..6)
        .unwrap()
        .iter()
        .filter(|x| x.get(0).unwrap() != "-")
        .map(|x| -> Discipline {
            Discipline {
                name: x.get(0).unwrap().clone(),
                value: x.get(7).unwrap().parse().unwrap_or(0),
            }
        })
        .collect::<Vec<Discipline>>()
}

/// reads the merits of the combined list
fn get_merits(input: Option<&ValueRange>) -> Vec<Merit> {
    let re = Regex::new(r"^(V(?P<merit_type>.*):|V )(?P<merit_name>.+)$").unwrap();

    input
        .unwrap()
        .clone()
        .values
        .unwrap()
        .get(0..7)
        .unwrap()
        .iter()
        .filter(|x| x.get(0).unwrap() != "-")
        .filter(|x| !x.get(0).unwrap().starts_with('N'))
        .map(|x| -> Merit {
            let captured: Option<Captures> = re.captures(x.get(0).unwrap());

            let name_normalized = match captured.as_ref().and_then(|cap| {
                cap.name("merit_name")
                    .map(|merit_name| merit_name.as_str().trim().to_string())
            }) {
                Some(value) => value,
                None => x.get(0).unwrap().clone(),
            };

            let merit_type = match captured.and_then(|cap| {
                cap.name("merit_type")
                    .map(|merit_type| merit_type.as_str().trim().to_string())
            }) {
                Some(value) => value,
                None => "General".to_string(),
            };

            Merit {
                name: name_normalized,
                value: match x.get(7).unwrap().parse() {
                    Ok(val) => val,
                    Err(..) => 0,
                },
                merit_type,
            }
        })
        .collect()
}

/// reads the flaws of the combined list
fn get_flaws(input: Option<&ValueRange>) -> Vec<Flaw> {
    let re = Regex::new(r"^(N(?P<flaw_type>.*):|N )(?P<flaw_name>.+)$").unwrap();

    input
        .unwrap()
        .clone()
        .values
        .unwrap()
        .get(0..7)
        .unwrap()
        .iter()
        .filter(|x| x.get(0).unwrap() != "-")
        .filter(|x| x.get(0).unwrap().starts_with('N'))
        .map(|x| -> Flaw {
            let captured: Option<Captures> = re.captures(x.get(0).unwrap());

            let name_normalized = match captured.as_ref().and_then(|cap| {
                cap.name("flaw_name")
                    .map(|flaw_name| flaw_name.as_str().trim().to_string())
            }) {
                Some(value) => value,
                None => x.get(0).unwrap().clone(),
            };

            let flaw_type = match captured.and_then(|cap| {
                cap.name("flaw_type")
                    .map(|flaw_type| flaw_type.as_str().trim().to_string())
            }) {
                Some(value) => value,
                None => "General".to_string(),
            };

            Flaw {
                name: name_normalized,
                value: match x.get(7).unwrap().parse() {
                    Ok(val) => val,
                    Err(..) => 0,
                },
                flaw_type,
            }
        })
        .collect()
}

/// reads the list of backgrounds
fn get_backgrounds(input: Option<&ValueRange>) -> Vec<Background> {
    input
        .unwrap()
        .clone()
        .values
        .unwrap()
        .get(0..9)
        .unwrap()
        .iter()
        .filter(|x| x.get(0).unwrap().trim() != "")
        .map(|x| -> Background {
            Background {
                name: x.get(0).unwrap().clone(),
                value: x.get(7).unwrap().parse().unwrap_or(0),
                description: x.get(9).unwrap_or(&"".to_string()).clone(),
            }
        })
        .collect::<Vec<Background>>()
}
/// the output to the character_data handler
#[derive(Serialize)]
struct PlayerCharacter {
    character_name: String,
    player_name: String,
    version_sheet: String,
    archetype: String,
    generation: Either<u8, String>,
    clan: String,
    blood_per_turn: u8,
    blood_pool: u8,
    attributes: Attributes,
    skills: Skills,
    powers: Powers,
    morality: Morality,
    faction: String,
    merits: Vec<Merit>,
    flaws: Vec<Flaw>,
    backgrounds: Vec<Background>,
}

/// Attributes Struct
#[derive(Serialize)]
struct Attributes {
    physical: Attribute,
    social: Attribute,
    mental: Attribute,
}

/// Attribut Struct
#[derive(Serialize)]
struct Attribute {
    value: u8,
    foci: Vec<String>,
}

/// Skills Struct
#[derive(Serialize)]
struct Skills {
    academics: Skill,
    athletics: Skill,
    animal_ken: Skill,
    awareness: Skill,
    brawl: Skill,
    computer: Skill,
    craft_a: Skill,
    craft_b: Skill,
    dodge: Skill,
    drive: Skill,
    empathy: Skill,
    firearms: Skill,
    intimidation: Skill,
    investigation: Skill,
    leadership: Skill,
    linguistics: Skill,
    lore: Skill,
    medicine: Skill,
    melee: Skill,
    occult: Skill,
    performance_a: Skill,
    performance_b: Skill,
    security: Skill,
    science_a: Skill,
    science_b: Skill,
    stealth: Skill,
    streetwise: Skill,
    subterfuge: Skill,
    survival: Skill,
}

/// Skill Struct
#[derive(Serialize)]
struct Skill {
    value: u8,
    foci: Option<Vec<String>>,
}

/// Powers Struct
#[derive(Serialize)]
struct Powers {
    in_clan_disciplines: Vec<Discipline>,
    out_of_clan_disciplines: Vec<Discipline>,
    techniques: Vec<String>,
    in_clan_elder_powers: Vec<String>,
    out_of_clan_elder_powers: Vec<String>,
}

/// Discipline Struct
#[derive(Serialize, Debug)]
struct Discipline {
    name: String,
    value: u8,
}

/// Morality Struct
#[derive(Serialize, Debug)]
struct Morality {
    name: String,
    value: u8,
}

/// Merit Struct
#[derive(Serialize)]
struct Merit {
    name: String,
    value: i8,
    merit_type: String,
}

/// Flaw Struct
#[derive(Serialize)]
struct Flaw {
    name: String,
    value: i8,
    flaw_type: String,
}

/// Background Struct
#[derive(Serialize, Debug)]
struct Background {
    name: String,
    value: u8,
    description: String,
}
