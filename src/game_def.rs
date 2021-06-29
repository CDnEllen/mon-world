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
