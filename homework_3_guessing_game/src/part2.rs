use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        
        let mut current_max = max ;
        let mut current_min = min ;

        loop {

            let midpoint = current_min + (current_max - current_min) / 2;

            match player.ask_to_compare(midpoint) {
                0 => return midpoint ,
                -1 => {
                    current_max = midpoint - 1 ;
                }
                1 => {
                    current_min = midpoint + 1 ;
                }
                _ => unreachable!()
            }    
        }
    }
}
