// 4. Understanding Ownership
fn greet_with_job(name: &String, job: &String){
    let article = if job[..1].to_lowercase() == "a" {
        String::from("an")
    } else {
        String::from("a")
    };

    println!("My name is {}. I am {} {}.", name, article, job);
}

fn greet_main(){
    let name = String::from("David");
    let job = String::from("Accountant");

    greet_with_job(&name, &job);
}

// 5. Using Structs to Structure Related Data

#[derive(Debug)]
struct Pokemon {
    name: String,
    pokemon_name: String,
    health_points: u32,
    experience: u32,
    attack_points: u32,
}

impl Pokemon {
    fn attack(&mut self, pokemon: &mut Pokemon) {
        pokemon.health_points -= self.attack_points;
        self.health_points -= (pokemon.attack_points as f32 * 0.5) as u32;

        self.experience += 10;
    }
}

fn pokemon_main(){
    let mut p1 = Pokemon {
        name: String::from("pika"),
        pokemon_name: String::from("Pikachu"),
        health_points: 20,
        experience: 0,
        attack_points: 12,
    };

    let mut p2 = Pokemon {
        name: String::from("pika2"),
        pokemon_name: String::from("pikacho de fuego"),
        health_points: 35,
        experience: 0,
        attack_points: 7,
    };

    println!("Pokemon 1 before: {:?}", p1);
    println!("Pokemon 2 before: {:?}", p2);

    p1.attack(&mut p2);

    println!("Pokemon 1 after: {:?}", p1);
    println!("Pokemon 2 after: {:?}", p2);
}


// 6. Enums

#[derive(Debug)]
struct Song {
    id: u32,
    title: String,
    artist: String,
}

#[derive(Debug)]
enum Message {
    Play(Song),
    Stop,
    SetVolume(f32),
}

impl Message {
    fn send(&self){
        println!("{:?}", self);
    }
}

fn send(m: Message){
    match m {
        Message::Play(song) => println!("Now playing: {} by {}", song.title, song.artist),
        Message::Stop => println!("Stopping..."),
        Message::SetVolume(level) => println!("Setting volume to: {}", level),
    }
}

fn messaging_main(){
    let s = Song {id: 1,
        title: String::from("Nada Personal"),
        artist: String::from("Soda Stereo"),
    };

    let m = Message::Play(s);

    send(m);

    let m = Message::SetVolume(0.5);

    send(m);

    let m = Message::Stop;

    send(m);
}

fn if_let_test(){
    let config_max = Some(100u8);

    match config_max {
        Some(max) => println!("The maximum set is {}", max),
        _ => (),
    }

    let config_max: Option<u8> = None;

    match config_max {
        Some(max) => println!("The maximum set is {}", max),
        _ => (),
    }

    let config_max = Some(100u8);

    if let Some(max) = config_max {
        println!("The maximum set is {}", max)
    }
}
// 7 Managing growing projects (incomplete)

use crate::my_module::my_submodule::print_poop;
pub mod my_module;

fn main() {
    print_poop();
}