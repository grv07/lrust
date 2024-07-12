// Referances
// https://www.youtube.com/watch?v=Vw8BFScm0K0w

// compute at compiletime but not everything can be added there

const GAME_NUMBER: i32 = 39;
const PLAYERS_COUNT: usize = 3;
// Use const in const

// Can't call on const fun in const
// const NAME: String = String::from("Gaurav");

const GAME_PLAYERS: [i32; PLAYERS_COUNT] = [0; PLAYERS_COUNT];

const GAME_SCORE: [(i32, usize); 3] = [(0, 0); 3];

//TODO(next video): usize vs i32/i64

fn get_active_player() -> i32 {
    // scoped const
    const ACTIVE_PLAYER: i32 = 69;
    ACTIVE_PLAYER
}

// ----------- const on custom types

const AGE: i32 = 69;

#[derive(Debug)]
struct Student<'a> {
    age: i32,
    name: &'a str,
}

const GAMER: Student = Student {
    age: AGE,
    name: "Joy",
};

// ------- const in traits
trait State {
    const PLAYERS: [i32; 3] = [0; 3];

    const LOW_AGE_LIMIT: i32 = 12;
}

// ------- override default in struct
#[derive(Debug)]
struct GameState {
    time: i32,
}

impl State for GameState {
    const LOW_AGE_LIMIT: i32 = 11;
}

// ------------- Unnamed const

// annonimus const
const _: () = { () };
//             ----> this code stil runs at compile time;

// example use: lets check if GameState impl State at compile time
const _: () = {
    struct ImplState<T: State>(T);

    // pass case
    let _ = ImplState(GameState {});

    // fail case
    // struct GameStateNew {}
    // let _ = ImplState(GameStateNew {});
};

fn test<'a>() -> &'a GameState {
    //&'a Vec<i32> {
    // return &vec![32];

    /* Returning structs from functions moves them, and moves may change their address,
      which invalidates all references to that struct.
      Rust chose to promise moves are simple memcpy without any extra code running
      https://users.rust-lang.org/t/is-it-possible-to-return-a-struct-containing-reference-to-local-variable-and-the-variable-it-self-by-moving/107566
    */
    return &GameState { time: 23 };
}

fn main() {
    println!("{}\n{:?}\n{:?}", GAME_NUMBER, GAME_PLAYERS, GAME_SCORE);
    println!("Active Playe: {}", get_active_player());

    println!("");

    println!("Gamer student age {:?}", GAMER.age);
    println!("Gamer student nam {:?}", GAMER.name);

    println!("");

    // const copy when assign
    let mut stu: Student = GAMER;

    stu.age = 16;
    println!("Gamer student age {:?}", stu.age);
    println!("Gamer student nam {:?}", stu.name);

    println!("");

    // real const un effected
    println!("Gamer student age {:?}", GAMER.age);
    println!("Gamer student nam {:?}", GAMER.name);

    println!("");

    // override default trait const
    let gs = GameState {};

    println!("{:?}", gs);
    println!("low age limit: {:?}", GameState::LOW_AGE_LIMIT);
}
