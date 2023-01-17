#![allow(unused)]

use wasm_bindgen::JsCast;
use weblog::console_log;
use webru::document;
use yew::prelude::*;
/*
English to Bangla

Frequency -> গনসংখ্যা
Class interval -> শ্রেণী ব্যপ্তি / শ্রেণী ব্যবধান
Discrete class interval -> অবিচ্ছিন্ন শ্রেণীসীমা
Tally -> ট্যালি
Range -> পরিসর
Cumulative frequency -> ক্রমযোজিত গনসংখ্যা
Mid Point/Value -> মধ্যবিন্দু / মধ্যমান


*/

#[derive(Clone, Debug, PartialEq, Default)]
struct Data {
    only_numbers: Option<Vec<i32>>,
    frequency: Option<Vec<i32>>,
    class_interval: Option<Vec<(i32, i32)>>,
    class_interval_length: Option<i32>,
    class_interval_diff_length: Option<i32>,
    discrete_class_interval: Option<Vec<(f32, f32)>>,
    range: Option<f32>,
    cumulative_frequency: Option<Vec<i32>>,
    mid_point: Option<Vec<f32>>,
    smallest: Option<i32>,
    largest: Option<i32>,
    fixi: Option<Vec<f32>>,
    frequency_sum: Option<i32>,
    fixi_sum: Option<f32>,
    arithmetic_mean: Option<f32>,
    step_deviations: Option<Vec<f32>>, // ui
    a: Option<f32>,
    fiui: Option<Vec<f32>>,
    fiui_sum: Option<f32>,
    short_cut_mean: Option<f32>,
}

fn main() {
    yew::Renderer::<App>::new().render();
    // let num: i32 = 55 / 2;
    // println!("{}", num);
}

#[function_component]
fn App() -> Html {
    let only_numbers = use_state(|| vec![]);
    let frequency_class_interval = use_state(|| None);
    let data = use_state(|| Data::default());
    let show_output = use_state(|| false);

    {
        let only_numbers = only_numbers.clone();
        let data = data.clone();
        use_effect_with_deps(
            move |only_numbers| {
                if !only_numbers.is_empty() {
                    let data_using_numbers = get_data_using_only_numbers((**only_numbers).clone());
                    data.set(data_using_numbers);
                }

                || ()
            },
            only_numbers,
        );
    }
    html! {
        <div>
            <ContextProvider<UseStateHandle<Vec<i32>>> context={only_numbers.clone()}>
            <ContextProvider<UseStateHandle<Option<(Vec<i32>, Vec<(i32, i32)>)>>> context={frequency_class_interval.clone()}>
            <ContextProvider<UseStateHandle<Data>> context={data.clone()}>
            <ContextProvider<UseStateHandle<bool>> context={show_output.clone()}>
                <>
                    <TakeInput  />
                    if *show_output {
                        <ShowOutput  />
                    }
                </>
            </ContextProvider<UseStateHandle<bool>>>
            </ContextProvider<UseStateHandle<Data>>>
            </ContextProvider<UseStateHandle<Option<(Vec<i32>, Vec<(i32, i32)>)>>>>
            </ContextProvider<UseStateHandle<Vec<i32>>>>
        </div>
    }
}

#[function_component]
fn TakeInput() -> Html {
    let only_numbers = use_context::<UseStateHandle<Vec<i32>>>().unwrap();
    let show_output = use_context::<UseStateHandle<bool>>().unwrap();

    html! {
        <div class="take-input">
            <div class="only-numbers">
                <label for="only-numbers">{"শুধু সংখ্যা লিখুন"}</label>
                <input type="text" id="only-numbers" name="only-numbers" value="70 40 35 60 55 58 45 60 65 80 70 46 50 60 65 70 58 60 48 70 36 85 60 50 46 65 55 61 72 85 90 68 65 50 40 56 60 65 46 76" />
                <button
                    onclick={
                        let only_numbers = only_numbers.clone();
                        let show_output = show_output.clone();
                        move |_| {
                            let input = document().get_element_by_id("only-numbers").unwrap();
                            let input = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            let input = input.value();
                            let input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                            only_numbers.set(input);
                            show_output.set(true);
                    }}>
                    {"Calculate"}
                </button>
            </div>

            <div class="frequency-class-interval">

            </div>
        </div>
    }
}

#[function_component]
fn ShowOutput() -> Html {
    // let ShowOutputProps { only_numbers } = props;
    // let only_numbers = use_context::<UseStateHandle<Vec<i32>>>().unwrap();
    let data = use_context::<UseStateHandle<Data>>().unwrap();

    html! {
        <div class="show-output">
            <div class="only-numbers">
            if let Some(only_numbers) = data.only_numbers.clone() {
                // Show some details
                <p>{"সংখ্যাগুলো: "}{data.only_numbers.clone().unwrap().iter().map(|x| format!("{}, ", x)).collect::<String>()}</p>
                <p>{"সর্বোচ্চ: "}{data.largest.unwrap()}</p>
                <p>{"সর্বনিম্ন: "}{data.smallest.unwrap()}</p>
                <p>{"পরিসর: "}{data.range.unwrap()}</p>
                <p>{"শ্রেণী ব্যপ্তি: "}{format!("{:?}", data.class_interval.clone().unwrap())}</p>
                <p>{"লাইন নাম্বার: "}{data.class_interval_length.unwrap()}</p>
                <p>{"শ্রেণী ব্যবধান: "}{data.class_interval_diff_length.unwrap()}</p>
                <p>{"গনসংখ্যাগুলার সমষ্টি: "}{data.frequency_sum.unwrap()}</p>
                <p>{"fixi সমষ্টি: "}{data.fixi_sum.unwrap()}</p>
                <p>{"গাণিতিক গড়: "}{data.arithmetic_mean.unwrap()}</p>
                <p>{"সংক্ষিপ্ত গড়: "}{data.short_cut_mean.unwrap()}</p>
                <p>{"a: "}{data.a.unwrap()}</p>
                <p>{"fiui সমষ্টি: "}{data.fiui_sum.unwrap()}</p>
            }
                // Make a table
                <table style="
                    border-collapse: collapse;
                    border: 3px solid yellow;
                ">
                // class_interval
                    // frequency
                    // cumulative_frequency
                    // discrete_class_interval
                    // mid_point
                <tr style="
                border: 3px solid yellow; padding: 5px;
                ">
                    <th style="border: 3px solid yellow; padding: 5px;">{"শ্রেণী ব্যপ্তি"} </th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"গনসংখ্যা (fi)"} </th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"অবিচ্ছিন্ন শ্রেণীসীমা"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"ক্রমযোজিত গনসংখ্যা"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"মধ্যমান (xi)"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"(fixi)"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"(fiui)"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"(ui)"}</th>

                </tr>
                {
                    if let Some(class_interval_length) = data.class_interval_length.clone() {
                        // for i in 0..class_interval_length as usize {
                            console_log!(class_interval_length);
                        (0..class_interval_length as usize).into_iter().map(|i| {
                            html! {
                                <tr>
                                <td style="border: 3px solid yellow; padding: 5px;">{format!("{} - {}", data.class_interval.clone().unwrap()[i].0, data.class_interval.clone().unwrap()[i].1)}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.frequency.clone().unwrap()[i]}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{format!("{} - {}", data.discrete_class_interval.clone().unwrap()[i].0, data.discrete_class_interval.clone().unwrap()[i].1)}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.cumulative_frequency.clone().unwrap()[i]}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.mid_point.clone().unwrap()[i]}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.fixi.clone().unwrap()[i]}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.fiui.clone().unwrap()[i]}</td>
                                <td style="border: 3px solid yellow; padding: 5px;">{data.step_deviations.clone().unwrap()[i]}</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    } else {
                        html! {}
                    }
                }
                </table>
            </div>
        </div>
    }
}

fn get_data_using_only_numbers(only_numbers: Vec<i32>) -> Data {
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

fn find_range(smallest: i32, largest: i32) -> f32 {
    ((largest - smallest) + 1) as f32
}

fn find_class_interval_length(range: f32, class_interval_diff_length: i32) -> i32 {
    // range / class_interval_diff_length

    // if there is a remainder, then add 1 to the result
    let class_interval_length: f32 = range as f32 / class_interval_diff_length as f32;
    let class_interval_length = class_interval_length.ceil() as i32;

    class_interval_length
}

fn find_class_interval(
    smallest: i32,
    largest: i32,
    class_interval_diff_length: i32,
) -> Vec<(i32, i32)> {
    let mut class_interval = vec![];
    let mut start = smallest;
    let mut end = smallest + class_interval_diff_length - 1;
    while end <= (largest + class_interval_diff_length - 1) {
        class_interval.push((start, end));
        start = end + 1;
        end = start + class_interval_diff_length - 1;
        // console_log!(start, " - ", end);
    }
    class_interval
}

fn find_discrete_class_interval(class_interval: Vec<(i32, i32)>) -> Vec<(f32, f32)> {
    // class interval: [(46, 50), (51, 55)]
    // discrete class interval: [(45.5, 50.5), (50.5, 55.5)]
    let mut discrete_class_interval = vec![];

    for (start, end) in class_interval {
        discrete_class_interval.push((start as f32 - 0.5, end as f32 + 0.5));
    }

    discrete_class_interval
}

fn find_frequency(only_numbers: Vec<i32>, class_interval: Vec<(i32, i32)>) -> Vec<i32> {
    let mut frequency = vec![];

    for (start, end) in class_interval {
        let mut count = 0;
        for number in only_numbers.iter() {
            if start <= *number && *number <= end {
                count += 1;
            }
        }
        frequency.push(count);
    }

    frequency
}

fn find_cumulative_frequency(frequency: Vec<i32>) -> Vec<i32> {
    let mut cumulative_frequency = vec![];
    let mut sum = 0;
    for count in frequency {
        sum += count;
        cumulative_frequency.push(sum);
    }
    cumulative_frequency
}

fn find_mid_points(class_interval: Vec<(i32, i32)>) -> Vec<f32> {
    let mut mid_point = vec![];
    for (start, end) in class_interval {
        mid_point.push((start + end) as f32 / 2.0);
    }
    mid_point
}

fn find_fixi(frequency: Vec<i32>, mid_points: Vec<f32>) -> Vec<f32> {
    let mut fixi = vec![];
    for (count, mid_point) in frequency.iter().zip(mid_points) {
        fixi.push(*count as f32 * mid_point);
    }
    fixi
}

fn find_frequency_sum(frequency: Vec<i32>) -> i32 {
    let mut sum = 0;
    for count in frequency {
        sum += count;
    }
    sum
}

fn find_fixi_sum(fixi: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for count in fixi {
        sum += count;
    }
    sum
}

fn find_arithmetic_mean(frequency_sum: i32, fixi_sum: f32) -> f32 {
    fixi_sum / frequency_sum as f32
}

fn find_a(frequency: Vec<i32>, mid_points: Vec<f32>) -> f32 {
    // find the largest frequency
    // find the index of the largest frequency
    // that index is the index of the mid point
    // that mid point is the a

    let mut largest_frequency = 0;
    let mut largest_frequency_index = 0;

    for (index, count) in frequency.iter().enumerate() {
        if *count > largest_frequency {
            largest_frequency = *count;
            largest_frequency_index = index;
        }
    }

    mid_points[largest_frequency_index]
}

fn find_step_deviations(a: f32, mid_points: Vec<f32>, class_interval_diff_length: i32) -> Vec<f32> {
    // find the index of the a (in mid_points)
    // that index will be 0 for the array
    // then before that 0 there will be negative numbers like -1, -2, -3
    // then after that 0 there will be positive numbers like 1, 2, 3
    // -4, -3, -2, -1, 0, 1, 2, 3
    //              // ^
    //              // a

    let mut step_deviations = vec![];

    // let mut index = 0;
    // for (i, mid_point) in mid_points.iter().enumerate() {
    //     if *mid_point == a {
    //         index = i;
    //         break;
    //     }
    // }

    // let mut i = index as i32;
    // while i >= 0 {
    //     step_deviations.push(-i);
    //     i -= 1;
    // }

    // let mut i = 1;
    // while i < index as i32 {
    //     step_deviations.push(i);
    //     i += 1;
    // }

    for mid_point in mid_points {
        step_deviations.push(((mid_point - a) / (class_interval_diff_length - 1) as f32));
    }

    step_deviations
}

fn find_fiui(frequency: Vec<i32>, step_deviations: Vec<f32>) -> Vec<f32> {
    let mut fiui = vec![];
    for (count, step_deviation) in frequency.iter().zip(step_deviations) {
        fiui.push(*count as f32 * step_deviation);
    }
    fiui
}

fn find_fiui_sum(fiui: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for count in fiui {
        sum += count;
    }
    sum
}

fn find_short_cut_mean(
    a: f32,
    fiui_sum: f32,
    frequency_sum: i32,
    class_interval_diff_length: i32,
) -> f32 {
    a + (fiui_sum / frequency_sum as f32) * (class_interval_diff_length - 1) as f32
}

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_find_range() {
        let smallest = 35;
        let largest = 90;
        let range = find_range(smallest, largest);
        assert_eq!(range, 56.);
    }

    #[test]
    fn test_find_class_interval() {
        let smallest = 6;
        let largest = 14;
        let class_interval_length = 3;
        let class_interval = find_class_interval(smallest, largest, class_interval_length);
        assert_eq!(class_interval, vec![(6, 8), (9, 11), (12, 14),]);
    }

    #[test]
    fn test_find_discrete_class_interval() {
        let class_interval = vec![(46, 50), (51, 55), (56, 60), (61, 65), (66, 70)];
        let discrete_class_interval = find_discrete_class_interval(class_interval);
        assert_eq!(
            discrete_class_interval,
            vec![
                (45.5, 50.5),
                (50.5, 55.5),
                (55.5, 60.5),
                (60.5, 65.5),
                (65.5, 70.5)
            ]
        );
    }

    #[test]
    fn test_find_frequency() {
        let only_numbers = vec![
            70, 40, 35, 60, 55, 58, 45, 60, 65, 80, 70, 46, 50, 60, 65, 70, 58, 60, 48, 70, 36, 85,
            60, 50, 46, 65, 55, 61, 72, 85, 90, 68, 65, 50, 40, 56, 60, 65, 46, 76,
        ];
        let class_interval = vec![
            (35, 39),
            (40, 44),
            (45, 49),
            (50, 54),
            (55, 59),
            (60, 64),
            (65, 69),
            (70, 74),
            (75, 79),
            (80, 84),
            (85, 89),
            (90, 94),
        ];
        let frequency = find_frequency(only_numbers, class_interval);
        assert_eq!(frequency, vec![2, 2, 5, 3, 5, 7, 6, 5, 1, 1, 2, 1]);
    }

    #[test]
    fn test_find_cumulative_frequency() {
        let frequency = vec![2, 2, 5, 3, 5, 7, 6, 5, 1, 1, 2, 1];
        let cumulative_frequency = find_cumulative_frequency(frequency);
        assert_eq!(
            cumulative_frequency,
            vec![2, 4, 9, 12, 17, 24, 30, 35, 36, 37, 39, 40]
        );
    }

    #[test]
    fn test_find_mid_points() {
        let class_interval = vec![(46, 50), (51, 55), (56, 60), (61, 65), (66, 70)];
        let mid_points = find_mid_points(class_interval);
        assert_eq!(mid_points, vec![48.0, 53.0, 58.0, 63.0, 68.0]);
    }

    #[test]
    fn test_find_fixi() {
        let frequency = vec![5, 10, 15, 20, 30, 16, 4];
        let mid_points = vec![29.5, 39.5, 49.5, 59.5, 69.5, 79.5, 89.5];

        let fixi = find_fixi(frequency, mid_points);

        assert_eq!(
            fixi,
            vec![147.5, 395.0, 742.5, 1190.0, 2085.0, 1272.0, 358.0]
        );
    }

    #[test]
    fn test_find_frequency_sum() {
        let frequency = vec![5, 10, 15, 20, 30, 16, 4];
        let frequency_sum = find_frequency_sum(frequency);
        assert_eq!(frequency_sum, 100);
    }

    #[test]
    fn test_find_fixi_sum() {
        let fixi = vec![147.5, 395.0, 742.5, 1190.0, 2085.0, 1272.0, 358.0];
        let fixi_sum = find_fixi_sum(fixi);
        assert_eq!(fixi_sum, 6190.0);
    }

    #[test]
    fn test_find_arithmetic_mean() {
        let frequency_sum = 100;
        let fixi_sum = 6190.0;
        let arithmetic_mean = find_arithmetic_mean(frequency_sum, fixi_sum);
        assert_eq!(arithmetic_mean, 61.9);
    }

    #[test]
    fn test_find_a() {
        let mid_points = vec![4., 8., 12., 16., 20., 24., 28., 32.];
        let frequency = vec![1, 9, 21, 47, 52, 36, 19, 3];
        let a = find_a(frequency, mid_points);
        assert_eq!(a, 20 as f32);
    }

    #[test]
    fn test_find_step_deviations() {
        let mid_points = vec![4., 8., 12., 16., 20., 24., 28., 32.];
        let a = 20 as f32;
        let class_interval_diff_length = 5;
        let step_deviations = find_step_deviations(a, mid_points, class_interval_diff_length);
        assert_eq!(step_deviations, vec![-4., -3., -2., -1., 0., 1., 2., 3.]);
    }

    #[test]
    fn test_find_fiui() {
        let frequency = vec![1, 9, 21, 47, 52, 36, 19, 3];
        let step_deviations = vec![-4., -3., -2., -1., 0., 1., 2., 3.];
        let fiui = find_fiui(frequency, step_deviations);
        assert_eq!(fiui, vec![-4., -27., -42., -47., 0., 36., 38., 9.]);
    }

    #[test]
    fn test_find_fiui_sum() {
        let fiui = vec![-4., -27., -42., -47., 0., 36., 38., 9.];
        let fiui_sum = find_fiui_sum(fiui);
        assert_eq!(fiui_sum, -37.0);
    }

    #[test]
    fn test_find_short_cut_mean() {
        let fiui_sum = -37.;
        let frequency_sum = 188;
        let class_interval_diff_length = 5;
        let a = 20 as f32;
        let short_cut_mean =
            find_short_cut_mean(a, fiui_sum, frequency_sum, class_interval_diff_length);
        assert_eq!(short_cut_mean, 19.212767);
    }
}

// 70 40 35 60 55 58 45 60 65 80 70 46 50 60 65 70 58 60 48 70 76 85 60 50 46 65 55 61 72 85 90 68 65 50 40 56 60 65 46 76
