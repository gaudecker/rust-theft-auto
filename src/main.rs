use map::{Map};

mod map;

fn main() {
    let map = match Map::from_file("data/nyc.cmp") {
        Err(why) => fail!("could not load map: {}", why.desc),
        Ok(map) => map
    };

    for location in map.get_locations(&map::location::Hospital).iter() {
        println!("{}", location);
    }
}
