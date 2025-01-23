pub fn reverse(input: &str) -> String {
    let nums: Vec<char> = input.chars().collect();  
    let reversed: String = nums.clone().into_iter().rev().collect();  
    reversed  
}