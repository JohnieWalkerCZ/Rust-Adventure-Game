use std::{char, collections::HashMap};

use crate::libs::consts::room_consts;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

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

pub fn create_next_room (grid_position: (i8, i8), direction: char, rooms: &mut HashMap<(i8, i8), Room>) -> Room {
   // Doors - 1 from the coming direction, next random
   let mut rng = rand::thread_rng();
   let all_directions = vec![TOP, BOTTOM, LEFT, RIGHT];
   
   let mut neighbour_rooms = vec![];
   let mut banned_directions = vec![];
   // Checking room on the left
   if let Some(room) = rooms.get(&(grid_position.0 + 1, grid_position.1)) {
      if room.doors.contains(&LEFT) {
         neighbour_rooms.push(RIGHT);
      } else {
         banned_directions.push(RIGHT);
      }
   }
   // Checking room on the right
   if let Some(room) = rooms.get(&(grid_position.0 - 1, grid_position.1)) {
      if room.doors.contains(&RIGHT) {
         neighbour_rooms.push(LEFT);
      } else {
          banned_directions.push(LEFT);
      }
   }
   // Checking room on the top
   if let Some(room) = rooms.get(&(grid_position.0, grid_position.1 + 1)) {
      if room.doors.contains(&BOTTOM) {
         neighbour_rooms.push(TOP);
      } else {
          banned_directions.push(TOP);
      }
   }
   // Checking room on the bottom
   if let Some(room) = rooms.get(&(grid_position.0, grid_position.1 - 1)) {
      if room.doors.contains(&TOP) {
         neighbour_rooms.push(BOTTOM);
      } else {
          banned_directions.push(BOTTOM);
      }
   }
   
   neighbour_rooms.push(direction);
   let neighbour_rooms_set: HashSet<_> = neighbour_rooms.clone().into_iter().collect();

   let all_directions_set: HashSet<_> = all_directions.into_iter().collect();

   let diff: Vec<_> = all_directions_set.difference(&neighbour_rooms_set).cloned().collect();

   let banned_set: HashSet<_> = banned_directions.into_iter().collect();

   let diff_set: HashSet<_> = diff.into_iter().collect();
   let result: Vec<_> = diff_set.difference(&banned_set).cloned().collect();

   // let index = all_directions.iter().position(|&x| x == direction).unwrap();
   // all_directions.remove(index);

   let num_doors = rng.gen_range(0..result.len());
   
   let mut new_doors: Vec<_> = result.choose_multiple(&mut rng, num_doors).cloned().collect();
   new_doors.append(&mut neighbour_rooms);
   
   let new_room = Room::new(grid_position, new_doors);
   return new_room;

}
