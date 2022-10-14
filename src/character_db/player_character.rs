use super::{
    attribute::Attribute,
    attributes::{Attributes, AttributesUpdateInput},
    background::{Background, BackgroundUpdateInput},
    battle_base_information::BattleBaseInformation,
    battle_defense_information::BattleDefenseInformation,
    battle_information::BattleInformation,
    battle_offense_information::BattleOffenseInformation,
    discipline::{Discipline, DisciplineUpdateInput},
    experience_information::{ExperienceInformation, ExperienceInformationUpdateInput},
    flaw::{Flaw, FlawUpdateInput},
    health_track::HealthTrack,
    health_tracks::HealthTracks,
    item::Item,
    merit::{Merit, MeritUpdateInput},
    morality::{Morality, MoralityUpdateInput},
    name_value::NameValue,
    physical_defense_pool::PhysicalDefensePool,
    powers::{Powers, PowersUpdateInput},
    ritual::{Ritual, RitualUpdateInput},
    skill::Skill,
    skills::{Skills, SkillsUpdateInput},
};
use crate::config::{config_client::ConfigClient, field_name::FieldName};
use either::Either::{self, Left, Right};
use google_sheets4::api::ValueRange;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, str::FromStr};

/// the output to the character_data handler
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerCharacter {
    pub character_name: String,
    pub player_name: String,
    pub version_sheet: String,
    pub valid: bool,
    pub archetype: String,
    pub generation: Either<u8, String>,
    pub clan: String,
    pub blood_per_turn: u8,
    pub blood_pool: u8,
    pub attributes: Attributes,
    pub skills: Skills,
    pub powers: Powers,
    pub morality: Morality,
    pub faction: String,
    pub merits: Vec<Merit>,
    pub flaws: Vec<Flaw>,
    pub backgrounds: Vec<Background>,
    pub experience_information: ExperienceInformation,
    pub battle_information: BattleInformation,
    pub rituals: Vec<Ritual>,
    pub items: Vec<Item>,
}

/// the input for updating a character
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerCharacterUpdateInput {
    pub character_name: Option<String>,
    pub player_name: Option<String>,
    pub archetype: Option<String>,
    pub generation: Option<Either<u8, String>>,
    pub clan: Option<String>,
    pub attributes: Option<AttributesUpdateInput>,
    pub skills: Option<SkillsUpdateInput>,
    pub powers: Option<PowersUpdateInput>,
    pub morality: Option<MoralityUpdateInput>,
    pub faction: Option<String>,
    pub merits: Option<Vec<MeritUpdateInput>>,
    pub flaws: Option<Vec<FlawUpdateInput>>,
    pub backgrounds: Option<Vec<BackgroundUpdateInput>>,
    pub experience_information: Option<ExperienceInformationUpdateInput>,
    pub rituals: Option<Vec<RitualUpdateInput>>,
}

impl From<Vec<ValueRange>> for PlayerCharacter {
    fn from(data: Vec<ValueRange>) -> Self {
        let config = ConfigClient::new();

        let character_name = get_value(&data, FieldName::CharacterName, &config);
        let player_name = get_value(&data, FieldName::PlayerName, &config);
        let version_sheet = get_value(&data, FieldName::VersionSheet, &config);
        let archetype = get_value(&data, FieldName::Archetype, &config);
        let generation_raw = get_value::<String>(&data, FieldName::Generation, &config);
        let generation: Either<u8, String> = if let Ok(parsed) = generation_raw.parse() {
            Left(parsed)
        } else {
            Right(generation_raw)
        };
        let clan = get_value(&data, FieldName::Clan, &config);
        let blood_per_turn = get_value::<u8>(&data, FieldName::BlutvorratBlutProRunde, &config);
        let blood_pool = get_value::<u8>(&data, FieldName::BlutvorratBlutpool, &config);
        let attribut_physical_value =
            get_value::<u8>(&data, FieldName::AttributKörperlichWert, &config);
        let attribut_social_value = get_value::<u8>(&data, FieldName::AttributSozialWert, &config);
        let attribut_mental_value = get_value::<u8>(&data, FieldName::AttributMentalWert, &config);
        let attribut_physical_foci =
            get_value_vec(&data, FieldName::AttributKörperlicheFoki, &config);
        let attribut_social_foci = get_value_vec(&data, FieldName::AttributSozialeFoki, &config);
        let attribut_mental_foci = get_value_vec(&data, FieldName::AttributMentaleFoki, &config);
        let academics_value = get_value::<u8>(&data, FieldName::SkillAkademischesWissen, &config);
        let academics_foci = get_skill_specialization(
            &data,
            FieldName::SkillAkademischesWissenSpezialisierung,
            &config,
        );
        let subterfuge_value = get_value::<u8>(&data, FieldName::SkillAusfluechte, &config);
        let dodge_value = get_value::<u8>(&data, FieldName::SkillAusweichen, &config);
        let computer_value = get_value::<u8>(&data, FieldName::SkillComputer, &config);
        let intimidation_value = get_value::<u8>(&data, FieldName::SkillEinschüchtern, &config);
        let empathy_value = get_value::<u8>(&data, FieldName::SkillEmpathie, &config);
        let drive_value = get_value::<u8>(&data, FieldName::SkillFahren, &config);
        let leadership_value = get_value::<u8>(&data, FieldName::SkillFührungsqualitäten, &config);
        let brawl_value = get_value::<u8>(&data, FieldName::SkillHandgemenge, &config);
        let craft_a_value = get_value::<u8>(&data, FieldName::SkillHandwerkA, &config);
        let craft_a_foci =
            get_skill_specialization(&data, FieldName::SkillHandwerkASpezialisierung, &config);
        let craft_b_value = get_value::<u8>(&data, FieldName::SkillHandwerkB, &config);
        let craft_b_foci =
            get_skill_specialization(&data, FieldName::SkillHandwerkBSpezialisierung, &config);
        let stealth_value = get_value::<u8>(&data, FieldName::SkillHeimlichkeit, &config);
        let linguistics_value = get_value::<u8>(&data, FieldName::SkillLinguistik, &config);
        let linguistics_foci =
            get_skill_specialization(&data, FieldName::SkillLinguistikSpezialisierung, &config);
        let awareness_value = get_value::<u8>(&data, FieldName::SkillMagiegespür, &config);
        let medicine_value = get_value::<u8>(&data, FieldName::SkillMedizin, &config);
        let investigation_value = get_value::<u8>(&data, FieldName::SkillNachforschungen, &config);
        let melee_value = get_value::<u8>(&data, FieldName::SkillNahkampf, &config);
        let science_a_value = get_value::<u8>(&data, FieldName::SkillNaturwissenschaftenA, &config);
        let science_a_foci = get_skill_specialization(
            &data,
            FieldName::SkillNaturwissenschaftenASpezialisierung,
            &config,
        );
        let science_b_value = get_value::<u8>(&data, FieldName::SkillNaturwissenschaftenB, &config);
        let science_b_foci = get_skill_specialization(
            &data,
            FieldName::SkillNaturwissenschaftenBSpezialisierung,
            &config,
        );
        let occult_value = get_value::<u8>(&data, FieldName::SkillOkkultismus, &config);
        let firearms_value = get_value::<u8>(&data, FieldName::SkillSchusswaffen, &config);
        let security_value = get_value::<u8>(&data, FieldName::SkillSicherheit, &config);
        let athletics_value = get_value::<u8>(&data, FieldName::SkillSportlichkeit, &config);
        let streetwise_value = get_value::<u8>(&data, FieldName::SkillSzenekenntnis, &config);
        let animal_ken_value = get_value::<u8>(&data, FieldName::SkillTierkunde, &config);
        let survival_value = get_value::<u8>(&data, FieldName::SkillÜberleben, &config);
        let performance_a_value = get_value::<u8>(&data, FieldName::SkillVortragA, &config);
        let performance_a_foci =
            get_skill_specialization(&data, FieldName::SkillVortragASpezialisierung, &config);
        let performance_b_value = get_value(&data, FieldName::SkillVortragB, &config);
        let performance_b_foci =
            get_skill_specialization(&data, FieldName::SkillVortragBSpezialisierung, &config);
        let lore_value = get_value::<u8>(&data, FieldName::SkillÜbernatürlichesWissen, &config);
        let lore_foci = get_skill_specialization(
            &data,
            FieldName::SkillÜbernatürlichesWissenSpezialisierung,
            &config,
        );
        let in_clan_disciplines = get_disciplines(&data, FieldName::InClanDisziplinen, &config);
        let out_of_clan_disciplines =
            get_disciplines(&data, FieldName::OutOfClanDisziplinen, &config);
        let techniques = get_value_vec(&data, FieldName::Techniken, &config);
        let in_clan_elder_powers = get_value_vec(&data, FieldName::InClanAhnenkräfte, &config);
        let out_of_clan_elder_powers =
            get_value_vec(&data, FieldName::OutOfClanAhnenkräfte, &config);
        let morality_name = get_value(&data, FieldName::MoralvorstellungName, &config);
        let morality_value = get_value::<u8>(&data, FieldName::MoralvorstellungWert, &config);
        let faction_name = get_value(&data, FieldName::FraktionName, &config);
        let merits_and_flaws =
            extract_value_from_vec(&data, FieldName::MeritsFlaws, &config).unwrap();
        let merits = extract_merits(&merits_and_flaws);
        let flaws = extract_flaws(&merits_and_flaws);
        let backgrounds = get_backgrounds(&data, FieldName::Backgrounds, &config);
        let experience_start_value =
            get_value::<u8>(&data, FieldName::ErfahrungspunkteStartpunkte, &config);
        let experience_spent_total =
            get_value::<u16>(&data, FieldName::ErfahrungspunkteGesamtAusgegeben, &config);
        let experience_remaining =
            get_value::<i16>(&data, FieldName::ErfahrungspunkteAktuellFrei, &config);
        let experience_received_total =
            get_value::<u8>(&data, FieldName::ErfahrungspunkteGesamtErhalten, &config);
        let initiative = get_value::<u8>(&data, FieldName::Initiative, &config);
        let initiative_with_celerity =
            get_value::<u8>(&data, FieldName::InitiativeGeschwindigkeit, &config);
        let health_healthy_track = get_health_track(&data, FieldName::GesundheitHealthy, &config);
        let health_injured_track = get_health_track(&data, FieldName::GesundheitInjured, &config);
        let health_incapacitated_track =
            get_health_track(&data, FieldName::GesundheitIncapacitated, &config);
        let physical_defense_base =
            get_value::<u8>(&data, FieldName::VerteidigungKörperlichRegulär, &config);
        let physical_defense_with_celerity = get_value::<u8>(
            &data,
            FieldName::VerteidigungKörperlichMitGeschwindigkeit,
            &config,
        );
        let physical_defense_frenzy_modifier = get_value::<i8>(
            &data,
            FieldName::VerteidigungKörperlichRasereiModifier,
            &config,
        );
        let physical_defense_on_the_ground_closer_than_3_meters = get_value::<i8>(
            &data,
            FieldName::VerteidigungKörperlichAmBodenGegnerNäherAls3Meter,
            &config,
        );
        let physical_defense_on_the_ground_further_than_3_meters = get_value::<u8>(
            &data,
            FieldName::VerteidigungKörperlichAmBodenGegnerMindestens3MeterWeg,
            &config,
        );
        let physical_defense_special =
            get_value::<i8>(&data, FieldName::VerteidigungKörperlichSpecial, &config);
        let social_defense_pool =
            get_non_physical_defense_pool(&data, FieldName::VerteidigungSozial, &config);
        let mental_defense_pool =
            get_non_physical_defense_pool(&data, FieldName::VerteidigungMental, &config);
        let offense_pools = get_attack_pools(&data, FieldName::AngriffsPools, &config);
        let rituals = get_rituals(&data, FieldName::Rituale, &config);
        let items = get_items(&data, FieldName::Items, &config);
        let valid = !experience_spent_total.gt(&900_u16);

        // creating the result struct
        PlayerCharacter {
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
        }
    }
}
//

#[allow(clippy::from_over_into)]
impl Into<Vec<ValueRange>> for PlayerCharacterUpdateInput {
    fn into(self) -> Vec<ValueRange> {
        let mut result: Vec<ValueRange> = vec![];
        let config = ConfigClient::new();

        // Converting the data
        let character_name_option =
            set_value(self.character_name, FieldName::CharacterName, &config);
        let player_name_option = set_value(self.player_name, FieldName::PlayerName, &config);
        let archetype_option = set_value(self.archetype, FieldName::Archetype, &config);
        let generation_option = match self.generation {
            Some(value) => match value {
                Left(a_number) => set_value(Some(a_number), FieldName::Generation, &config),
                Right(a_string) => set_value(Some(a_string), FieldName::Generation, &config),
            },
            None => None,
        };
        let clan_option = set_value(self.clan, FieldName::Clan, &config);

        let mut attribute_physical_value_option: Option<ValueRange> = None;
        let mut attribute_social_value_option: Option<ValueRange> = None;
        let mut attribute_mental_value_option: Option<ValueRange> = None;

        let mut attribute_physical_specialization_option: Option<ValueRange> = None;
        let mut attribute_social_specialization_option: Option<ValueRange> = None;
        let mut attribute_mental_specialization_option: Option<ValueRange> = None;

        if let Some(attributes) = self.attributes {
            if let Some(physical_attribute) = attributes.physical {
                attribute_physical_value_option = set_value(
                    physical_attribute.value,
                    FieldName::AttributKörperlichWert,
                    &config,
                );

                attribute_physical_specialization_option = set_value_vec(
                    physical_attribute.foci,
                    FieldName::AttributKörperlicheFoki,
                    &config,
                    None,
                );
            }

            if let Some(social_attribute) = attributes.social {
                attribute_social_value_option = set_value(
                    social_attribute.value,
                    FieldName::AttributSozialWert,
                    &config,
                );

                attribute_social_specialization_option = set_value_vec(
                    social_attribute.foci,
                    FieldName::AttributSozialeFoki,
                    &config,
                    None,
                );
            }

            if let Some(mental_attribute) = attributes.mental {
                attribute_mental_value_option = set_value(
                    mental_attribute.value,
                    FieldName::AttributMentalWert,
                    &config,
                );

                attribute_mental_specialization_option = set_value_vec(
                    mental_attribute.foci,
                    FieldName::AttributMentaleFoki,
                    &config,
                    None,
                );
            }
        }

        let mut skill_academics_value_option: Option<ValueRange> = None;
        let mut skill_academics_specialization_option: Option<ValueRange> = None;
        let mut skill_animal_ken_value_option: Option<ValueRange> = None;
        let mut skill_athletics_value_option: Option<ValueRange> = None;
        let mut skill_awareness_value_option: Option<ValueRange> = None;
        let mut skill_brawl_value_option: Option<ValueRange> = None;
        let mut skill_computer_value_option: Option<ValueRange> = None;
        let mut skill_craft_a_value_option: Option<ValueRange> = None;
        let mut skill_craft_a_specialization_option: Option<ValueRange> = None;
        let mut skill_craft_b_value_option: Option<ValueRange> = None;
        let mut skill_craft_b_specialization_option: Option<ValueRange> = None;
        let mut skill_dodge_value_option: Option<ValueRange> = None;
        let mut skill_drive_value_option: Option<ValueRange> = None;
        let mut skill_empathy_value_option: Option<ValueRange> = None;
        let mut skill_firearms_value_option: Option<ValueRange> = None;
        let mut skill_intimidation_value_option: Option<ValueRange> = None;
        let mut skill_investigation_value_option: Option<ValueRange> = None;
        let mut skill_leadership_value_option: Option<ValueRange> = None;
        let mut skill_linguistics_value_option: Option<ValueRange> = None;
        let mut skill_linguistics_specialization_option: Option<ValueRange> = None;
        let mut skill_lore_value_option: Option<ValueRange> = None;
        let mut skill_lore_specialization_option: Option<ValueRange> = None;
        let mut skill_medicine_value_option: Option<ValueRange> = None;
        let mut skill_melee_value_option: Option<ValueRange> = None;
        let mut skill_occult_value_option: Option<ValueRange> = None;
        let mut skill_performance_a_value_option: Option<ValueRange> = None;
        let mut skill_performance_a_specialization_option: Option<ValueRange> = None;
        let mut skill_performance_b_value_option: Option<ValueRange> = None;
        let mut skill_performance_b_specialization_option: Option<ValueRange> = None;
        let mut skill_security_option: Option<ValueRange> = None;
        let mut skill_science_a_value_option: Option<ValueRange> = None;
        let mut skill_science_a_specialization_option: Option<ValueRange> = None;
        let mut skill_science_b_value_option: Option<ValueRange> = None;
        let mut skill_science_b_specialization_option: Option<ValueRange> = None;
        let mut skill_stealth_value_option: Option<ValueRange> = None;
        let mut skill_streetwise_value_option: Option<ValueRange> = None;
        let mut skill_subterfuge_value_option: Option<ValueRange> = None;
        let mut skill_survival_value_option: Option<ValueRange> = None;

        if let Some(skills) = self.skills {
            if let Some(academics) = skills.academics {
                skill_academics_value_option =
                    set_value(academics.value, FieldName::SkillAkademischesWissen, &config);
                skill_academics_specialization_option = set_skill_specialization(
                    academics.foci,
                    FieldName::SkillAkademischesWissenSpezialisierung,
                    &config,
                );
            }

            if let Some(animal_ken) = skills.animal_ken {
                skill_animal_ken_value_option =
                    set_value(animal_ken.value, FieldName::SkillTierkunde, &config)
            }

            if let Some(athletics) = skills.athletics {
                skill_athletics_value_option =
                    set_value(athletics.value, FieldName::SkillSportlichkeit, &config)
            }

            if let Some(awareness) = skills.awareness {
                skill_awareness_value_option =
                    set_value(awareness.value, FieldName::SkillMagiegespür, &config)
            }

            if let Some(brawl) = skills.brawl {
                skill_brawl_value_option =
                    set_value(brawl.value, FieldName::SkillHandgemenge, &config)
            }

            if let Some(computer) = skills.computer {
                skill_computer_value_option =
                    set_value(computer.value, FieldName::SkillComputer, &config)
            }

            if let Some(craft_a) = skills.craft_a {
                skill_craft_a_value_option =
                    set_value(craft_a.value, FieldName::SkillHandwerkA, &config);
                skill_craft_a_specialization_option = set_skill_specialization(
                    craft_a.foci,
                    FieldName::SkillHandwerkASpezialisierung,
                    &config,
                );
            }

            if let Some(craft_b) = skills.craft_b {
                skill_craft_b_value_option =
                    set_value(craft_b.value, FieldName::SkillHandwerkB, &config);
                skill_craft_b_specialization_option = set_skill_specialization(
                    craft_b.foci,
                    FieldName::SkillHandwerkBSpezialisierung,
                    &config,
                );
            }

            if let Some(dodge) = skills.dodge {
                skill_dodge_value_option =
                    set_value(dodge.value, FieldName::SkillAusweichen, &config)
            }

            if let Some(drive) = skills.drive {
                skill_drive_value_option = set_value(drive.value, FieldName::SkillFahren, &config)
            }

            if let Some(empathy) = skills.empathy {
                skill_empathy_value_option =
                    set_value(empathy.value, FieldName::SkillEmpathie, &config)
            }

            if let Some(firearms) = skills.firearms {
                skill_firearms_value_option =
                    set_value(firearms.value, FieldName::SkillSchusswaffen, &config)
            }

            if let Some(intimidation) = skills.intimidation {
                skill_intimidation_value_option =
                    set_value(intimidation.value, FieldName::SkillEinschüchtern, &config)
            }

            if let Some(investigation) = skills.investigation {
                skill_investigation_value_option = set_value(
                    investigation.value,
                    FieldName::SkillNachforschungen,
                    &config,
                )
            }

            if let Some(leadership) = skills.leadership {
                skill_leadership_value_option = set_value(
                    leadership.value,
                    FieldName::SkillFührungsqualitäten,
                    &config,
                )
            }

            if let Some(linguistics) = skills.linguistics {
                skill_linguistics_value_option =
                    set_value(linguistics.value, FieldName::SkillLinguistik, &config);
                skill_linguistics_specialization_option = set_skill_specialization(
                    linguistics.foci,
                    FieldName::SkillLinguistikSpezialisierung,
                    &config,
                );
            }

            if let Some(lore) = skills.lore {
                skill_lore_value_option =
                    set_value(lore.value, FieldName::SkillÜbernatürlichesWissen, &config);
                skill_lore_specialization_option = set_skill_specialization(
                    lore.foci,
                    FieldName::SkillÜbernatürlichesWissenSpezialisierung,
                    &config,
                );
            }

            if let Some(medicine) = skills.medicine {
                skill_medicine_value_option =
                    set_value(medicine.value, FieldName::SkillMedizin, &config)
            }

            if let Some(melee) = skills.melee {
                skill_melee_value_option = set_value(melee.value, FieldName::SkillNahkampf, &config)
            }

            if let Some(occult) = skills.occult {
                skill_occult_value_option =
                    set_value(occult.value, FieldName::SkillOkkultismus, &config)
            }

            if let Some(performance_a) = skills.performance_a {
                skill_performance_a_value_option =
                    set_value(performance_a.value, FieldName::SkillVortragA, &config);
                skill_performance_a_specialization_option = set_skill_specialization(
                    performance_a.foci,
                    FieldName::SkillVortragASpezialisierung,
                    &config,
                );
            }

            if let Some(performance_b) = skills.performance_b {
                skill_performance_b_value_option =
                    set_value(performance_b.value, FieldName::SkillVortragB, &config);
                skill_performance_b_specialization_option = set_skill_specialization(
                    performance_b.foci,
                    FieldName::SkillVortragBSpezialisierung,
                    &config,
                );
            }

            if let Some(security) = skills.security {
                skill_security_option =
                    set_value(security.value, FieldName::SkillSicherheit, &config)
            }

            if let Some(science_a) = skills.science_a {
                skill_science_a_value_option = set_value(
                    science_a.value,
                    FieldName::SkillNaturwissenschaftenA,
                    &config,
                );
                skill_science_a_specialization_option = set_skill_specialization(
                    science_a.foci,
                    FieldName::SkillNaturwissenschaftenASpezialisierung,
                    &config,
                );
            }

            if let Some(science_b) = skills.science_b {
                skill_science_b_value_option = set_value(
                    science_b.value,
                    FieldName::SkillNaturwissenschaftenB,
                    &config,
                );
                skill_science_b_specialization_option = set_skill_specialization(
                    science_b.foci,
                    FieldName::SkillNaturwissenschaftenBSpezialisierung,
                    &config,
                );
            }

            if let Some(stealth) = skills.stealth {
                skill_stealth_value_option =
                    set_value(stealth.value, FieldName::SkillHeimlichkeit, &config)
            }

            if let Some(streetwise) = skills.streetwise {
                skill_streetwise_value_option =
                    set_value(streetwise.value, FieldName::SkillSzenekenntnis, &config)
            }

            if let Some(subterfuge) = skills.subterfuge {
                skill_subterfuge_value_option =
                    set_value(subterfuge.value, FieldName::SkillAusfluechte, &config)
            }

            if let Some(survival) = skills.survival {
                skill_survival_value_option =
                    set_value(survival.value, FieldName::SkillÜberleben, &config)
            }
        }

        let mut power_in_clan_disciplines_option: Option<ValueRange> = None;
        let mut power_out_of_clan_disciplines_option: Option<ValueRange> = None;
        let mut power_in_clan_elder_powers_option: Option<ValueRange> = None;
        let mut power_out_of_clan_elder_powers_option: Option<ValueRange> = None;
        let mut power_techniques_option: Option<ValueRange> = None;

        if let Some(powers) = self.powers {
            power_in_clan_disciplines_option = set_disciplines(
                powers.in_clan_disciplines,
                FieldName::InClanDisziplinen,
                &config,
            );

            power_out_of_clan_disciplines_option = set_disciplines(
                powers.out_of_clan_disciplines,
                FieldName::OutOfClanDisziplinen,
                &config,
            );

            power_in_clan_elder_powers_option = set_value_vec(
                powers.in_clan_elder_powers,
                FieldName::InClanAhnenkräfte,
                &config,
                None,
            );

            power_out_of_clan_elder_powers_option = set_value_vec(
                powers.out_of_clan_elder_powers,
                FieldName::OutOfClanAhnenkräfte,
                &config,
                None,
            );

            power_techniques_option = set_value_vec(
                powers.techniques,
                FieldName::Techniken,
                &config,
                Some("".to_string()),
            );
        }

        let mut morality_name_option: Option<ValueRange> = None;
        let mut morality_value_option: Option<ValueRange> = None;

        if let Some(morality) = self.morality {
            morality_name_option =
                set_value(morality.name, FieldName::MoralvorstellungName, &config);
            morality_value_option =
                set_value(morality.value, FieldName::MoralvorstellungWert, &config);
        }

        let faction_name_option: Option<ValueRange> =
            set_value(self.faction, FieldName::FraktionName, &config);

        let mut merits_and_flaws_option: Option<ValueRange> = None;

        if self.merits.is_some() || self.flaws.is_some() {
            merits_and_flaws_option = combine_merits_and_flaws(
                self.merits,
                self.flaws,
                FieldName::MeritsFlawsName,
                &config,
            )
        }

        let backgrounds_option: Option<ValueRange> =
            set_backgrounds(self.backgrounds, FieldName::Backgrounds, &config);

        let mut experience_start_value_option: Option<ValueRange> = None;

        if let Some(experience_information) = self.experience_information {
            experience_start_value_option = set_value(
                Some(experience_information.start_value),
                FieldName::ErfahrungspunkteStartpunkte,
                &config,
            )
        }

        let rituals_option: Option<ValueRange> =
            set_rituals(self.rituals, FieldName::Rituale, &config);

        // Checking if data was present, if not then we won't include that field in the payload for the google spreadsheet
        if let Some(character_name) = character_name_option {
            result.push(character_name)
        };

        if let Some(player_name) = player_name_option {
            result.push(player_name)
        };

        if let Some(archetype) = archetype_option {
            result.push(archetype)
        }

        if let Some(generation) = generation_option {
            result.push(generation)
        }

        if let Some(clan) = clan_option {
            result.push(clan)
        }

        if let Some(attribute_physical_value) = attribute_physical_value_option {
            result.push(attribute_physical_value)
        }

        if let Some(attribute_social_value) = attribute_social_value_option {
            result.push(attribute_social_value)
        }

        if let Some(attribute_mental_value) = attribute_mental_value_option {
            result.push(attribute_mental_value)
        }

        if let Some(attribute_physical_foki) = attribute_physical_specialization_option {
            result.push(attribute_physical_foki)
        }

        if let Some(attribute_social_foki) = attribute_social_specialization_option {
            result.push(attribute_social_foki)
        }

        if let Some(attribute_mental_foki) = attribute_mental_specialization_option {
            result.push(attribute_mental_foki)
        }

        if let Some(skill_academics_value) = skill_academics_value_option {
            result.push(skill_academics_value)
        }

        if let Some(skill_academics_foki) = skill_academics_specialization_option {
            result.push(skill_academics_foki)
        }

        if let Some(skill_animal_ken_value) = skill_animal_ken_value_option {
            result.push(skill_animal_ken_value)
        }

        if let Some(skill_athletics_value) = skill_athletics_value_option {
            result.push(skill_athletics_value)
        }

        if let Some(skill_awareness_value) = skill_awareness_value_option {
            result.push(skill_awareness_value)
        }

        if let Some(skill_brawl_value) = skill_brawl_value_option {
            result.push(skill_brawl_value)
        }

        if let Some(skill_computer_value) = skill_computer_value_option {
            result.push(skill_computer_value)
        }

        if let Some(skill_craft_a_value) = skill_craft_a_value_option {
            result.push(skill_craft_a_value)
        }

        if let Some(skill_craft_a_foki) = skill_craft_a_specialization_option {
            result.push(skill_craft_a_foki)
        }

        if let Some(skill_craft_b_value) = skill_craft_b_value_option {
            result.push(skill_craft_b_value)
        }

        if let Some(skill_craft_b_foki) = skill_craft_b_specialization_option {
            result.push(skill_craft_b_foki)
        }

        if let Some(skill_dodge_value) = skill_dodge_value_option {
            result.push(skill_dodge_value)
        }

        if let Some(skill_drive_value) = skill_drive_value_option {
            result.push(skill_drive_value)
        }

        if let Some(skill_empathy_value) = skill_empathy_value_option {
            result.push(skill_empathy_value)
        }

        if let Some(skill_firearms_value) = skill_firearms_value_option {
            result.push(skill_firearms_value)
        }

        if let Some(skill_intimidation_value) = skill_intimidation_value_option {
            result.push(skill_intimidation_value)
        }

        if let Some(skill_investigation_value) = skill_investigation_value_option {
            result.push(skill_investigation_value)
        }

        if let Some(skill_leadership_value) = skill_leadership_value_option {
            result.push(skill_leadership_value)
        }

        if let Some(skill_linguistics_value) = skill_linguistics_value_option {
            result.push(skill_linguistics_value)
        }

        if let Some(skill_linguistics_foki) = skill_linguistics_specialization_option {
            result.push(skill_linguistics_foki)
        }

        if let Some(skill_lore_value) = skill_lore_value_option {
            result.push(skill_lore_value)
        }

        if let Some(skill_lore_foki) = skill_lore_specialization_option {
            result.push(skill_lore_foki)
        }

        if let Some(skill_medicine_value) = skill_medicine_value_option {
            result.push(skill_medicine_value)
        }

        if let Some(skill_melee_value) = skill_melee_value_option {
            result.push(skill_melee_value)
        }

        if let Some(skill_occult_value) = skill_occult_value_option {
            result.push(skill_occult_value)
        }

        if let Some(skill_performance_a_value) = skill_performance_a_value_option {
            result.push(skill_performance_a_value)
        }

        if let Some(skill_performance_a_foki) = skill_performance_a_specialization_option {
            result.push(skill_performance_a_foki)
        }

        if let Some(skill_performance_b_value) = skill_performance_b_value_option {
            result.push(skill_performance_b_value)
        }

        if let Some(skill_performance_b_foki) = skill_performance_b_specialization_option {
            result.push(skill_performance_b_foki)
        }

        if let Some(skill_security_value) = skill_security_option {
            result.push(skill_security_value)
        }

        if let Some(skill_science_a_value) = skill_science_a_value_option {
            result.push(skill_science_a_value)
        }

        if let Some(skill_science_a_foki) = skill_science_a_specialization_option {
            result.push(skill_science_a_foki)
        }

        if let Some(skill_science_b_value) = skill_science_b_value_option {
            result.push(skill_science_b_value)
        }

        if let Some(skill_science_b_foki) = skill_science_b_specialization_option {
            result.push(skill_science_b_foki)
        }

        if let Some(skill_stealth_value) = skill_stealth_value_option {
            result.push(skill_stealth_value)
        }

        if let Some(skill_streetwise_value) = skill_streetwise_value_option {
            result.push(skill_streetwise_value)
        }

        if let Some(skill_subterfuge_value) = skill_subterfuge_value_option {
            result.push(skill_subterfuge_value)
        }

        if let Some(skill_survival_value) = skill_survival_value_option {
            result.push(skill_survival_value)
        }

        if let Some(in_clan_disciplines) = power_in_clan_disciplines_option {
            result.push(in_clan_disciplines)
        }

        if let Some(out_of_clan_disciplines) = power_out_of_clan_disciplines_option {
            result.push(out_of_clan_disciplines)
        }

        if let Some(in_clan_elder_powers) = power_in_clan_elder_powers_option {
            result.push(in_clan_elder_powers)
        }

        if let Some(out_of_clan_elder_powers) = power_out_of_clan_elder_powers_option {
            result.push(out_of_clan_elder_powers)
        }

        if let Some(techniques) = power_techniques_option {
            result.push(techniques)
        }

        if let Some(morality_name) = morality_name_option {
            result.push(morality_name)
        }

        if let Some(morality_value) = morality_value_option {
            result.push(morality_value)
        }

        if let Some(faction_name) = faction_name_option {
            result.push(faction_name)
        }

        if let Some(merits_and_flaws) = merits_and_flaws_option {
            result.push(merits_and_flaws)
        }

        if let Some(backgrounds) = backgrounds_option {
            result.push(backgrounds);
        }

        if let Some(experience_start_value) = experience_start_value_option {
            result.push(experience_start_value)
        }

        if let Some(ritual_values) = rituals_option {
            result.push(ritual_values)
        }

        result
    }
}

/// Takes care of retrieving the value from the passed in result range
fn extract_value_from_vec(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<Vec<Vec<String>>> {
    value_range
        .get(sheet_config.get_field_config(field_name).position as usize)
        .unwrap()
        .clone()
        .values
}

/// creates a standardized value range
fn create_value_range(
    values: Option<Vec<Vec<String>>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> ValueRange {
    ValueRange {
        major_dimension: Some("ROWS".to_string()),
        range: Some(sheet_config.get_field_config(field_name).range),
        values,
    }
}

/// reads a single cell
fn get_value<T>(value_range: &[ValueRange], field_name: FieldName, sheet_config: &ConfigClient) -> T
where
    T: FromStr + Default,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let binding = extract_value_from_vec(value_range, field_name, sheet_config)
        .unwrap_or_else(|| vec![vec!["-".to_string()]]);
    let field = binding.get(0).unwrap().get(0).unwrap().clone();
    field.parse::<T>().unwrap_or_default()
}

/// sets a value in a single cell
fn set_value<T>(
    value: Option<T>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange>
where
    T: FromStr + Display,
{
    match value {
        Some(val) => {
            let mut value_to_set: String = val.to_string();

            if value_to_set == *"0" {
                value_to_set = "-".to_string();
            }

            Some(create_value_range(
                Some(vec![vec![value_to_set]]),
                field_name,
                sheet_config,
            ))
        }
        None => None,
    }
}

/// extracts a vec
fn get_value_vec(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<String> {
    let data = extract_value_from_vec(value_range, field_name, sheet_config);
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

fn set_value_vec<T>(
    value: Option<Vec<T>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
    empty_value: Option<String>,
) -> Option<ValueRange>
where
    T: FromStr + Display,
{
    let config = sheet_config.get_field_config(field_name.clone());
    let empty_value_string = empty_value.unwrap_or_else(|| "-".to_string());

    match value {
        Some(val) => {
            let mut result: Vec<Vec<String>> = Vec::new();

            for n in 0..config.range_length.unwrap_or(1) {
                if let Some(content) = val.get(n as usize) {
                    result.push(vec![content.to_string()])
                } else {
                    result.push(vec![empty_value_string.clone()])
                }
            }

            Some(create_value_range(Some(result), field_name, sheet_config))
        }
        None => None,
    }
}

/// reads a skill specialization
fn get_skill_specialization(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<Vec<String>> {
    Some(
        get_value::<String>(value_range, field_name, sheet_config)
            .split(',')
            .map(|x| x.trim().to_string())
            .filter_map(|x| match x != "-" {
                true => Some(x),
                false => None,
            })
            .collect(),
    )
}

// sets a skill specialization
fn set_skill_specialization(
    value: Option<Vec<String>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange> {
    match value {
        Some(val) => {
            let mut value_to_set: String = val.join(", ");
            if value_to_set.is_empty() {
                value_to_set = "-".to_string();
            }

            Some(create_value_range(
                Some(vec![vec![value_to_set]]),
                field_name,
                sheet_config,
            ))
        }
        None => None,
    }
}

/// reads the list of disciplines
fn get_disciplines(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<Discipline> {
    extract_value_from_vec(value_range, field_name, sheet_config)
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

/// sets the list of disciplines
fn set_disciplines(
    value: Option<Vec<DisciplineUpdateInput>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange> {
    let mut data_to_write: Vec<Vec<String>> = Vec::new();
    match value {
        Some(val) => {
            for entry in val {
                let mut row_vec: Vec<String> = vec!["".to_string(); 8];
                row_vec[0] = entry.name;
                row_vec[7] = match entry.value {
                    0 => "-".to_string(),
                    x => x.to_string(),
                };
                data_to_write.push(row_vec);
            }

            // Clearing (possibly) remaining entries in the discipline list in the sheet
            for _ in data_to_write.iter().len()..6 {
                let mut row_vec: Vec<String> = vec!["".to_string(); 8];
                row_vec[0] = "-".to_string();
                row_vec[7] = "-".to_string();
                data_to_write.push(row_vec);
            }

            Some(create_value_range(
                Some(data_to_write),
                field_name,
                sheet_config,
            ))
        }
        None => None,
    }
}

/// reads the merits of the combined list
fn extract_merits(input: &[Vec<String>]) -> Vec<Merit> {
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

/// reads the flaws of the combined list
fn extract_flaws(input: &[Vec<String>]) -> Vec<Flaw> {
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

fn combine_merits_and_flaws(
    merits: Option<Vec<MeritUpdateInput>>,
    flaws: Option<Vec<FlawUpdateInput>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange> {
    let mut data_to_write: Vec<Vec<String>> = Vec::new();
    let config_entry = sheet_config.get_field_config(field_name.clone());

    if let Some(val) = merits {
        for entry in val {
            if entry.name.trim().starts_with("Clan") {
                continue;
            }

            let mut row_vec: Vec<String> = Vec::new();

            let merit_type = match entry.merit_type.starts_with("General") {
                true => "".to_string(),
                false => format!("{}: ", entry.merit_type),
            };

            row_vec.push(format!("V {}{}", merit_type, entry.name));

            data_to_write.push(row_vec);
        }
    }

    if let Some(val) = flaws {
        for entry in val {
            let mut row_vec: Vec<String> = Vec::new();

            let flaw_type = match entry.flaw_type.starts_with("General") {
                true => "".to_string(),
                false => format!("{}: ", entry.flaw_type),
            };

            row_vec.push(format!("N {}{}", flaw_type, entry.name));
            data_to_write.push(row_vec);
        }
    }

    // Clearing (possibly) remaining entries in the discipline list in the sheet
    for _ in data_to_write.iter().len()..=config_entry.range_length.unwrap() as usize {
        let row_vec: Vec<String> = vec!["-".to_string()];
        data_to_write.push(row_vec);
    }

    Some(create_value_range(
        Some(data_to_write),
        field_name,
        sheet_config,
    ))
}

/// reads the list of backgrounds
fn get_backgrounds(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<Background> {
    extract_value_from_vec(value_range, field_name, sheet_config)
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

/// converts the list of backgrounds into a value range
fn set_backgrounds(
    value: Option<Vec<BackgroundUpdateInput>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange> {
    let mut data_to_write: Vec<Vec<String>> = Vec::new();
    let config_entry = sheet_config.get_field_config(field_name.clone());

    match value {
        Some(val) => {
            for entry in val {
                let mut row_vec: Vec<String> = vec!["".to_string(); 10];
                row_vec[0] = entry.name;
                row_vec[7] = match entry.value {
                    0 => "-".to_string(),
                    x => x.to_string(),
                };
                row_vec[9] = match entry.description {
                    Some(value) => value,
                    None => "".to_string(),
                };
                data_to_write.push(row_vec);
            }

            // Clearing (possibly) remaining entries in the discipline list in the sheet
            for n in data_to_write.iter().len()..=config_entry.range_length.unwrap() as usize {
                let mut row_vec: Vec<String> = vec!["".to_string(); 10];
                row_vec[7] = "-".to_string();
                data_to_write.insert(n, row_vec);
            }

            Some(create_value_range(
                Some(data_to_write),
                field_name,
                sheet_config,
            ))
        }
        None => None,
    }
}

/// reads a health track information
fn get_health_track(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> HealthTrack {
    let binding = extract_value_from_vec(value_range, field_name, sheet_config).unwrap();
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
fn get_non_physical_defense_pool(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> HashMap<u8, u8> {
    let mut result: HashMap<u8, u8> = HashMap::new();
    let values = extract_value_from_vec(value_range, field_name, sheet_config)
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
fn get_attack_pools(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<BattleOffenseInformation> {
    let mut result: Vec<BattleOffenseInformation> = Vec::new();
    let binding = extract_value_from_vec(value_range, field_name, sheet_config).unwrap();
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
fn get_rituals(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<Ritual> {
    let mut result: Vec<Ritual> = Vec::new();
    let binding = extract_value_from_vec(value_range, field_name, sheet_config).unwrap();
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

/// sets the rituals list
fn set_rituals(
    value: Option<Vec<RitualUpdateInput>>,
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Option<ValueRange> {
    let mut data_to_write: Vec<Vec<String>> = Vec::new();
    let config_entry = sheet_config.get_field_config(field_name.clone());

    if let Some(val) = value {
        for entry in val {
            let mut row_vec: Vec<String> = Vec::new();

            let ritual_type = match entry.ritual_type.as_ref() {
                "Abyssal" => 'A',
                "Necromancy" => 'N',
                "Thaumaturgy" => 'T',
                _ => 'U',
            };

            row_vec.push(format!("{}{} {}", ritual_type, entry.level, entry.name));

            data_to_write.push(row_vec);
        }
    }

    // Clearing (possibly) remaining entries in the discipline list in the sheet
    for _ in data_to_write.iter().len()..=config_entry.range_length.unwrap() as usize {
        let row_vec: Vec<String> = vec!["-".to_string()];
        data_to_write.push(row_vec);
    }

    Some(create_value_range(
        Some(data_to_write),
        field_name,
        sheet_config,
    ))
}

/// reads the items section
fn get_items(
    value_range: &[ValueRange],
    field_name: FieldName,
    sheet_config: &ConfigClient,
) -> Vec<Item> {
    let mut result: Vec<Item> = Vec::new();
    let binding = extract_value_from_vec(value_range, field_name, sheet_config).unwrap_or_default();

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
