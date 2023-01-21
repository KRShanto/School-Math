use crate::data::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, KeyboardEvent};
use weblog::console_warn;
use webru::document;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    let data = use_state(Data::default);

    html! {
        <div>
            <ContextProvider<UseStateHandle<Data>> context={data.clone()}>
                <>
                    <TakeInput  />
                    <ShowOutput  />
                </>
            </ContextProvider<UseStateHandle<Data>>>
        </div>
    }
}

#[function_component]
fn TakeInput() -> Html {
    let data = use_context::<UseStateHandle<Data>>().unwrap();
    let class_interval_input_values: UseStateHandle<Vec<[Option<i32>; 3]>> =
        use_state(|| vec![[None, None, None], [None, None, None]]);

    html! {
        <div class="take-input">
            <div class="only-numbers">
                <label for="only-numbers">{"শুধু সংখ্যা লিখুন"}</label>
                <input
                    type="text"
                    id="only-numbers"
                    name="only-numbers"
                />
                <button
                    onclick={
                        let data = data.clone();
                        move |_| {
                            let input = document().get_element_by_id("only-numbers").unwrap();
                            let input = input.dyn_into::<HtmlInputElement>().unwrap();
                            let input = input.value();
                            let input = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                            // TODO: handle error
                            let data_using_numbers = Data::new_using_only_numbers((input).clone());
                            data.set(data_using_numbers);
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
                                {
                                    for (0..3).into_iter().map(|input_index| {
                                        html! {
                                            <input
                                                type="number"
                                                id={format!("class-interval-{}-0", index)}
                                                name={format!("class-interval-{}", index)}
                                                value={format!("{}", class_interval[input_index].unwrap_or(0))}
                                                onkeyup={
                                                    let class_interval_input_values = class_interval_input_values.clone();
                                                    move |event: KeyboardEvent| {
                                                        let input = event.target().unwrap();
                                                        let input = input.dyn_into::<HtmlInputElement>().unwrap();
                                                        let input = input.value();

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
                                                }
                                            />
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
                        // add a new value
                        let mut values = (*class_interval_input_values).clone();
                        values.push([None, None, None]);

                        class_interval_input_values.set(values);
                    }
                }>
                    {"Add"}
                </button>

                <button onclick={
                    move |_| {
                        let mut class_interval_frequency = vec![];
                        for class_inerval in (*class_interval_input_values).clone() {
                            let lower = class_inerval[0];
                            let upper = class_inerval[1];
                            let frequency = class_inerval[2];

                            if lower.is_none() || upper.is_none() || frequency.is_none() ||  lower.unwrap() == 0 || upper.unwrap() == 0 || frequency.unwrap() == 0 {
                                // TODO: warn the user
                                console_warn!("Please fill all the fields");
                                // make the class_interval empty
                                class_interval_frequency = vec![];
                                // break the loop
                                break;
                            }

                            class_interval_frequency.push((lower.unwrap(), upper.unwrap(), frequency.unwrap()) );
                        }


                        let new_data = Data::new_using_class_interval_frequency(class_interval_frequency);
                        data.set(new_data);
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
            if let Some(class_interval) = data.class_interval.clone() {
                <div class="only-numbers">
                    if let Some(only_numbers) = data.only_numbers.clone() {
                        // Show some details
                        <p>{"সংখ্যাগুলো: "}{only_numbers.iter().map(|x| format!("{}, ", x)).collect::<String>()}</p>
                        <p>{"সর্বোচ্চ: "}{data.largest.unwrap_or(-1)}</p>
                        <p>{"সর্বনিম্ন: "}{data.smallest.unwrap_or(-1)}</p>
                        <p>{"পরিসর: "}{data.range.unwrap_or(-1.)}</p>
                        <p>{"শ্রেণী ব্যপ্তি: "}{format!("{:?}", class_interval)}</p>
                        // <p>{"লাইন নাম্বার: "}{data.class_interval_length.unwrap()}</p>
                        <p>{"শ্রেণী ব্যবধান: "}{data.class_interval_diff_length.unwrap()}</p>
                        <p>{"গনসংখ্যাগুলার সমষ্টি: "}{data.frequency_sum.unwrap()}</p>
                        <p>{"fixi সমষ্টি: "}{data.fixi_sum.unwrap()}</p>
                        <p>{"গাণিতিক গড়: "}{data.arithmetic_mean.unwrap()}</p>
                        <p>{"সংক্ষিপ্ত গড়: "}{data.short_cut_mean.unwrap()}</p>
                        <p>{"a: "}{data.a.unwrap()}</p>
                        <p>{"fiui সমষ্টি: "}{data.fiui_sum.unwrap()}</p>
                    }
                    // Make a table
                    <table style="border-collapse: collapse;border: 3px solid yellow;">
                        <tr style="border: 3px solid yellow; padding: 5px;">
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
                            (0..class_interval.len() as usize).into_iter().map(|i| {
                                html! {
                                    <tr>
                                        <td>
                                            {format!(
                                                "{} - {}",
                                                class_interval[i].0,
                                                class_interval[i].1
                                            )}
                                        </td>
                                        <td>{data.frequency.clone().unwrap()[i]}</td>
                                        <td>
                                            {format!(
                                                "{} - {}",
                                                data.discrete_class_interval.clone().unwrap()[i].0,
                                                data.discrete_class_interval.clone().unwrap()[i].1
                                            )}
                                        </td>
                                        <td>{data.cumulative_frequency.clone().unwrap()[i]}</td>
                                        <td>{data.mid_point.clone().unwrap()[i]}</td>
                                        <td>{data.fixi.clone().unwrap()[i]}</td>
                                        <td>{data.fiui.clone().unwrap()[i]}</td>
                                        <td>{data.step_deviations.clone().unwrap()[i]}</td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }
                </table>
            </div>
            }

        </div>
    }
}
