use rand::prelude::*;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

const CARDS_PER_SUIT : u32 = 13;
const CARDS_PER_WAR : u32 = 3;
const NUM_SUITS : usize = 4;

fn main() {
    // each person will get the same number of cards
    assert!(CARDS_PER_SUIT*(NUM_SUITS as u32) % 2 ==0);
    println!("Started");
    let mut all_stats = Vec::new();
    let num_games = 10000;
    for _ in 0..num_games{
        let game = Game::default();
        let stats = game.play();
        all_stats.push(stats);
    }
    let mean_rounds :f64= all_stats.iter().map(|s| s.num_rounds as f64).sum::<f64>() / (num_games as f64);
    let variance_rounds :f64= all_stats.iter().map(|s| (s.num_rounds as f64 - mean_rounds).powf(2.)).sum::<f64>() / (num_games as f64);
    let mean_wars :f64= all_stats.iter().map(|s| s.num_wars as f64).sum::<f64>() / (num_games as f64);
    let mean_winner :f64= all_stats.iter().map(|s| s.winner as f64).sum::<f64>() / (num_games as f64);
    let variance_winner :f64= all_stats.iter().map(|s| (s.winner as f64 - mean_winner).powf(2.)).sum::<f64>() / (num_games as f64);
    let mut rounds_file = File::create("rounds_data.txt").unwrap();
    for data in all_stats{
        write!(&mut rounds_file, "{},",data.num_rounds).unwrap()
    }
    dbg!(mean_rounds);
    dbg!(mean_wars);
    dbg!(mean_winner);
    dbg!(variance_rounds.sqrt());
    dbg!(variance_winner.sqrt());
    println!("Finished");
}
#[derive(Debug)]
struct Game{
    p1 : VecDeque<u32>,
    p2 : VecDeque<u32>,
}
#[derive(Debug,Default)]
struct GameStats{
    num_rounds : u32,
    num_wars : u32,
    winner : u32,

}
impl Default for Game{
    fn default() -> Self {
        let one_suit = 2..=(CARDS_PER_SUIT+1);
        let mut all_cards : Vec<u32>= one_suit.flat_map(|x| std::iter::repeat(x).take(NUM_SUITS)).collect();
        all_cards.shuffle(&mut rand::thread_rng());
        let player_two_cards = all_cards.split_off(all_cards.len()/2);
        Game {
            p1: all_cards.into_iter().collect(),
            p2: player_two_cards.into_iter().collect(),
        }

    }
}

impl Game{
    fn play(mut self)->GameStats{
        let mut stats = GameStats::default();
        let mut winner: Option<u32> = None;
        while winner == None {
            winner = self.play_round(&mut stats);
        }
        stats.winner = winner.unwrap();
        stats
    }
    // return winner
    fn play_round(&mut self, stats : &mut GameStats)->Option<u32>{
        stats.num_rounds+=1;
        let mut p1_stack = VecDeque::new();
        let mut p2_stack = VecDeque::new();
        match self.p1.pop_front(){
            Some(x)=> p1_stack.push_back(x),
            None=> return Some(2),
        }
        match self.p2.pop_front(){
            Some(x)=> p2_stack.push_back(x),
            None=> return Some(1),
        }
        while p1_stack.back().unwrap() == p2_stack.back().unwrap() {
            stats.num_wars+=1;
            for _ in 0..=CARDS_PER_WAR{
                match self.p1.pop_front(){
                    Some(x)=> p1_stack.push_back(x),
                    None=> return Some(2),
                }
                match self.p2.pop_front(){
                    Some(x)=> p2_stack.push_back(x),
                    None=> return Some(1),
                }
            }
        }
        if p1_stack.back().unwrap() > p2_stack.back().unwrap(){
            self.p1.append(&mut p1_stack);    
            self.p1.append(&mut p2_stack);    
        }else{
            self.p2.append(&mut p2_stack);    
            self.p2.append(&mut p1_stack);    
        }
        None

    }
}




