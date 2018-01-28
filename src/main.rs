extern crate modules;


pub mod a {
  pub mod series {
    pub mod of {
      pub fn nested_modules() {}
    }
  }
}

enum TrafficLights {
  Green,
  Yellow,
  Red
}

enum VideoGames {
  Fortnite,
  LoL,
  MonsterHunterWorld,

}

// examples of bringing into scope
use a::series::of;
// can bring enums into scope as well
// can bring multiple items from one scope with this syntax
use TrafficLights::{Yellow, Red};
// can bring all items into scope with the glob operator
use VideoGames::*;

fn main () {
  modules::client::connect();
  // instead of calling
  // a::series::of:nested_modules();
  // 'use' lets us bring nested modules
  // into scope
  of::nested_modules();

  let red = Red;
  let yellow = Yellow;
  let green = TrafficLights::Green;
}