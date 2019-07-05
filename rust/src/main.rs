#![feature(vec_remove_item)]

use rand::prelude::*;
use rand::distributions::{Uniform};

use std::vec::Vec;
use std::iter::FromIterator;

const TRIALS: usize = 10_000_000;
const SWITCH: Strategy = Strategy::Switch;
const STAY: Strategy = Strategy::Stay;

fn switch(d: Door, ds: Doors) -> Door { ds.without(d).index(0) }
fn stay(d: Door, _ds: Doors) -> Door { d }

fn all_doors() -> Doors {
    Doors::from_slice(&[Door::DoorOne, Door::DoorTwo, Door::DoorThree])
}

fn run_game(
    strategy: &Strategy,
    mut rng: &mut ThreadRng
) -> Result {
    let door_with_car = all_doors().random(&mut rng);
    let player_guess = all_doors().random(&mut rng);
    let opened_door = all_doors().without(door_with_car).random(&mut rng);    let player_final_guess = match &strategy {
        &Strategy::Switch => {
            switch(player_guess, all_doors().without(opened_door))
        },
        &Strategy::Stay => {
            stay(player_guess, all_doors().without(opened_door))
        },
    };
    if player_final_guess == door_with_car {
        Result::Win
    } else {
        Result::Lose
    }
}

struct Results {
    won: u64,
    lost: u64,
}

impl Results {
    fn new() -> Results {
        Results { won: 0, lost: 0 }
    }

    fn score(&mut self, res: Result) {
        match res {
            Result::Lose => self.lost += 1,
            Result::Win => self.won += 1,
        }
    }

    fn percentage(&self) -> f64 {
        100.0 * (self.won as f64) / ((self.won + self.lost) as f64)
    }
}

fn run_games(
    strategy: &Strategy,
    mut rng: &mut ThreadRng,
    num_games: usize,
) -> Results {
    let mut res = Results::new();

    for _ in 0..num_games {
      res.score(run_game(&strategy, &mut rng))
    }

    res
}

enum Result {
    Win,
    Lose,
}

enum Strategy {
    Switch,
    Stay,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
enum Door {
    DoorOne,
    DoorTwo,
    DoorThree,
}

struct Doors(Vec<Door>);

impl Doors {
    fn index(&self, index: usize) -> Door {
        self.0[index]
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn from_slice(data: &[Door]) -> Doors {
        Doors (Vec::from_iter(data.iter().cloned()))
    }

    fn without(&self, door: Door) -> Doors {
        let mut new_doors = self.0.to_owned();
        new_doors.remove_item(&door);
        Doors(new_doors)
    }

    fn random(&self, mut rng: &mut ThreadRng) -> Door {
        let ix = Uniform::from(1..10_000).sample(&mut rng) % self.len();
        self.index(ix)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let sw = run_games(&SWITCH, &mut rng, TRIALS).percentage();
    let st = run_games(&STAY, &mut rng, TRIALS).percentage();
    println!("Switch wins {}% of the time.", sw);
    println!("Stay wins {}% of the time.", st);
}
