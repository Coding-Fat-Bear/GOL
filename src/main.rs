use std::thread;
use std::time::Duration;

pub const  SIZE:usize = 23;
fn main() {
   let  mut u_matrix = matrix_creation();
   let  mut r_matrix = u_matrix;
   print!("\x1B[2J\x1B[1;1H");//clearing the screen
   for row in &u_matrix {
      println!("{:?}", row);
      }
   loop{
      set_result(&u_matrix,&mut r_matrix,SIZE);
      println!("");

      thread::sleep(Duration::from_millis(200));//waiting for 1 second 
      print!("\x1B[2J\x1B[1;1H");//clearing the screen


      for row in &r_matrix {
         for &cell in row {
             let display_char = match cell {
                 0 => '.',  
                 1 => '#',  
                 _ => '*',  
             };
             print!("{} ", display_char);
         }
         println!(); // Newline at the end of each row
     }
      if u_matrix == r_matrix{
         break;
      }
      u_matrix = r_matrix;
   }

}
fn set_result(int_mat:&[[i8;SIZE];SIZE],r_matrix:&mut [[i8;SIZE];SIZE],size:usize){
   for pos_x in 0..size-2{
      for pos_y in 0..size-2{
         // let result =if get_3x3(int_mat,pos_x,pos_y) {1} else {0};
         // println!("{}",result);
         // r_matrix[pos_x+1][pos_y+1] = result;
         r_matrix[pos_x+1][pos_y+1] = if get_3x3(int_mat,pos_x,pos_y) {1} else {0};
         // println!("to position {},{}",pos_x+1,pos_y+1);
      }
   }
}

fn get_3x3(mat:&[[i8;SIZE];SIZE],row:usize,col:usize)-> bool{
   let mut  count:i8 =0;
   let mut alive=true;
   for i in 0..3{
      for j in 0..3{
         if i == 1 && j== 1 && mat[i+row][j+col] == 1{
            alive = true;//true if the state is alive
            // println!("the cell was alive");
         }else if  i == 1 && j== 1 && mat[i+row][j+col] == 0{
            alive = false;//false if the state is dead
            // println!("the cell was dead");
         }
         else{
            count = count + mat[i+row][j+col];
         }
      }
   }
   // println!("the cell has {count} neighbour");
   if !alive && count>2{
      alive =true//birth
   }
   if alive && (count<2 || count>3) {
      alive = false; //overpopulation&isolated
   }
   alive
}

fn matrix_creation()->[[i8;SIZE];SIZE]{
   let matrix:[[i8;SIZE];SIZE]=[ [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,1,1,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,1,0,1,1,1,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
                                 [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]];
     matrix
}
// fn matrix_creation()->[[i8;SIZE];SIZE]{
//    const RANGE:i32 = SIZE as i32;
//    let arr1:[i8;SIZE]=[0,1,0,0,0,0];
//    let  mut matrix:[[i8;SIZE];SIZE]=[[0; SIZE]; SIZE];
//    let mut i:i32 = 0;
//    loop {
//       matrix[i as usize]=arr1;
//       if i >=RANGE-1{
//          break;
//       }
//       i = i+1;
//    }
//   matrix
// }
