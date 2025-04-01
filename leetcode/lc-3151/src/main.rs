pub fn is_array_special(nums: Vec<i32>) -> bool {
    let mut ok: bool = true;
    if nums.len() == 1 {
        return ok;
    }
    
    for i in 0..nums.len()-1 {
        if nums[i+1] % 2 == nums[i] % 2 {
            ok = false;
            break;
        }
    }

    ok
}

fn main() {
    let nums: Vec<i32> = vec![4, 3, 1, 6];
    println!("{}", is_array_special(nums));
}
