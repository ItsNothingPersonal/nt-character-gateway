extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;

use super::{
    attribute::Attribute, attributes::Attributes, background::Background,
    battle_base_information::BattleBaseInformation,
    battle_defense_information::BattleDefenseInformation, battle_information::BattleInformation,
    battle_offense_information::BattleOffenseInformation, discipline::Discipline,
    experience_information::ExperienceInformation, flaw::Flaw, health_track::HealthTrack,
    health_tracks::HealthTracks, item::Item, merit::Merit, morality::Morality,
    name_value::NameValue, physical_defense_pool::PhysicalDefensePool,
    player_character::PlayerCharacter, powers::Powers, ritual::Ritual, skill::Skill,
    skills::Skills,
};
use either::Either::{self, Left, Right};
use hyper::{client::HttpConnector, StatusCode};
use hyper_rustls::HttpsConnector;
use regex::{Captures, Regex};
use sheets4::{api::ValueRange, Sheets};
use std::collections::HashMap;

pub struct PlayerCharacterClient {
    hub: Sheets<HttpsConnector<HttpConnector>>,
}

impl PlayerCharacterClient {
    pub async fn new(
        service_account_information: String,
    ) -> Result<PlayerCharacterClient, StatusCode> {
        let secret = if let Ok(credentials) =
            oauth2::parse_service_account_key(&service_account_information)
        {
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
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
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

        Ok(PlayerCharacterClient { hub })
    }

    pub async fn parse_data(&self, sheet_id: String) -> Result<PlayerCharacter, StatusCode> {
        let result_google_spreadsheet: Vec<ValueRange> = match self.load_data(sheet_id).await {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        let character_name = self.get_value_from_result_range(result_google_spreadsheet.get(0));
        let player_name = self.get_value_from_result_range(result_google_spreadsheet.get(1));
        let version_sheet = self.get_value_from_result_range(result_google_spreadsheet.get(2));
        let archetype = self.get_value_from_result_range(result_google_spreadsheet.get(3));
        let generation_raw = self.get_value_from_result_range(result_google_spreadsheet.get(4));
        let generation: Either<u8, String> = if let Ok(parsed) = generation_raw.parse() {
            Left(parsed)
        } else {
            Right(generation_raw)
        };
        let clan = self.get_value_from_result_range(result_google_spreadsheet.get(5));
        let blood_per_turn: u8 = self
            .get_value_from_result_range(result_google_spreadsheet.get(6))
            .parse()
            .unwrap();
        let blood_pool: u8 = self
            .get_value_from_result_range(result_google_spreadsheet.get(7))
            .parse()
            .unwrap();
        let attribut_physical_value =
            self.get_value_from_result_range(result_google_spreadsheet.get(8));
        let attribut_social_value =
            self.get_value_from_result_range(result_google_spreadsheet.get(9));
        let attribut_mental_value =
            self.get_value_from_result_range(result_google_spreadsheet.get(10));
        let attribut_physical_foci =
            self.get_value_from_result_column_range(result_google_spreadsheet.get(11));
        let attribut_social_foci =
            self.get_value_from_result_column_range(result_google_spreadsheet.get(12));
        let attribut_mental_foci =
            self.get_value_from_result_column_range(result_google_spreadsheet.get(13));
        let academics_value: u8 = self
            .get_value_from_result_range(result_google_spreadsheet.get(14))
            .parse()
            .unwrap();
        let academics_foci = self.get_skill_specialization(result_google_spreadsheet.get(15));
        let subterfuge_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(16));
        let dodge_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(17));
        let computer_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(18));
        let intimidation_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(19));
        let empathy_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(20));
        let drive_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(21));
        let leadership_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(22));
        let brawl_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(23));
        let craft_a_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(24));
        let craft_a_foci = self.get_skill_specialization(result_google_spreadsheet.get(25));
        let craft_b_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(26));
        let craft_b_foci = self.get_skill_specialization(result_google_spreadsheet.get(27));
        let stealth_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(28));
        let linguistics_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(29));
        let linguistics_foci = self.get_skill_specialization(result_google_spreadsheet.get(30));
        let awareness_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(31));
        let medicine_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(32));
        let investigation_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(33));
        let melee_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(34));
        let science_a_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(35));
        let science_a_foci = self.get_skill_specialization(result_google_spreadsheet.get(36));
        let science_b_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(37));
        let science_b_foci = self.get_skill_specialization(result_google_spreadsheet.get(38));
        let occult_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(39));
        let firearms_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(40));
        let security_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(41));
        let athletics_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(42));
        let streetwise_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(43));
        let animal_ken_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(44));
        let survival_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(45));
        let performance_a_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(46));
        let performance_a_foci = self.get_skill_specialization(result_google_spreadsheet.get(47));
        let performance_b_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(48));
        let performance_b_foci = self.get_skill_specialization(result_google_spreadsheet.get(49));
        let lore_value: u8 = self.get_value_as_u8(result_google_spreadsheet.get(50));
        let lore_foci = self.get_skill_specialization(result_google_spreadsheet.get(51));
        let in_clan_disciplines = self.get_disciplines(result_google_spreadsheet.get(52));
        let out_of_clan_disciplines = self.get_disciplines(result_google_spreadsheet.get(53));
        let techniques = self.get_value_from_result_column_range(result_google_spreadsheet.get(54));
        let in_clan_elder_powers =
            self.get_value_from_result_column_range(result_google_spreadsheet.get(55));
        let out_of_clan_elder_powers =
            self.get_value_from_result_column_range(result_google_spreadsheet.get(56));
        let morality_name = self.get_value_from_result_range(result_google_spreadsheet.get(57));
        let morality_value = self.get_value_from_result_range(result_google_spreadsheet.get(58));
        let faction_name = self.get_value_from_result_range(result_google_spreadsheet.get(59));
        let merits_and_flaws = result_google_spreadsheet.get(60);
        let merits = self.get_merits(merits_and_flaws);
        let flaws = self.get_flaws(merits_and_flaws);
        let backgrounds = self.get_backgrounds(result_google_spreadsheet.get(61));
        let experience_start_value = self.get_value_as_u8(result_google_spreadsheet.get(62));
        let experience_spent_total = self.get_value_as_u8(result_google_spreadsheet.get(63));
        let experience_remaining = self.get_value_as_u8(result_google_spreadsheet.get(64));
        let experience_received_total = self.get_value_as_u8(result_google_spreadsheet.get(65));
        let initiative = self.get_value_as_u8(result_google_spreadsheet.get(66));
        let initiative_with_celerity = self.get_value_as_u8(result_google_spreadsheet.get(67));
        let health_healthy_track = self.get_health_track(result_google_spreadsheet.get(68));
        let health_injured_track = self.get_health_track(result_google_spreadsheet.get(69));
        let health_incapacitated_track = self.get_health_track(result_google_spreadsheet.get(70));
        let physical_defense_base = self.get_value_as_u8(result_google_spreadsheet.get(71));
        let physical_defense_with_celerity =
            self.get_value_as_u8(result_google_spreadsheet.get(72));
        let physical_defense_frenzy_modifier: i8 =
            self.get_value_as_i8(result_google_spreadsheet.get(73));
        let physical_defense_on_the_ground_closer_than_3_meters: i8 =
            self.get_value_as_i8(result_google_spreadsheet.get(74));
        let physical_defense_on_the_ground_further_than_3_meters =
            self.get_value_as_u8(result_google_spreadsheet.get(75));
        let physical_defense_special: i8 = self.get_value_as_i8(result_google_spreadsheet.get(76));
        let social_defense_pool =
            self.get_non_physical_defense_pool(result_google_spreadsheet.get(77));
        let mental_defense_pool =
            self.get_non_physical_defense_pool(result_google_spreadsheet.get(78));
        let offense_pools = self.get_attack_pools(result_google_spreadsheet.get(79));
        let rituals = self.get_rituals(result_google_spreadsheet.get(80));
        let items = self.get_items(result_google_spreadsheet.get(81));

        // creating the result struct
        Ok(PlayerCharacter {
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
            experience_information: ExperienceInformation {
                start_value: experience_start_value,
                spent_total: experience_spent_total,
                available: experience_remaining,
                received_total: experience_received_total,
            },
            battle_information: BattleInformation {
                base: BattleBaseInformation {
                    initiative,
                    initiative_with_celerity,
                    health: HealthTracks {
                        healthy: health_healthy_track,
                        injured: health_injured_track,
                        incapacitated: health_incapacitated_track,
                    },
                },
                defense: BattleDefenseInformation {
                    physical_defense_pool: PhysicalDefensePool {
                        base_value: physical_defense_base,
                        base_value_with_celerity: physical_defense_with_celerity,
                        frenzy_modifier: physical_defense_frenzy_modifier,
                        on_the_ground_closer_than_three_meters_modifier:
                            physical_defense_on_the_ground_closer_than_3_meters,
                        on_the_ground_further_than_three_meters_modifier:
                            physical_defense_on_the_ground_further_than_3_meters,
                        special: physical_defense_special,
                    },
                    social_defense_pool,
                    mental_defense_pool,
                },
                offense: offense_pools,
            },
            rituals,
            items,
        })
    }

    /// reads the list of backgrounds
    fn get_backgrounds(&self, input: Option<&ValueRange>) -> Vec<Background> {
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

    /// reads the list of disciplines
    fn get_disciplines(&self, input: Option<&ValueRange>) -> Vec<Discipline> {
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

    /// reads the flaws of the combined list
    fn get_flaws(&self, input: Option<&ValueRange>) -> Vec<Flaw> {
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

    /// reads the merits of the combined list
    fn get_merits(&self, input: Option<&ValueRange>) -> Vec<Merit> {
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

    /// reads a skill specialization
    fn get_skill_specialization(&self, input: Option<&ValueRange>) -> Option<Vec<String>> {
        Some(
            self.get_value_from_result_range(input)
                .split(',')
                .map(|x| x.trim().to_string())
                .filter_map(|x| match x != "-" {
                    true => Some(x),
                    false => None,
                })
                .collect(),
        )
    }

    /// reads a numeric value (unsigned)
    fn get_value_as_u8(&self, input: Option<&ValueRange>) -> u8 {
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

    /// reads a numeric value (signed)
    fn get_value_as_i8(&self, input: Option<&ValueRange>) -> i8 {
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
    /// reads a health track information
    fn get_health_track(&self, input: Option<&ValueRange>) -> HealthTrack {
        let binding = input.unwrap().clone().values.unwrap();
        let track = binding.get(0).unwrap().get(0..8).unwrap();
        let base_value: u8 = track.get(5).unwrap().parse::<u8>().unwrap();
        let with_boni: u8 = track.get(7).unwrap().parse().unwrap();

        let mut lost: u8 = 0;

        for n in 0..5 {
            match track.get(n).unwrap().trim() {
                "x" => lost += 1,
                _ => break,
            }
        }

        let remaining: u8 = match lost > with_boni {
            true => 0,
            false => with_boni - lost,
        };

        HealthTrack {
            base_value,
            with_boni,
            remaining,
        }
    }

    /// reads the non-physical defense pools (social/mental)
    fn get_non_physical_defense_pool(&self, input: Option<&ValueRange>) -> HashMap<u8, u8> {
        let values = input
            .unwrap()
            .clone()
            .values
            .unwrap()
            .get(0)
            .unwrap()
            .clone();
        let mut result: HashMap<u8, u8> = HashMap::new();
        let first_value = values[0].parse::<u8>().unwrap();
        result.insert(0, first_value);
        for n in 2_u8..9_u8 {
            result.insert(n - 1, values[n as usize].parse::<u8>().unwrap());
        }

        result
    }

    /// reads the different attack pools
    fn get_attack_pools(&self, input: Option<&ValueRange>) -> Vec<BattleOffenseInformation> {
        let mut result: Vec<BattleOffenseInformation> = Vec::new();
        let binding = input.unwrap().clone().values.unwrap();
        let values = binding.get(0..24).unwrap();

        for entry in values.iter() {
            let offense_pool_data = entry.clone();
            let skill_name: String = offense_pool_data.get(0).unwrap_or(&"-".to_string()).clone();
            let skill_value: u8 = offense_pool_data.get(7).unwrap().parse::<u8>().unwrap_or(0);
            let attribute_name: String = offense_pool_data
                .get(10)
                .unwrap_or(&"-".to_string())
                .clone();
            let attribute_value: u8 = offense_pool_data
                .get(14)
                .unwrap()
                .parse::<u8>()
                .unwrap_or(0);
            let wildcard_name: String = offense_pool_data
                .get(17)
                .unwrap_or(&"-".to_string())
                .clone();
            let wildcard_value: u8 = offense_pool_data
                .get(21)
                .unwrap()
                .parse::<u8>()
                .unwrap_or(0);
            let pool: u8 = offense_pool_data
                .get(24)
                .unwrap()
                .parse::<u8>()
                .unwrap_or(0);
            let description: String = offense_pool_data.get(26).unwrap_or(&"".to_string()).clone();

            if skill_name.ne("-") && attribute_name.ne("-") {
                let offense_pool_entry = BattleOffenseInformation {
                    skill: NameValue {
                        name: skill_name,
                        value: skill_value,
                    },
                    attribute: NameValue {
                        name: attribute_name,
                        value: attribute_value,
                    },
                    wildcard: NameValue {
                        name: wildcard_name,
                        value: wildcard_value,
                    },
                    pool,
                    description,
                };

                result.push(offense_pool_entry);
            }
        }

        result
    }

    /// reads the rituals section
    fn get_rituals(&self, input: Option<&ValueRange>) -> Vec<Ritual> {
        let mut result: Vec<Ritual> = Vec::new();
        let binding = input.unwrap().clone().values.unwrap();
        let values = binding.get(0..15).unwrap();

        for entry in values.iter() {
            let ritual_data = entry.clone();
            let ritual_name: String = ritual_data.get(0).unwrap_or(&"-".to_string()).clone();
            let level: u8 = ritual_data.get(7).unwrap().parse::<u8>().unwrap_or(0);
            let description: String = ritual_data.get(9).unwrap_or(&"-".to_string()).clone();

            if ritual_name.ne("-") && level.ne(&0) {
                let re = Regex::new(r"^(?P<ritual_type>\w{1})\d{1}(?P<ritual_name>.+)$").unwrap();

                let captured: Option<Captures> = re.captures(ritual_name.as_str());

                let ritual_type = match captured.as_ref().and_then(|cap| {
                    cap.name("ritual_type").map(|ritual_type| {
                        *ritual_type
                            .as_str()
                            .trim()
                            .chars()
                            .collect::<Vec<char>>()
                            .first()
                            .unwrap()
                    })
                }) {
                    Some('A') => "Abyssal".to_string(),
                    Some('N') => "Necromancy".to_string(),
                    Some('T') => "Thaumaturgy".to_string(),
                    _ => "Unbekannt".to_string(),
                };

                let name: String = captured
                    .and_then(|cap| {
                        cap.name("ritual_name")
                            .map(|ritual_name| Some(ritual_name.as_str().trim().to_string()))
                            .unwrap()
                    })
                    .unwrap();

                let ritual_entry = Ritual {
                    name,
                    level,
                    description,
                    ritual_type,
                };

                result.push(ritual_entry);
            }
        }

        result
    }

    /// reads the rituals section
    fn get_items(&self, input: Option<&ValueRange>) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        let binding = input.unwrap().clone().values.unwrap_or_default();

        if binding.is_empty() {
            return Vec::new();
        }

        let values = binding.get(0..10).unwrap();

        for entry in values.iter() {
            let item_data = entry.clone();
            let name = item_data.get(0).unwrap_or(&"".to_string()).clone();
            let trait_1 = item_data.get(5).unwrap_or(&"".to_string()).clone();
            let trait_1_description = item_data.get(9).unwrap_or(&"".to_string()).clone();
            let trait_2 = item_data.get(13).unwrap_or(&"".to_string()).clone();
            let trait_2_description = item_data.get(17).unwrap_or(&"".to_string()).clone();
            let additional_trait = item_data.get(21).unwrap_or(&"".to_string()).clone();
            let additional_trait_description = item_data.get(25).unwrap_or(&"".to_string()).clone();

            if name.ne(&"".to_string()) {
                let result_entry = Item {
                    name,
                    trait_1,
                    trait_1_description,
                    trait_2,
                    trait_2_description,
                    additional_trait,
                    additional_trait_description,
                };

                result.push(result_entry);
            }
        }

        result
    }

    /// extracts an vec from the selected column range
    fn get_value_from_result_column_range(&self, input: Option<&ValueRange>) -> Vec<String> {
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

    /// extracts the value from the selected value range
    /// assumes that the selected range is one field
    fn get_value_from_result_range(&self, input: Option<&ValueRange>) -> String {
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

    async fn load_data(&self, sheet_key: String) -> Result<Vec<ValueRange>, StatusCode> {
        // loading the data
        let result = self
            .hub
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
            // Feld Erfahrungspunkte > Startpunkte
            .add_ranges("H58")
            // Feld Erfahrungspunkte > Gesamt ausgegeben
            .add_ranges("R58")
            // Feld Erfahrungspunkte > Aktuell frei
            .add_ranges("AB58")
            // Feld Erfahrungspunkte > Gesamt erhalten
            .add_ranges("H59")
            // Feld Initiative
            .add_ranges("H62")
            // Feld Initiative (Geschwindigkeit)
            .add_ranges("H63")
            // Feld Gesundheit > Healthy
            .add_ranges("S63:Z63")
            // Feld Gesundheit > Injured
            .add_ranges("S64:Z64")
            // Feld Gesundheit > Incapacitated
            .add_ranges("S65:Z65")
            // Feld Verteidigung > Körperlich > regulär
            .add_ranges("J69")
            // Feld Verteidigung > Körperlich > +Geschw.
            .add_ranges("M69")
            // Feld Verteidigung > Körperlich > Raserei Modifier
            .add_ranges("K70")
            // Feld Verteidigung > Körperlich > Am Boden > Gegner näher als 3m
            .add_ranges("K71")
            // Feld Verteidigung > Körperlich > Am Boden > Gegner mind. 3m weg
            .add_ranges("K72")
            // Feld Verteidigung > Körperlich > Special
            .add_ranges("K73")
            // Felder Verteidigung Sozial
            .add_ranges("U68:AC68")
            // Felder Verteidigung Mental
            .add_ranges("U71:AC71")
            // Felder Angriffs-Pools
            .add_ranges("A77:AA100")
            // Felder Rituale
            .add_ranges("A104:J118")
            // Felder Items
            .add_ranges("A122:Z131")
            .doit()
            .await;

        match result {
            Ok(my_result) => Ok(my_result.1.value_ranges.unwrap()),
            Err(..) => Err(StatusCode::NOT_FOUND),
        }
    }
}
