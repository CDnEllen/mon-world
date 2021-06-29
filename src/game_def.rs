use std::collections::HashMap;

#[derive(Debug)]
pub struct MonDef {
    id: String,
    species: String,
    types: Vec<String>,
    abilities: Vec<String>,
    evos: Vec<String>,
    learnset: Vec<(i64, String)>,
    base_exp: i64,

    hp: i64,
    atk: i64,
    spatk: i64,
    def: i64,
    spdef: i64,

    height: f64,
    weight: f64,
}

pub fn build_mon_defs(src: &toml::Value) -> HashMap<String, MonDef> {
    let mut defs = HashMap::new();

    src.as_array()
        .unwrap()
        .iter()
        .map(|mon| mon.as_table().unwrap())
        .for_each(|mon| {
            let id = mon["id"].as_str().unwrap().to_owned();
            let species = mon["species"].as_str().unwrap().to_owned();
            let types = mon["types"]
                .as_array()
                .map(|vals| {
                    vals.iter()
                        .map(toml::Value::as_str)
                        .map(Option::unwrap)
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap();
            let abilities = mon["abilities"]
                .as_array()
                .map(|vals| {
                    vals.iter()
                        .map(toml::Value::as_str)
                        .map(Option::unwrap)
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap();
            let evos = mon["evolutions"]
                .as_array()
                .map(|vals| {
                    vals.iter()
                        .map(toml::Value::as_str)
                        .map(Option::unwrap)
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap();
            let learnset = mon["possible_moves"]
                .as_array()
                .map(|moves| {
                    moves
                        .iter()
                        .map(toml::Value::as_array)
                        .map(Option::unwrap)
                        .map(|move_| {
                            unwrap_matches!(
                                move_.as_slice(),
                                [lvl, move_name] => (
                                    lvl.as_integer().unwrap(),
                                    move_name.as_str().unwrap().to_owned()
                                )
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap();

            let base_exp = mon["base_exp"].as_integer().unwrap();

            let base_stats = mon["base_stats"].as_table().unwrap();

            let hp = base_stats["hp"].as_integer().unwrap();
            let atk = base_stats["atk"].as_integer().unwrap();
            let spatk = base_stats["spatk"].as_integer().unwrap();
            let def = base_stats["def"].as_integer().unwrap();
            let spdef = base_stats["spdef"].as_integer().unwrap();

            let height = mon["weight"].as_float().unwrap();
            let weight = mon["height"].as_float().unwrap();

            let def = MonDef {
                id: id.clone(),
                species,
                types,
                abilities,
                evos,
                learnset,
                base_exp,
                hp,
                atk,
                spatk,
                def,
                spdef,
                height,
                weight,
            };

            defs.insert(id, def);
        });

    defs
}

#[derive(Debug)]
pub struct MonInstance {
    id: String,
    def: String,
    nickname: String,
    ability: String,
    level: i64,
    current_moves: Vec<String>,
}

pub fn build_mon_instances(src: &toml::Value) -> HashMap<String, MonInstance> {
    let mut instances = HashMap::new();

    src.as_array()
        .unwrap()
        .iter()
        .map(|mon| mon.as_table().unwrap())
        .for_each(|mon| {
            let id = mon["id"].as_str().unwrap().to_owned();
            let def = mon["def"].as_str().unwrap().to_owned();
            let nickname = mon["nickname"].as_str().unwrap().to_owned();
            let ability = mon["ability"].as_str().unwrap().to_owned();
            let level = mon["level"].as_integer().unwrap();
            let current_moves = mon["current_moves"]
                .as_array()
                .map(|moves| {
                    moves
                        .iter()
                        .map(toml::Value::as_str)
                        .map(Option::unwrap)
                        .map(ToOwned::to_owned)
                        .collect::<Vec<_>>()
                })
                .unwrap();

            let instance = MonInstance {
                id: id.clone(),
                def,
                nickname,
                ability,
                level,
                current_moves,
            };
            instances.insert(id, instance);
        });

    instances
}

#[derive(Debug)]
pub enum Effectiveness {
    Double,
    One,
    Half,
}

#[derive(Debug)]
pub struct TypeChart(HashMap<String, HashMap<String, Effectiveness>>);

pub fn build_type_chart(src: &toml::Value) -> TypeChart {
    let mut chart = HashMap::new();

    let types = src
        .as_table()
        .unwrap()
        .iter()
        .map(|(type_name, _)| type_name)
        .collect::<Vec<_>>();

    for &ty in &types {
        let effectivenesses = chart.entry(ty.to_owned()).or_insert_with(HashMap::new);
        for &ty in &types {
            effectivenesses.insert(ty.to_owned(), Effectiveness::One);
        }
    }

    src.as_table()
        .unwrap()
        .iter()
        .for_each(|(type_name, data)| {
            let effectivity = data.as_table().unwrap();
            // if type_name == "fire"
            // then "weak_to = ["water"]
            effectivity["weak_to"]
                .as_array()
                .unwrap()
                .iter()
                .map(|ty| ty.as_str().unwrap().to_owned())
                .for_each(|ty| {
                    // chart["water"]["fire"] == Effectiveness::Double
                    chart
                        .get_mut(&ty)
                        .unwrap()
                        .insert(type_name.to_owned(), Effectiveness::Double);
                });
            effectivity["resistant_to"]
                .as_array()
                .unwrap()
                .iter()
                .map(|ty| ty.as_str().unwrap().to_owned())
                .for_each(|ty| {
                    chart
                        .get_mut(&ty)
                        .unwrap()
                        .insert(type_name.to_owned(), Effectiveness::Half);
                });
        });

    TypeChart(chart)
}

#[derive(Debug)]
pub enum Category {
    Physical,
    Special,
    Status,
}

#[derive(Debug)]
pub struct TargetType {
    targets_self: bool,
    target_type: TargetTypeModSelf,
}

#[derive(Debug)]
pub enum TargetTypeModSelf {
    OneFoe,
    AllFoes,
    OneAlly,
    AllAllies,
    OneMon,
    AllMons,
    Arena,
    None,
}

#[derive(Debug)]
pub struct MoveDef {
    id: String,
    name: String,
    accuracy: i64,
    base_power: i64,
    category: Category,
    priority: i64,
    target: TargetType,
    move_type: String,

    atk_boost: i64,
    spatk_boost: i64,
    def_boost: i64,
    spdef_boost: i64,
}

pub fn build_moves(src: &toml::Value) -> HashMap<String, MoveDef> {
    let mut moves = HashMap::new();

    src.as_array().unwrap().iter().for_each(|move_| {
        let move_ = move_.as_table().unwrap();
        let id = move_["id"].as_str().unwrap().to_owned();
        let name = move_["name"].as_str().unwrap().to_owned();
        let accuracy = move_["accuracy"].as_integer().unwrap();
        let base_power = move_["base_power"].as_integer().unwrap();
        let category = match move_["category"].as_str().unwrap() {
            "physical" => Category::Physical,
            "special" => Category::Special,
            "status" => Category::Status,
            _ => panic!(""),
        };
        let priority = move_["priority"].as_integer().unwrap();
        assert!(priority < 6);

        let (targets_self, target_type) = if let Some(target) = move_["target"].as_str() {
            (None, target)
        } else if let Some(target) = move_["target"].as_array() {
            assert!(target.len() == 2);
            (
                Some(target[0].as_bool().unwrap()),
                target[1].as_str().unwrap(),
            )
        } else {
            panic!("")
        };
        let target = TargetType {
            targets_self: targets_self.unwrap_or(false),
            target_type: match target_type {
                "one_foe" => TargetTypeModSelf::OneFoe,
                "all_foes" => TargetTypeModSelf::AllFoes,
                "one_ally" => TargetTypeModSelf::OneAlly,
                "all_allies" => TargetTypeModSelf::AllAllies,
                "one_mon" => TargetTypeModSelf::OneMon,
                "all_mons" => {
                    assert!(targets_self.is_some(), "write out a bool explicitly pls");
                    TargetTypeModSelf::AllMons
                }
                "arena" => {
                    assert!(targets_self.is_some(), "write out a bool explicitly pls");
                    TargetTypeModSelf::Arena
                }
                "" => {
                    assert!(targets_self.is_some(), "write out a bool explicitly pls");
                    TargetTypeModSelf::None
                }
                _ => panic!(""),
            },
        };
        let move_type = move_["type"].as_str().unwrap().to_owned();

        dbg!(move_.get("boosts"));
        let (atk_boost, spatk_boost, def_boost, spdef_boost) =
            match move_.get("boosts").and_then(toml::Value::as_table) {
                Some(boosts) => (
                    boosts
                        .get("atk")
                        .and_then(toml::Value::as_integer)
                        .unwrap_or(0),
                    boosts
                        .get("spatk")
                        .and_then(toml::Value::as_integer)
                        .unwrap_or(0),
                    boosts
                        .get("def")
                        .and_then(toml::Value::as_integer)
                        .unwrap_or(0),
                    boosts
                        .get("spdef")
                        .and_then(toml::Value::as_integer)
                        .unwrap_or(0),
                ),
                None => (0, 0, 0, 0),
            };

        let move_ = MoveDef {
            id: id.clone(),
            name,
            accuracy,
            base_power,
            category,
            priority,
            target,
            move_type,

            atk_boost,
            spatk_boost,
            def_boost,
            spdef_boost,
        };
        moves.insert(id, move_);
    });

    moves
}

#[derive(Debug)]
pub struct EncounterTable {
    id: String,
    mons: Vec<(i64, i64, String)>,
}

pub fn build_encounter_tables(src: &toml::Value) -> HashMap<String, EncounterTable> {
    let mut encounters = HashMap::new();

    src.as_array().unwrap().iter().for_each(|encounter| {
        let encounter = encounter.as_table().unwrap();
        let id = encounter["id"].as_str().unwrap();
        let mons = encounter["mons"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| {
                let entry = entry.as_array().unwrap();
                assert!(entry.len() == 3);
                let lvl_lower = entry[0].as_integer().unwrap();
                let lvl_upper = entry[1].as_integer().unwrap();
                let mon_id = entry[2].as_str().unwrap().to_owned();

                (lvl_lower, lvl_upper, mon_id)
            })
            .collect::<Vec<_>>();
        encounters.insert(
            id.to_owned(),
            EncounterTable {
                id: id.to_owned(),
                mons,
            },
        );
    });

    encounters
}
