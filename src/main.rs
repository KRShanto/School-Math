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

#[derive(Clone, Debug, PartialEq)]
struct Data {
    only_numbers: Option<Vec<i32>>,
    frequency: Option<Vec<i32>>,
    class_interval: Option<Vec<(i32, i32)>>,
    class_interval_length: Option<i32>,
    class_interval_diff_length: Option<i32>,
    discrete_class_interval: Option<Vec<(f32, f32)>>,
    range: Option<i32>,
    cumulative_frequency: Option<Vec<i32>>,
    mid_point: Option<Vec<f32>>,
    smallest: Option<i32>,
    largest: Option<i32>,
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
    let data = use_state(|| Data {
        only_numbers: None,
        frequency: None,
        class_interval: None,
        class_interval_length: None,
        discrete_class_interval: None,
        class_interval_diff_length: None,
        range: None,
        cumulative_frequency: None,
        mid_point: None,
        smallest: None,
        largest: None,
    });
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
                    <th style="border: 3px solid yellow; padding: 5px;">{"গনসংখ্যা"} </th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"অবিচ্ছিন্ন শ্রেণীসীমা"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"ক্রমযোজিত গনসংখ্যা"}</th>
                    <th style="border: 3px solid yellow; padding: 5px;">{"মধ্যমান"}</th>
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
    }
}

fn find_range(smallest: i32, largest: i32) -> i32 {
    (largest - smallest) + 1
}

fn find_class_interval_length(range: i32, class_interval_diff_length: i32) -> i32 {
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
    while end <= (largest + class_interval_diff_length) {
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

#[cfg(test)]
mod tester {
    use super::*;

    #[test]
    fn test_find_range() {
        let smallest = 35;
        let largest = 90;
        let range = find_range(smallest, largest);
        assert_eq!(range, 56);
    }

    #[test]
    fn test_find_class_interval() {
        let smallest = 1;
        let largest = 20;
        let class_interval_length = 5;
        let class_interval = find_class_interval(smallest, largest, class_interval_length);
        assert_eq!(class_interval, vec![(1, 5), (6, 10), (11, 15), (16, 20)]);
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
}

// 70 40 35 60 55 58 45 60 65 80 70 46 50 60 65 70 58 60 48 70 76 85 60 50 46 65 55 61 72 85 90 68 65 50 40 56 60 65 46 76
