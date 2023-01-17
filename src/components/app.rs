use crate::data::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};
use weblog::{console_log, console_warn};
use webru::document;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
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
    let frequency_class_interval =
        use_context::<UseStateHandle<Option<(Vec<i32>, Vec<(i32, i32)>)>>>().unwrap();
    // let class_interval_input_length = use_state(|| 2);
    // let class_interval_input_values =
    //     use_state::<Vec<(Option<i32>, Option<i32>)>>(|| vec![(None, None), (None, None)]);
    let class_interval_input_values: UseStateHandle<Vec<[Option<i32>; 2]>> =
        use_state(|| vec![[None, None], [None, None]]);

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
                            let input = input.dyn_into::<HtmlInputElement>().unwrap();
                            let input = input.value();
                            let input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                            only_numbers.set(input);
                            show_output.set(true);
                    }}>
                    {"Calculate"}
                </button>
            </div>

            <div class="frequency-class-interval">
                <p>{"শ্রেণী ব্যবধানগুলো লিখুন"}</p>
                {
                    for class_interval_input_values.iter().enumerate().map(|(index, class_interval)| {
                        html! {
                            <div class="class-interval">
                                // <input type="text" id={format!("class-interval-{}-0", i)} name={format!("class-interval-{}", i)} />
                                // <input type="text" id={format!("class-interval-{}-1", i)} name={format!("class-interval-{}", i)} />

                                {
                                    for (0..2).into_iter().map(|input_index| {
                                        html! {

                                            <input type="number" id={format!("class-interval-{}-0", index)} name={format!("class-interval-{}", index)} value={format!("{}", class_interval[input_index].unwrap_or(0))} onkeyup={
                                                let class_interval_input_values = class_interval_input_values.clone();
                                                move |event: KeyboardEvent| {
                                                    let input = event.target().unwrap();
                                                    let input = input.dyn_into::<HtmlInputElement>().unwrap();
                                                    let input = input.value();
                                                    // let input = input.parse::<i32>().unwrap();
                                                    // let mut class_interval_input_values = class_interval_input_values.clone();
                                                    // class_interval_input_values[class_interval].0 = Some(input);

                                                    // check if the this is a number or not
                                                    if let Ok(input) = input.parse::<i32>() {
                                                        let mut values = (*class_interval_input_values).clone();
                                                        values[index][input_index] = Some(input);
                                                        class_interval_input_values.set(values);
                                                    } else {
                                                        let mut values = (*class_interval_input_values).clone();
                                                        values[index][input_index] = None;
                                                        class_interval_input_values.set(values);
                                                    }

                                                }
                                            } />
                                        }
                            })
                        }


                                </div>
                            }

                    })
                }
                <button onclick={
                    let class_interval_input_values = class_interval_input_values.clone();
                    move |_| {
                        // let new_length = *class_interval_input_length + 1;
                        // class_interval_input_length.set(new_length);

                        // add a new value
                        let mut values = (*class_interval_input_values).clone();
                        values.push([None, None]);

                        class_interval_input_values.set(values);
                    }
                }>
                    {"Add"}
                </button>

                <button onclick={
                    let show_output = show_output.clone();
                    move |_| {
                        let mut class_interval = vec![];
                        for class_inerval in (*class_interval_input_values).clone() {
                            // let lower = document().get_element_by_id(&format!("class-interval-{}-0", i)).unwrap();
                            // let lower = lower.dyn_into::<web_sys::HtmlInputElement>().unwrap();

                            let lower = class_inerval[0];
                            let upper = class_inerval[1];

                            if lower.is_none() || upper.is_none() || lower.unwrap() == 0 || upper.unwrap() == 0 {
                                // TODO: warn the user
                                console_warn!("Please fill all the fields");
                                // make the class_interval empty
                                class_interval = vec![];
                                // break the loop
                                break;
                            }

                            // let lower = lower.value().parse::<i32>().unwrap();

                            // let upper = document().get_element_by_id(&format!("class-interval-{}-1", i)).unwrap();
                            // let upper = upper.dyn_into::<web_sys::HtmlInputElement>().unwrap();

                            // if upper.value().is_empty() {
                            //     // warn the user
                            //     console_warn!("Please fill all the fields");
                            //     // make the class_interval empty
                            //     class_interval = vec![];
                            //     // break the loop
                            //     break;
                            // }

                            // let upper = upper.value().parse::<i32>().unwrap();

                            class_interval.push((lower.unwrap(), upper.unwrap()));


                        }

                        // frequency_class_interval.set(Some((vec![], class_interval)));
                        // show_output.set(true);
                        console_log!("Class interval: ", format!("{:#?}", class_interval));
                        console_log!("Class interval input values: ", format!("{:#?}", class_interval_input_values));
                    }
                }
                >
                    {"Calculate"}
                </button>
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
                <p>{"সংখ্যাগুলো: "}{only_numbers.iter().map(|x| format!("{}, ", x)).collect::<String>()}</p>
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