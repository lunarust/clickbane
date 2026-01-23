use yew::prelude::*;
use yew::suspense::{use_future};
use wasm_bindgen_futures::spawn_local;

use gloo_net::http::Request;
use common::*;

use crate::pages::jobs::Jobs;

#[derive(Properties, PartialEq)]
pub struct ShowListProps {
    pub on_click: Callback<Customer>
}

#[function_component(CustomersDisplay)]
pub fn customers_display( ShowListProps { on_click }: &ShowListProps ) -> HtmlResult {
    let selected_customer = use_state(|| Customer::default());
    let search_state = use_state(|| "".to_string());

    let customers_handle = use_future(|| async {
        Request::get("http://localhost:9000/clients")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Vec<Customer>>()
        .await
    })?;

    let customers_all = use_state(|| match &*customers_handle {
        Ok(v) => v.clone(),
        Err(_) => vec![],
    });

    if let Err(e) = &*customers_handle {
        return Ok(html! { <p class="footer">{ "Error loading customers: "}{ e }</p> });
    }

    //let displayed_customers = customers_all.iter();
    let displayed_customers: Vec<&Customer> =
        customers_all.iter()
        .filter(|c| c.customerName.to_lowercase().contains(&search_state.to_lowercase().as_str()))
        .collect();

    let on_select = | cust: &Customer| {
        let on_click = on_click.clone();
        let cust = cust.clone();
        Callback::from(move |_| {
            on_click.emit(cust.clone())
        })
    };


    Ok(
        html! {
            <>
            <p class="menu-label">{ "Customers        " }
            <input
            class="input"
                placeholder="Search..."
                oninput={Callback::from(
                    move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        search_state.set(input.value());
                    }
                )}
            /></p>
            <table class="table">
            <thead>
            <tr>
                <th>{ "Number" }</th>
                <th>{ "Name" }</th>
                <th>{ "Contact last Name" }</th>
            </tr>
            </thead>
            <tbody>
            { for displayed_customers.into_iter().map(|cust| {
                let mut selected = "";
                if cust.customerName == *selected_customer.customerName { selected = "is-selected"; }
                html! {
                    <tr key={cust.customerNumber}
                        onclick={on_select(cust)} class={selected}>
                        <td>{ &cust.customerNumber.to_string() }</td>
                        <td>{ &cust.customerName }</td>
                        <td>{ format!("{}, {}", &cust.contactLastName, &cust.contactFirstName) }</td>
                    </tr>
                }
            })}
            </tbody>
            </table>
            </>
    })
}

#[function_component(Customers)]
pub fn loading() -> Html {

    let selected_customer = use_state(|| None);
    let sftp_state = use_state(|| ("".to_string(), "".to_string(), "".to_string()));

    let on_customer_select = {
        let selected_customer = selected_customer.clone();
        Callback::from(move |cust: Customer| {
            selected_customer.set(Some(cust))
        })
    };
    let fallback = html! { <div>{ "Loading customers..." }</div> };

    let sftp_state = sftp_state.clone();
    let value = selected_customer.clone();
    let sftpvalue = sftp_state.clone();
    let delete_all = {
        Callback::from(move |_| {
            let selected_customer = value.clone();
            spawn_local(async move {
                let url = format!("http://localhost:9000/jasper/remove/");
                let json_body = serde_json::to_string(&selected_customer.as_ref().unwrap().clone()).expect("DRAMA");
                let _ = Request::delete(url.as_str())
                .header("Content-Type", "application/json")
                .body(json_body.to_string()).expect("Still DRAMA")
                .send()
                .await;
            });
        })
    };
    let value = selected_customer.clone();
    let schedule_all = {
        Callback::from(move |_| {
            let sftp_state = sftpvalue.clone();
            let selected_customer = value.clone();
            spawn_local(async move {
                let build_body: CustomerJobSchedule = CustomerJobSchedule {
                    customer: selected_customer.as_ref().unwrap().clone(),
                    ftpHost: sftp_state.0.clone(),
                    ftpUser: sftp_state.1.clone(),
                    ftpPassword: sftp_state.2.clone(),
                };
                let url = "http://localhost:9000/jasper/all";
                let json_body = serde_json::to_string(&build_body).expect("Some DRAMA");
                let _ = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(json_body.to_string()).expect("Still DRAMA")
                    .send()
                    .await;
            });

        })
    };

    html! {
        <>

        <aside class="menu">
            <Suspense {fallback}>
                <CustomersDisplay on_click={ on_customer_select } />
            </Suspense>
        </aside>
        <section class="section">
        <div id="details">
        if let Some(cust) = &*selected_customer {
            <span class="title is-4">{ cust.customerName.to_string() }</span>
            <Jobs customer={ cust.clone() } />


            <span id="form">
            <h4 class="title is-5">{ "Schedule all default reports for this customer:" }</h4>
            <br />

            <div class="field is-horizontal">
                <div class="field-label is-normal"><label class="label">{ "Email Address:" }</label></div>
                <div class="field-body">
                    <div class="field">
                        <p class="control">
                            <input
                                class="input is-normal"
                                value={ cust.clone().email }
                            />
                        </p>
                    </div>
                </div>
            </div>

            <div class="field is-horizontal">
                <div class="field-label is-normal"><label class="label">{ "sftp drop: " }</label></div>
                <div class="field-body">
                    <div class="field">
                        <p class="control">
                            <input
                                value={ sftp_state.0.clone() }
                                class="input is-normal"
                                placeholder="sftpaddress"
                                oninput={Callback::from({
                                let sftp_state = sftp_state.clone();
                                move |e: InputEvent| {
                                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    sftp_state.set(
                                        (input.value(), sftp_state.1.clone(), sftp_state.2.clone())
                                    );
                                }})}
                            />
                        </p>
                    </div>
                </div>
            </div>


            <div class="field is-horizontal">
            <div class="field-label is-normal"><label class="label">{"username: "}</label></div>
                <div class="field-body">
                    <div class="field">
                        <p class="control">
                        <input
                            value={ sftp_state.1.clone() }
                            class="input is-normal"
                            placeholder="username"
                            oninput={Callback::from({
                            let sftp_state = sftp_state.clone();
                            move |e: InputEvent| {
                               let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                               sftp_state.set(
                                   (sftp_state.0.clone(), input.value(), sftp_state.2.clone())
                               );
                            }})}
                        />
                        </p>
                    </div>
                </div>
            </div>


            <div class="field is-horizontal">
                <div class="field-label is-normal"><label class="label">{"password: "}</label></div>
                <div class="field-body">
                    <div class="field">
                        <p class="control">
                        <input
                            value={ sftp_state.2.clone() }
                            class="input is-normal"
                            placeholder="password"
                            oninput={Callback::from({
                            let sftp_state = sftp_state.clone();
                            move |e: InputEvent| {
                               let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                               sftp_state.set(
                                   (sftp_state.0.clone(), sftp_state.1.clone(), input.value())
                               );
                            }})}

                        />
                        </p>
                    </div>
                </div>
            </div>

            <div class="buttons are-small">
                <button class="button is-hovered" onclick={schedule_all}>{ " Schedule all " }</button>
                <button class="button is-hovered" onclick={delete_all}>{ " Remove all " }</button>
            </div>
            </span>

        }
        else {
            <span class="title">{ "Select a customer to see details" }</span>
        }
        </div>
        </section>

        </>
    }
}
