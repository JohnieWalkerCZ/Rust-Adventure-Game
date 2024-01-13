use std::{char, collections::HashMap};

use crate::libs::consts::room_consts;
use rand::Rng;
use rand::seq::SliceRandom;

use super::consts::room_consts::{TOP, BOTTOM, LEFT, RIGHT};

pub struct Room {
   pub grid_position: (i8, i8),
   pub doors: Vec<char>,
}

impl Room {
   pub fn new(grid_position: (i8, i8), doors: Vec<char>) -> Room {
      Room {
         grid_position,
         doors,
      }
   }
   

   pub fn render_room(&self, position: (u8, u8)) {

      for r in 0..7 {
         for c in 0..12 {
            if r == 0 || r == 6 || c == 0 || c == 11 {
               let t_b_range = 3..9;
               let r_l_range = 2..5; 
               if (self.doors.contains(&room_consts::TOP) && t_b_range.contains(&c) && r == 0) || 
                  (self.doors.contains(&room_consts::BOTTOM) && t_b_range.contains(&c) && r == 6) ||
                  (self.doors.contains(&room_consts::LEFT) && r_l_range.contains(&r) && c == 0) ||
                  (self.doors.contains(&room_consts::RIGHT) && r_l_range.contains(&r) && c == 11) {
                  if r == position.1 && c == position.0 {
                     print!("&");
                     continue;
                  }
                  print!(":");
                  continue;
               }
               print!("@");
               continue;
            }
            if r == position.1 && c == position.0 {
              print!("&");
               continue;
            }
            print!(".");
         }
         print!("\n\r");
      }
   }

   pub fn clone(&self) -> Room {
      return Room {
         grid_position: self.grid_position,
         doors: self.doors.clone(),
      };
   }
}

pub fn create_next_room (grid_position: (i8, i8), direction: char, rooms: HashMap<(i8, i8), Room>) -> Room {
   // Doors - 1 from the coming direction, next random
   let mut rng = rand::thread_rng();
   let mut all_directions = vec![TOP, BOTTOM, LEFT, RIGHT];
   let index = all_directions.iter().position(|&x| x == direction).unwrap();
   all_directions.remove(index);
   let num_doors = rng.gen_range(0..all_directions.len());
   let mut new_doors: Vec<_> = all_directions.choose_multiple(&mut rng, num_doors).cloned().collect();
   new_doors.push(direction);
   let new_room = Room::new(grid_position, new_doors);
   return new_room;

}
