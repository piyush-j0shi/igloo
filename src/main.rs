use std::collections::HashMap;
use std::hash::Hash;
use std::io;

type RoomId = String;
type ItemId = String;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

#[derive(Debug, Clone)]
struct Room {
    id: RoomId,
    name: String,
    description: String,
    exits: HashMap<Direction, RoomId>,
    items: Vec<ItemId>,
}

#[derive(Debug)]
struct Item {
    id: ItemId,
    name: String,
    description: String,
}

#[derive(Debug)]
struct Player {
    currentroom: RoomId,
    inventory: Vec<ItemId>,
}

struct GameState {
    rooms: HashMap<RoomId, Room>,
    items: HashMap<ItemId, Item>,
    player: Player,
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read lines");
    input.trim().to_string()
}

fn create_world() -> GameState {
    let item1 = Item {
        id: "item1".to_string(),
        name: "someitem1".to_string(),
        description: "this is room no 1".to_string(),
    };

    let item2 = Item {
        id: "item2".to_string(),
        name: "someitem2".to_string(),
        description: "this is room no 2".to_string(),
    };

    let item3 = Item {
        id: "item3".to_string(),
        name: "someitem3".to_string(),
        description: "this is room no 3".to_string(),
    };

    let mut exit1 = HashMap::new();
    exit1.insert(Direction::East, "room2".to_string());

    let mut exit2 = HashMap::new();
    exit2.insert(Direction::South, "room3".to_string());
    exit2.insert(Direction::West, "room1".to_string());

    let mut exit3 = HashMap::new();
    exit3.insert(Direction::North, "room2".to_string());

    let mut items_list: Vec<String> = Vec::new();
    items_list.push("item1".to_string());
    items_list.push("item2".to_string());
    items_list.push("item3".to_string());

    let room1 = Room {
        id: "room1".to_string(),
        name: "someroom1".to_string(),
        description: "this is some room1".to_string(),
        exits: exit1,
        items: vec![items_list[0].clone()],
    };

    let room2 = Room {
        id: "room2".to_string(),
        name: "someroom2".to_string(),
        description: "this is some room2".to_string(),
        exits: exit2,
        items: vec![items_list[1].clone()],
    };

    let room3 = Room {
        id: "room3".to_string(),
        name: "someroom3".to_string(),
        description: "this is some room3".to_string(),
        exits: exit3,
        items: vec![items_list[2].clone()],
    };

    let player = Player {
        currentroom: "room1".to_string(),
        inventory: vec![],
    };

    let mut rooms = HashMap::new();
    rooms.insert("room1".to_string(), room1);
    rooms.insert("room2".to_string(), room2);
    rooms.insert("room3".to_string(), room3);

    let mut items = HashMap::new();
    items.insert("item1".to_string(), item1);
    items.insert("item2".to_string(), item2);
    items.insert("item3".to_string(), item3);

    GameState {
        rooms: rooms,
        items: items,
        player: player,
    }
}

fn main() {
    println!("type something");
    let user_input = read_input();
    println!("typed : {}", user_input);

    let gamestate = create_world();

    println!(
        "current room of player : {:?}",
        gamestate.player.currentroom
    );

    if let Some(room) = gamestate.rooms.get(&gamestate.player.currentroom) {
        println!("current room info : {:?}", room);
    } else {
        println!("nothing here");
    }

    if let Some(item) = gamestate.items.get(&gamestate.player.inventory); 

    // println!(
    //     "all rooms : {:?}",
    //     gamestate.rooms.get(&gamestate.player.currentroom)
    // );
}
