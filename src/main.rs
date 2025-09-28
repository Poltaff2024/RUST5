fn main() {
   const USER_MAX_SCORE: u32 = 1_000_000;
   println!("Info: {}", USER_MAX_SCORE);

   let mut user_alex = (42, true, 1.86, 'R');
   user_alex.2 = 46.0;
    println!("Info: {}", user_alex.2);

    let mut nums: [i8; 6] = [4, 3, 7, 9, 1, 8];
    nums[0] = 10; 
    println!("Info: {}", nums[0]);
}
