use std::string;


pub const  SIZE:usize = 6;
fn main() {
   let  u_matrix = matrix_creation();
   let  r_matrix = u_matrix;
   let mut state :bool = get_3x3(&u_matrix,0,2);
   //hange this hard coded num to fetch from (+1+1 to -last-last mid point)butloop will be for 0,0 to last-3,last-3
   if state{
      println!("the cell is alive");
   }else{
      println!("the cell is dead");
   }
  
  for row in &r_matrix {
   println!("{:?}", row);
   }
}

fn get_3x3(mat:&[[i8;6];6],row:usize,col:usize)-> bool{
   let mut  count:i8 =0;
   let mut alive=true;
   for i in 0..3{
      for j in 0..3{
         if i == 1 && j== 1 && mat[i+row][j+col] == 1{
            alive = true;//true if the state is alive
            println!("the cell was alive");
         }else if  i == 1 && j== 1 && mat[i+row][j+col] == 0{
            alive = false;//false if the state is dead
            println!("the cell was dead");
         }
         else{
            count = count + mat[i+row][j+col];
         }
      }
   }
   println!("the cell is has {count} neighbour");
   if !alive && count>2{
      alive =true//birth
   }
   if alive && (count==0 || count>3) {
      alive = false; //overpopulation&isolated
   }
   alive
}

fn matrix_creation()->[[i8;SIZE];SIZE]{
   const RANGE:i32 = SIZE as i32;
   let arr1:[i8;SIZE]=[0,1,0,0,0,0];
   let  mut matrix:[[i8;SIZE];SIZE]=[[0; SIZE]; SIZE];
   let mut i:i32 = 0;
   loop {
      matrix[i as usize]=arr1;
      if i >=RANGE-1{
         break;
      }
      i = i+1;
   }
  matrix
}
