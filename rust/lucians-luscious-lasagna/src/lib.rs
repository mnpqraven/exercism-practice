// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// defines how long the food should be in the oven
// @returns i32 duration
pub fn expected_minutes_in_oven() -> i32 {
  let dur= 40;
   return dur;
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
  let remain_dur = expected_minutes_in_oven() - actual_minutes_in_oven;
  return remain_dur;
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
  return number_of_layers * 2; 
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
  return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;
}
