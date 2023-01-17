#![allow(unused)]

use data::*;
use wasm_bindgen::JsCast;
use weblog::console_log;
use webru::document;
use yew::prelude::*;

mod data;
pub mod find;

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

fn main() {
    yew::Renderer::<App>::new().render();
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
                    let data_using_numbers = Data::new_using_only_numbers((**only_numbers).clone());
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
