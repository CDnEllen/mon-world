use rand::Rng;

macro_rules! unwrap_matches {
    ($e:expr, $p:pat) => {
        match $e {
            $p => (),
            _ => panic!(""),
        }
    };
    ($e:expr, $p:pat => $body:expr) => {
        match $e {
            $p => $body,
            _ => panic!(""),
        }
    };
}

mod game_def;

fn main() {
    let input = std::fs::read_to_string("./data.toml").unwrap();
    let root = input.parse::<toml::Value>().unwrap();
    let game_def = game_def::GameDef::from_toml(&root);

    for encounter_id in game_def.encounter_tables.keys() {
        for _ in 0..10 {
            dbg!(generate_rng_encounter(
                &game_def,
                vec![],
                encounter_id.clone()
            ));
        }
    }
}

#[derive(Debug)]
pub struct Mon {
    def: String,
    moves: Vec<String>,
    lvl: i64,
    exp: i64,
    cur_hp: i64,
}

#[derive(Debug)]
pub struct Battle {
    player_team: Vec<Mon>,
    active_player_mons: Box<[Option<usize>]>,
    enemy_team: Vec<Mon>,
    active_enemy_mons: Box<[Option<usize>]>,
}

pub fn generate_rng_encounter(
    game_def: &game_def::GameDef,
    player_team: Vec<Mon>,
    encounter_id: String,
) -> Battle {
    let encounter_table = &game_def.encounter_tables[&encounter_id];
    let rng_mon = || {
        let mon_idx = rand::thread_rng().gen_range(0..encounter_table.mons.len());
        let &(min_lvl, max_lvl, ref mon_def_id) = &encounter_table.mons[mon_idx];
        let lvl = rand::thread_rng().gen_range(min_lvl..=max_lvl);
        let mon_def = &game_def.mon_defs[mon_def_id];
        let moves = mon_def.moves_at_level(lvl);

        Mon {
            def: mon_def_id.clone(),
            moves,
            lvl,
            exp: 0,
            cur_hp: mon_def.hp,
        }
    };
    let enemy_team = vec![rng_mon(), rng_mon()];

    let active_player_mons = player_team
        .iter()
        .enumerate()
        .map(|(n, _)| Some(n))
        .take(2)
        .collect::<Box<[_]>>();

    Battle {
        player_team,
        active_player_mons,
        enemy_team,
        active_enemy_mons: Box::new([Some(0), Some(1)]),
    }
}
