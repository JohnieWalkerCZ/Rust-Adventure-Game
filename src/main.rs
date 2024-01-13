use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod libs;
use libs::room::{Room, create_next_room};
use libs::consts::room_consts::TOP;
use libs::consts::room_consts::BOTTOM;
use libs::consts::room_consts::LEFT;
use libs::consts::room_consts::RIGHT;

fn main() {
   std::process::Command::new("clear").status().unwrap();
   let stdin = stdin();
   let mut stdout = stdout().into_raw_mode().unwrap();
   
   let mut x = 1;
   let mut y = 1;
   
   let mut rooms: HashMap<(i8, i8), Room> = HashMap::new();
   
   let mut current_room = Room::new((0, 0), vec![TOP, BOTTOM, RIGHT, LEFT]);
   
   let new_room = current_room.clone();
   rooms.insert((0, 0), new_room);
   current_room.render_room((1, 1));

   for c in stdin.keys() {
      match c.unwrap() {
         Key::Left => {
            let l_r_range = 2..5;
            if current_room.doors.contains(&LEFT) && l_r_range.contains(&y) && x == 1 {
               x = 11;

               let new_position = (current_room.grid_position.0 - 1, current_room.grid_position.1);
               if let Some(room) = rooms.get(&new_position) {
                  current_room = room.clone();
               } else {
                  let new_room = create_next_room(new_position, RIGHT);
                  current_room = new_room.clone();
                  rooms.insert(new_position, new_room);
               }
            }
            if x > 1 {
               x = x - 1;
            }
         },
         Key::Right => {
            let l_r_range = 2..5;
            if current_room.doors.contains(&RIGHT) && l_r_range.contains(&y) && x == 10 {
               x = 0;
               
               let new_position = (current_room.grid_position.0 + 1, current_room.grid_position.1);
               if let Some(room) = rooms.get(&new_position) {
                  current_room = room.clone();
               } else {
                  let new_room = create_next_room(new_position, LEFT);
                  current_room = new_room.clone();
                  rooms.insert(new_position, new_room);
               }
           }

            if x < 10 {
               x = x + 1;
            }
         },
         Key::Up => {
            let t_b_range = 3..9;
            if current_room.doors.contains(&TOP) && t_b_range.contains(&x) && y == 1 {
               y = 6;
               
               let new_position = (current_room.grid_position.0, current_room.grid_position.1 + 1);
               if let Some(room) = rooms.get(&new_position) {
                  current_room = room.clone();
               } else {
                  let new_room = create_next_room(new_position, BOTTOM);
                  current_room = new_room.clone();
                  rooms.insert(new_position, new_room);
               }
            }

            if y > 1 {
               y = y - 1;
            }
         }
         Key::Down => {
            let t_b_range = 3..9;
            if current_room.doors.contains(&BOTTOM) && t_b_range.contains(&x) && y == 5 {
               y = 0;
               
               let new_position = (current_room.grid_position.0, current_room.grid_position.1 - 1);
               if let Some(room) = rooms.get(&new_position) {
                  current_room = room.clone();
               } else {
                  let new_room = create_next_room(new_position, TOP);
                  current_room = new_room.clone();
                  rooms.insert(new_position, new_room);
               }
            }

            if y < 5 {
               y = y + 1;
            }
         }
         Key::Esc => break,
         _ => (),
      }
      stdout.flush().unwrap();
      std::process::Command::new("clear").status().unwrap();
      current_room.render_room((x, y));
   }
}
