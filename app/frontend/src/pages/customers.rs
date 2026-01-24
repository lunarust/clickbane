use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;
use gloo::timers::callback::Timeout;

use common::*;
use crate::pages::jobs::Jobs;

#[derive(Properties, PartialEq)]
pub struct Customers {
    customers: Vec<Customer>,
    filtered_customers: Vec<Customer>,
    selected_customer: Customer,
    reload_event: i32,
    email: String,
    ftp_user: String,
    ftp_password: String,
    ftp_url: String,
}
pub enum Msg {
    GetCustomers(Vec<Customer>),
    FilterCustomers(String),
    SelectCustomer(Customer),
    EditEmail(String),
    EditFtpURL(String),
    EditFtpUsername(String),
    EditFtpPassword(String),
    ScheduleAll(),
    DeleteAll(),
    Done(),
}

impl Component for Customers {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self::get_customers(ctx);
        Self {
            customers: vec![],
            filtered_customers: vec![],
            selected_customer: Customer::default(),
            reload_event: 0,
            email: "".to_string(),
            ftp_user: "".to_string(),
            ftp_password: "".to_string(),
            ftp_url: "".to_string(),
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCustomers(c) => {
                self.filtered_customers = c.clone();
                self.customers = c;
                true
            }
            Msg::FilterCustomers(filter) => {
                let cs: Vec<Customer> = self.customers.clone()
                    .into_iter()
                    .filter(|c| c.customerName.to_lowercase().contains(&filter.to_lowercase().as_str()))
                    .collect();
                    self.filtered_customers = cs;
                    true
            }
            Msg::SelectCustomer(c) => {
                self.selected_customer = c;
                true
            }
            Msg::EditEmail(input) => {
                self.email = input;
                false
            }
            Msg::EditFtpURL(input) => {
                self.ftp_url = input;
                false
            }
            Msg::EditFtpUsername(input) => {
                self.ftp_user = input;
                false
            }
            Msg::EditFtpPassword(input) => {
                self.ftp_password = input;
                false
            }
            Msg::ScheduleAll() => {
                Self::schedule_all(self.selected_customer.clone(), self.email.clone(), self.ftp_url.clone(), self.ftp_user.clone(), self.ftp_password.clone());

                // Setup a delay knowing it will take some time to submit the new reports to JasperServer
                let link = ctx.link().clone();
                let timeout = Timeout::new(1000, move || {
                    link.send_message(Msg::Done());
                });

                // Prevent the timer from dropping immediately
                timeout.forget(); //

                false
            }
            Msg::DeleteAll() => {
                Self::delete_all(self.selected_customer.clone());
               let link = ctx.link().clone();
               let timeout = Timeout::new(1500, move || {
                   link.send_message(Msg::Done()); //
               });
               timeout.forget();
               false
            }
            Msg::Done() => {
                // Finally re-render the page
                self.reload_event += 1;
                true
            }
        }
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::FilterCustomers(target.value())
        });
        let rl = self.reload_event;

        let edit_email = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditEmail(target.value())
        });
        let edit_ftpurl = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditFtpURL(target.value())
        });
        let edit_ftpusername = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditFtpUsername(target.value())
        });
        let edit_ftppassword = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::EditFtpPassword(target.value())
        });

        html!{

            <>
            <aside class="menu">
                <p class="menu-label">{ format!("Customers        {}/{}", self.filtered_customers.len(), self.customers.len()) }
                <input
                class="input"
                    placeholder="Filter..."
                    {oninput} /></p>
                <table class="table">
                <thead>
                <tr>
                    <th>{ "Number" }</th>
                    <th>{ "Name" }</th>
                    <th>{ "Contact last Name" }</th>
                </tr>
                </thead>
                <tbody>
                { for self.filtered_customers.clone().into_iter().map(|cust| {
                    let mut selected = "";
                    if cust.customerName == *self.selected_customer.customerName { selected = "is-selected"; }
                    html! {
                        <tr key={cust.customerNumber}
                            onclick={
                                let selected_customer = cust.clone();
                                ctx.link().callback(move |_| Msg::SelectCustomer(selected_customer.clone()))
                            }
                            class={selected}>
                            <td>{ &cust.customerNumber.to_string() }</td>
                            <td>{ &cust.customerName }</td>
                            <td>{ format!("{}, {}", &cust.contactLastName, &cust.contactFirstName) }</td>
                        </tr>
                    }})}
                </tbody>
                </table>
            </aside>

            <section class="section">
            <div id="details">
            if self.selected_customer.customerName != "" {
                <span class="title is-4">{ self.selected_customer.customerName.to_string() }</span>

                <p>
                    <Jobs customer={ self.selected_customer.clone() } reload={ rl } />
                </p>
                <hr />


                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Email Address:" }</label></div>
                    <div class="field-body"><div class="field"><p class="control">
                    <input
                        class="input is-normal"
                        value={ self.selected_customer.clone().email }
                        oninput={ edit_email }
                    />
                    </p></div></div>
                </div>
                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "FTP URL:" }</label></div>
                    <div class="field-body"><div class="field"><p class="control">
                    <input
                        placeholder="ftp url"
                        class="input is-normal"
                        value={ self.ftp_url.clone() }
                        oninput={ edit_ftpurl }
                    />
                    </p></div></div>
                </div>
                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "FTP Username:" }</label></div>
                    <div class="field-body"><div class="field"><p class="control">
                    <input
                        placeholder="ftp username"
                        class="input is-normal"
                        value={ self.ftp_user.clone() }
                        oninput={ edit_ftpusername }
                    />
                    </p></div></div>
                </div>
                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "FTP Password:" }</label></div>
                    <div class="field-body"><div class="field"><p class="control">
                    <input
                        placeholder="ftp password"
                        class="input is-normal"
                        value={ self.ftp_password.clone() }
                        oninput={ edit_ftppassword }
                    />
                    </p></div></div>
                </div>

                <div class="buttons are-small">
                    <button class="button is-hovered" onclick={ctx.link().callback(move |_| Msg::ScheduleAll())}>{ " Schedule all " }</button>
                    <button class="button is-hovered" onclick={ctx.link().callback(move |_| Msg::DeleteAll())}>{ " Remove all " }</button>
                </div>

            }
            else {
                <span class="title">{ "Select a customer to see details" }</span>
            }
            </div>
            </section>
            </>
        }
    }
}

impl Customers {
    fn get_customers(ctx: &Context<Self>) {
        let link = ctx.link().clone();
        spawn_local(async move {
            let fetched_customers: Vec<Customer> = Request::get("http://localhost:9000/clients")
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            link.send_message(Msg::GetCustomers(fetched_customers));
        });
    }
    fn schedule_all(mut c: Customer, email: String, uri: String, user: String, pass: String) {

         spawn_local(async move {
             c.email = email;
             let build_body: CustomerJobSchedule = CustomerJobSchedule {
                 customer: c.clone(),
                 ftpHost: uri.to_string(),
                 ftpUser: user.to_string(),
                 ftpPassword: pass.to_string(),
             };
             let url = "http://localhost:9000/jasper/all";
             let json_body = serde_json::to_string(&build_body).expect("Some DRAMA");
             let _ = Request::post(url)
                 .header("Content-Type", "application/json")
                 .body(json_body.to_string()).expect("Still DRAMA")
                 .send()
                 .await;
         });
    }
    fn delete_all(c: Customer) {
        spawn_local(async move {
            let url = format!("http://localhost:9000/jasper/remove/");
            let json_body = serde_json::to_string(&c.clone()).expect("DRAMA");
            let _ = Request::delete(url.as_str())
            .header("Content-Type", "application/json")
            .body(json_body.to_string()).expect("Still DRAMA")
            .send()
            .await;
        });
    }
}
