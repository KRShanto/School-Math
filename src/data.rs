use crate::find::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Data {
    pub only_numbers: Option<Vec<i32>>,
    pub frequency: Option<Vec<i32>>,
    pub class_interval: Option<Vec<(i32, i32)>>,
    pub class_interval_length: Option<i32>, // how many lines in the table
    pub class_interval_diff_length: Option<i32>,
    pub discrete_class_interval: Option<Vec<(f32, f32)>>,
    pub range: Option<f32>,
    pub cumulative_frequency: Option<Vec<i32>>,
    pub mid_point: Option<Vec<f32>>,
    pub smallest: Option<i32>,
    pub largest: Option<i32>,
    pub fixi: Option<Vec<f32>>,
    pub frequency_sum: Option<i32>,
    pub fixi_sum: Option<f32>,
    pub arithmetic_mean: Option<f32>,
    pub step_deviations: Option<Vec<f32>>, // ui
    pub a: Option<f32>,
    pub fiui: Option<Vec<f32>>,
    pub fiui_sum: Option<f32>,
    pub short_cut_mean: Option<f32>,
}

pub fn get_data_using_only_numbers(only_numbers: Vec<i32>) -> Data {
    // find the frequency
    // find the smallest and largest
    // find the range
    // Choose a class interval length of 5
    // find the class interval
    // find the discrete class interval
    // find the cumulative frequency
    // find the mid point for each class interval

    let smallest = only_numbers.clone().iter().min().unwrap().clone();
    let largest = only_numbers.clone().iter().max().unwrap().clone();
    let range = find_range(smallest, largest);
    let class_interval_diff_length = 5;
    let class_interval_length = find_class_interval_length(range, class_interval_diff_length);
    let class_interval = find_class_interval(smallest, largest, class_interval_diff_length);
    let discrete_class_interval = find_discrete_class_interval(class_interval.clone());
    let frequency = find_frequency(only_numbers.clone(), class_interval.clone());
    let cumulative_frequency = find_cumulative_frequency(frequency.clone());
    let mid_points = find_mid_points(class_interval.clone());
    let fixi = find_fixi(frequency.clone(), mid_points.clone());
    let frequency_sum = find_frequency_sum(frequency.clone());
    let fixi_sum = find_fixi_sum(fixi.clone());
    let arithmetic_mean = find_arithmetic_mean(frequency_sum, fixi_sum);
    let a = find_a(frequency.clone(), mid_points.clone());
    let step_deviations = find_step_deviations(a, mid_points.clone(), class_interval_diff_length);
    let fiui = find_fiui(frequency.clone(), step_deviations.clone());
    let fiui_sum = find_fiui_sum(fiui.clone());
    let short_cut_mean =
        find_short_cut_mean(a, fiui_sum, frequency_sum, class_interval_diff_length);

    Data {
        only_numbers: Some(only_numbers),
        frequency: Some(frequency),
        class_interval: Some(class_interval),
        class_interval_length: Some(class_interval_length),
        class_interval_diff_length: Some(class_interval_diff_length),
        discrete_class_interval: Some(discrete_class_interval),
        range: Some(range),
        cumulative_frequency: Some(cumulative_frequency),
        mid_point: Some(mid_points),
        smallest: Some(smallest),
        largest: Some(largest),
        fixi: Some(fixi),
        frequency_sum: Some(frequency_sum),
        fixi_sum: Some(fixi_sum),
        arithmetic_mean: Some(arithmetic_mean),
        a: Some(a),
        step_deviations: Some(step_deviations),
        fiui: Some(fiui),
        fiui_sum: Some(fiui_sum),
        short_cut_mean: Some(short_cut_mean),
        // ..Default::default()
    }
}
