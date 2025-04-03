
pub const  SIZE:usize = 6;
fn main() {
   let  u_matrix = matrix_creation();
   let  r_matrix = u_matrix;
   
   get_3x3(&u_matrix,1,1);
  println!("result");
  for row in &r_matrix {
   println!("{:?}", row);
   }
}
fn get_3x3(mat:&[[i8;6];6],row:usize,col:usize){
   let mut sub_mat=[[0;3];3];
   for i in 0..3{
      for j in 0..3{
         sub_mat[i][j] = mat[i+row][j+col];
      }
   }
   for row in &sub_mat {
      println!("{:?}", row);
      }
}

fn matrix_creation()->[[i8;SIZE];SIZE]{
   const RANGE:i32 = SIZE as i32;
   let arr1:[i8;SIZE]=[0,0,0,0,0,0];
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
