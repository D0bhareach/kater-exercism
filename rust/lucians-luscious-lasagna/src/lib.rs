// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
const MINUTES_PER_LAYER: i32  = 2;
const EXPECTED_MINUTES_IN: i32 = 40;

pub fn expected_minutes_in_oven() -> i32 {
    EXPECTED_MINUTES_IN
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    
        expected_minutes_in_oven() - actual_minutes_in_oven
    
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    
        number_of_layers * MINUTES_PER_LAYER
    
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
   
        preparation_time_in_minutes(number_of_layers) +
        actual_minutes_in_oven
    }
