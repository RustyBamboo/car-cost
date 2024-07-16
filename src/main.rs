#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut bat_capacity_kwh = use_signal(|| 8.8);
    let mut gas_capacity_gal = use_signal(|| 11.3);
    let mut miles_per_kwh = use_signal(|| 4.0);
    let mut miles_per_gal = use_signal(|| 50.0);
    let mut dollars_per_kwh = use_signal(|| 0.12);
    let mut dollars_per_gal = use_signal(|| 3.00);

    let mut total_miles_electric = use_signal(|| 0.0);
    let mut total_cost_kwh = use_signal(|| 0.0);
    let mut miles_per_dollar_electric = use_signal(|| 0.0);

    let mut total_miles_gas = use_signal(|| 0.0);
    let mut miles_per_dollar_gas = use_signal(|| 0.0);

    let mut calculate_statistics = move || {
        total_miles_electric.set((*bat_capacity_kwh)() * (*miles_per_kwh)());
        total_cost_kwh.set((*bat_capacity_kwh)() * (*dollars_per_kwh)());
        miles_per_dollar_electric.set((*miles_per_kwh)() / (*dollars_per_kwh)());

        total_miles_gas.set((*gas_capacity_gal)() * (*miles_per_gal)());
        miles_per_dollar_gas.set((*miles_per_gal)() / (*dollars_per_gal)());
        // let total_cost_gal = total_miles / *miles_per_gal * *dollars_per_gal;
    };
    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        fieldset {
            legend {"Electric"}
            label { r#for: "bat_capacity_kwh", "Battery Capacity (kWh):" }
            input {
                value: "{bat_capacity_kwh}",
                id: "bat_capacity_kwh",
                oninput: move |event| {bat_capacity_kwh.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
            label { r#for: "miles_per_kwh", "Miles per kWh:" }
            input {
                value: "{miles_per_kwh}",
                oninput: move |event| {miles_per_kwh.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
            label { r#for: "dollars_per_kwh", "Dollars per kWh:" }
            input {
                value: "{dollars_per_kwh}",
                oninput: move |event| {dollars_per_kwh.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
        }
        fieldset {
            legend {"Gasoline"}

            label { r#for: "gas_capacity_gal", "Tank Capacity (gals):" }
            input {
                value: "{gas_capacity_gal}",
                oninput: move |event| {gas_capacity_gal.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
            label { r#for: "miles_per_gal", "Miles per gallon:" }
            input {
                value: "{miles_per_gal}",
                oninput: move |event| {miles_per_gal.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
            label { r#for: "dollars_per_gal", "Dollars per gallon:" }
            input {
                value: "{dollars_per_gal}",
                oninput: move |event| {dollars_per_gal.set(event.value().parse().unwrap_or(0.0)); calculate_statistics()}
            }
        }
        h1 {"Total miles electric: {total_miles_electric}"}
        h1 {"Total miles gas: {total_miles_gas}"}
        h1 {"Total cost: {total_cost_kwh}"}
        h1 {"miles/$ electric: {miles_per_dollar_electric}"}
        h1 {"miles/$ gas: {miles_per_dollar_gas}"}
    }
}
