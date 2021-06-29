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

    let mon_defs = game_def::build_mon_defs(root.get("mon_defs").unwrap());
    let mon_instances = game_def::build_mon_instances(root.get("mon_instances").unwrap());

    dbg!(mon_defs);
    dbg!(mon_instances);
}
