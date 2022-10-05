extern crate google_sheets4 as sheets4;
extern crate yup_oauth2 as oauth2;

use crate::config::{config_client::ConfigClient, field_name::FieldName};

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
use std::{clone::Clone, cmp::PartialEq, collections::HashMap, str::FromStr};

pub struct PlayerCharacterClient {
    hub: Sheets<HttpsConnector<HttpConnector>>,
    sheet_config: ConfigClient,
    data: Option<Vec<ValueRange>>,
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

        let sheet_config = ConfigClient::new();
        Ok(PlayerCharacterClient {
            hub,
            sheet_config,
            data: None,
        })
    }

    pub async fn parse_data(&mut self, sheet_id: String) -> Result<PlayerCharacter, StatusCode> {
        self.data = match self.load_data(sheet_id).await {
            Ok(result) => Some(result),
            Err(err) => return Err(err),
        };

        let character_name = self.get_value(FieldName::CharacterName);
        let player_name = self.get_value(FieldName::PlayerName);
        let version_sheet = self.get_value(FieldName::VersionSheet);
        let archetype = self.get_value(FieldName::Archetype);
        let generation_raw = self.get_value::<String>(FieldName::Generation);
        let generation: Either<u8, String> = if let Ok(parsed) = generation_raw.parse() {
            Left(parsed)
        } else {
            Right(generation_raw)
        };
        let clan = self.get_value(FieldName::Clan);
        let blood_per_turn = self.get_value::<u8>(FieldName::BlutvorratBlutProRunde);
        let blood_pool = self.get_value::<u8>(FieldName::BlutvorratBlutpool);
        let attribut_physical_value = self.get_value::<u8>(FieldName::AttributKörperlichWert);
        let attribut_social_value = self.get_value::<u8>(FieldName::AttributSozialWert);
        let attribut_mental_value = self.get_value::<u8>(FieldName::AttributMentalWert);
        let attribut_physical_foci = self.get_value_vec(FieldName::AttributKörperlicheFoki);
        let attribut_social_foci = self.get_value_vec(FieldName::AttributSozialeFoki);
        let attribut_mental_foci = self.get_value_vec(FieldName::AttributMentaleFoki);
        let academics_value = self.get_value::<u8>(FieldName::SkillAkademischesWissen);
        let academics_foci =
            self.get_skill_specialization(FieldName::SkillAkademischesWissenSpezialisierung);
        let subterfuge_value = self.get_value::<u8>(FieldName::SkillAusfluechte);
        let dodge_value = self.get_value::<u8>(FieldName::SkillAusweichen);
        let computer_value = self.get_value::<u8>(FieldName::SkillComputer);
        let intimidation_value = self.get_value::<u8>(FieldName::SkillEinschüchtern);
        let empathy_value = self.get_value::<u8>(FieldName::SkillEmpathie);
        let drive_value = self.get_value::<u8>(FieldName::SkillFahren);
        let leadership_value = self.get_value::<u8>(FieldName::SkillFührungsqualitäten);
        let brawl_value = self.get_value::<u8>(FieldName::SkillHandgemenge);
        let craft_a_value = self.get_value::<u8>(FieldName::SkillHandwerkA);
        let craft_a_foci = self.get_skill_specialization(FieldName::SkillHandwerkASpezialisierung);
        let craft_b_value = self.get_value::<u8>(FieldName::SkillHandwerkB);
        let craft_b_foci = self.get_skill_specialization(FieldName::SkillHandwerkBSpezialisierung);
        let stealth_value = self.get_value::<u8>(FieldName::SkillHeimlichkeit);
        let linguistics_value = self.get_value::<u8>(FieldName::SkillLinguistik);
        let linguistics_foci =
            self.get_skill_specialization(FieldName::SkillLinguistikSpezialisierung);
        let awareness_value = self.get_value::<u8>(FieldName::SkillMagiegespür);
        let medicine_value = self.get_value::<u8>(FieldName::SkillMedizin);
        let investigation_value = self.get_value::<u8>(FieldName::SkillNachforschungen);
        let melee_value = self.get_value::<u8>(FieldName::SkillNahkampf);
        let science_a_value = self.get_value::<u8>(FieldName::SkillNaturwissenschaftenA);
        let science_a_foci =
            self.get_skill_specialization(FieldName::SkillNaturwissenschaftenASpezialisierung);
        let science_b_value = self.get_value::<u8>(FieldName::SkillNaturwissenschaftenB);
        let science_b_foci =
            self.get_skill_specialization(FieldName::SkillNaturwissenschaftenBSpezialisierung);
        let occult_value = self.get_value::<u8>(FieldName::SkillOkkultismus);
        let firearms_value = self.get_value::<u8>(FieldName::SkillSchusswaffen);
        let security_value = self.get_value::<u8>(FieldName::SkillSicherheit);
        let athletics_value = self.get_value::<u8>(FieldName::SkillSportlichkeit);
        let streetwise_value = self.get_value::<u8>(FieldName::SkillSzenekenntnis);
        let animal_ken_value = self.get_value::<u8>(FieldName::SkillTierkunde);
        let survival_value = self.get_value::<u8>(FieldName::SkillÜberleben);
        let performance_a_value = self.get_value::<u8>(FieldName::SkillVortragA);
        let performance_a_foci =
            self.get_skill_specialization(FieldName::SkillVortragASpezialisierung);
        let performance_b_value = self.get_value(FieldName::SkillVortragB);
        let performance_b_foci =
            self.get_skill_specialization(FieldName::SkillVortragBSpezialisierung);
        let lore_value = self.get_value::<u8>(FieldName::SkillÜbernatürlichesWissen);
        let lore_foci =
            self.get_skill_specialization(FieldName::SkillÜbernatürlichesWissenSpezialisierung);
        let in_clan_disciplines = self.get_disciplines(FieldName::InClanDisziplinen);
        let out_of_clan_disciplines = self.get_disciplines(FieldName::OutOfClanDisziplinen);
        let techniques = self.get_value_vec(FieldName::Techniken);
        let in_clan_elder_powers = self.get_value_vec(FieldName::InClanAhnenkräfte);
        let out_of_clan_elder_powers = self.get_value_vec(FieldName::OutOfClanAhnenkräfte);
        let morality_name = self.get_value(FieldName::MoralvorstellungName);
        let morality_value = self.get_value::<u8>(FieldName::MoralvorstellungWert);
        let faction_name = self.get_value(FieldName::FraktionName);
        let merits_and_flaws = self
            .extract_value_from_result(FieldName::MeritsFlaws)
            .unwrap();
        let merits = self.extract_merits(&merits_and_flaws);
        let flaws = self.get_flaws(&merits_and_flaws);
        let backgrounds = self.get_backgrounds(FieldName::Backgrounds);
        let experience_start_value = self.get_value::<u8>(FieldName::ErfahrungspunkteStartpunkte);
        let experience_spent_total =
            self.get_value::<u16>(FieldName::ErfahrungspunkteGesamtAusgegeben);
        let experience_remaining = self.get_value::<i16>(FieldName::ErfahrungspunkteAktuellFrei);
        let experience_received_total =
            self.get_value::<u8>(FieldName::ErfahrungspunkteGesamtErhalten);
        let initiative = self.get_value::<u8>(FieldName::Initiative);
        let initiative_with_celerity = self.get_value::<u8>(FieldName::InitiativeGeschwindigkeit);
        let health_healthy_track = self.get_health_track(FieldName::GesundheitHealthy);
        let health_injured_track = self.get_health_track(FieldName::GesundheitInjured);
        let health_incapacitated_track = self.get_health_track(FieldName::GesundheitIncapacitated);
        let physical_defense_base = self.get_value::<u8>(FieldName::VerteidigungKörperlichRegulär);
        let physical_defense_with_celerity =
            self.get_value::<u8>(FieldName::VerteidigungKörperlichMitGeschwindigkeit);
        let physical_defense_frenzy_modifier =
            self.get_value::<i8>(FieldName::VerteidigungKörperlichRasereiModifier);
        let physical_defense_on_the_ground_closer_than_3_meters =
            self.get_value::<i8>(FieldName::VerteidigungKörperlichAmBodenGegnerNäherAls3Meter);
        let physical_defense_on_the_ground_further_than_3_meters =
            self.get_value::<u8>(FieldName::VerteidigungKörperlichAmBodenGegnerMindestens3MeterWeg);
        let physical_defense_special =
            self.get_value::<i8>(FieldName::VerteidigungKörperlichSpecial);
        let social_defense_pool = self.get_non_physical_defense_pool(FieldName::VerteidigungSozial);
        let mental_defense_pool = self.get_non_physical_defense_pool(FieldName::VerteidigungMental);
        let offense_pools = self.get_attack_pools(FieldName::AngriffsPools);
        let rituals = self.get_rituals(FieldName::Rituale);
        let items = self.get_items(FieldName::Items);
        let valid = !experience_spent_total.gt(&900_u16);

        // creating the result struct
        Ok(PlayerCharacter {
            character_name,
            player_name,
            version_sheet,
            valid,
            archetype,
            generation,
            clan,
            blood_per_turn,
            blood_pool,
            attributes: Attributes {
                physical: Attribute {
                    value: attribut_physical_value,
                    foci: attribut_physical_foci,
                },
                social: Attribute {
                    value: attribut_social_value,
                    foci: attribut_social_foci,
                },
                mental: Attribute {
                    value: attribut_mental_value,
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
                value: morality_value,
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
    fn get_backgrounds(&self, field_name: FieldName) -> Vec<Background> {
        self.extract_value_from_result(field_name)
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
    fn get_disciplines(&self, field_name: FieldName) -> Vec<Discipline> {
        self.extract_value_from_result(field_name)
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
    fn get_flaws(&self, input: &[Vec<String>]) -> Vec<Flaw> {
        let re = Regex::new(r"^(N(?P<flaw_type>.*):|N )(?P<flaw_name>.+)$").unwrap();

        input
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
    fn extract_merits(&self, input: &[Vec<String>]) -> Vec<Merit> {
        let re = Regex::new(r"^(V(?P<merit_type>.*):|V )(?P<merit_name>.+)$").unwrap();

        input
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
    fn get_skill_specialization(&self, field_name: FieldName) -> Option<Vec<String>> {
        Some(
            self.get_value::<String>(field_name)
                .split(',')
                .map(|x| x.trim().to_string())
                .filter_map(|x| match x != "-" {
                    true => Some(x),
                    false => None,
                })
                .collect(),
        )
    }

    /// reads a health track information
    fn get_health_track(&self, field_name: FieldName) -> HealthTrack {
        let binding = self.extract_value_from_result(field_name).unwrap();
        let track = binding.get(0).unwrap().get(0..8).unwrap();
        let base_value = track.get(5).unwrap().parse::<u8>().unwrap();
        let with_boni = track.get(7).unwrap().parse::<u8>().unwrap();

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
    fn get_non_physical_defense_pool(&self, field_name: FieldName) -> HashMap<u8, u8> {
        let mut result: HashMap<u8, u8> = HashMap::new();
        let values = self
            .extract_value_from_result(field_name)
            .unwrap()
            .get(0)
            .unwrap()
            .clone();
        let first_value = values[0].parse::<u8>().unwrap();
        result.insert(0, first_value);
        for n in 2_u8..9_u8 {
            result.insert(n - 1, values[n as usize].parse::<u8>().unwrap());
        }

        result
    }

    /// reads the different attack pools
    fn get_attack_pools(&self, field_name: FieldName) -> Vec<BattleOffenseInformation> {
        let mut result: Vec<BattleOffenseInformation> = Vec::new();
        let binding = self.extract_value_from_result(field_name).unwrap();
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
    fn get_rituals(&self, field_name: FieldName) -> Vec<Ritual> {
        let mut result: Vec<Ritual> = Vec::new();
        let binding = self.extract_value_from_result(field_name).unwrap();
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
    fn get_items(&self, field_name: FieldName) -> Vec<Item> {
        let mut result: Vec<Item> = Vec::new();
        let binding = self
            .extract_value_from_result(field_name)
            .unwrap_or_default();

        if binding.is_empty() {
            return Vec::new();
        }

        let number_of_items = binding.len();
        let values = binding.get(0..number_of_items).unwrap();

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

    /// extracts a vec
    fn get_value_vec(&self, field_name: FieldName) -> Vec<String> {
        let data = self.extract_value_from_result(field_name);
        let max_data_size: usize = match data {
            Some(ref x) => x.len(),
            None => 1,
        };

        data.unwrap_or_else(|| vec![vec!["-".to_string()]])
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

    /// reads a single cell
    fn get_value<T>(&self, field_name: FieldName) -> T
    where
        T: FromStr + Default,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let binding = self
            .extract_value_from_result(field_name)
            .unwrap_or_else(|| vec![vec!["-".to_string()]]);
        let field = binding.get(0).unwrap().get(0).unwrap().clone();
        field.parse::<T>().unwrap_or_default()
    }

    fn extract_value_from_result(&self, field_name: FieldName) -> Option<Vec<Vec<String>>> {
        self.data
            .as_ref()
            .unwrap()
            .get(self.sheet_config.get_field_config(field_name).position as usize)
            .unwrap()
            .clone()
            .values
    }

    async fn load_data(&self, sheet_key: String) -> Result<Vec<ValueRange>, StatusCode> {
        let mut result = self.hub.spreadsheets().values_batch_get(sheet_key.as_str());

        for entry in self.sheet_config.get_field_config_sorted() {
            result = result.add_ranges(&entry.range);
        }

        let response = result.doit().await;

        match response {
            Ok(my_result) => Ok(my_result.1.value_ranges.unwrap()),
            Err(..) => Err(StatusCode::NOT_FOUND),
        }
    }
}
