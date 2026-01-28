use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use gloo::timers::callback::Timeout;
use common::*;
use common::jasper::{JS_Report,InputParam};

pub struct Reports {
    reports: Vec<JS_Report>,
    filtered_reports: Vec<JS_Report>,
    reload: i32,
    scheduled: bool,
}
pub enum Msg {
    SetDefault(JS_Report),
    FilterReports(),
    FetchJobs(Vec<JS_Report>),
    ChangeFrequency(JS_Report),
    SyncReport(),
    Done(),
}
impl Component for Reports {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Initial fetch
        Self::fetch_reports(ctx);
        Self { reports: vec![], filtered_reports: vec![], reload: 0, scheduled: false }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetDefault(r) => {
                Self::set_default(r);
                false
            }
            Msg::ChangeFrequency(r) => {
                Self::set_frequency(r);
                false
            }
            Msg::FetchJobs(reports) => {
                self.reports = reports.clone();
                self.filtered_reports = reports;
                true
            }
            Msg::SyncReport() => {
                Self::sync_reports();
               let link = ctx.link().clone();
               let timeout = Timeout::new(1500, move || {
                   link.send_message(Msg::Done()); //
               });
               timeout.forget();
               false
            }
            Msg::FilterReports() => {
                if self.scheduled {
                    self.scheduled = false;
                    self.filtered_reports = self.reports.clone();
                }
                else {  self.scheduled = true;
                let rp: Vec<JS_Report> = self.reports.clone()
                    .into_iter()
                    .filter(|r| r.default)
                    .collect();
                    self.filtered_reports = rp;
                }
                true
            }
            Msg::Done() => {
                self.reload += 1;
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let ontoggle = {
            ctx.link().callback(move |_| Msg::FilterReports())
        };

        html!{
            <div>
            <aside class="menu">
                <p class="menu-label">{"JS"}</p>
                <button class="button is-hovered" onclick={
                    ctx.link().callback(move |_| Msg::SyncReport())}>{"Fetch reports"}</button>
            </aside>

            <div class="card">
              <header class="card-header">
                <p class="card-header-title">{ " Reports " }</p>
                //<h1 class="title is-4">{ " Reports " }</h1>
                </header>
                <div class="card-content">
                <table class="table">
                <thead>
                    <tr>
                        <th>{ "Label" }</th>
                        <th>{ "Description" }</th>
                        <th>{ "Uri" }</th>
                        <th>{ "#mapped / params" }</th>
                        <th>
                            <div class="field is-horizontal"><div class="field-label is-normal"><label class="label">{ "schedule" }</label></div>
                            <div class="field-body"><div class="field"><p class="field">
                                <input type="checkbox"
                                checked={ self.scheduled }
                                onclick={ontoggle}
                                />
                            </p></div></div></div>
                        </th>
                        <th>{ "Daily" }</th>
                        <th>{ "Weekly" }</th>
                        <th>{ "Monthly" }</th>
                    </tr>
                    </thead>
                <tfoot>
                <tr>
                    <th colspan="7">{ "Total:" }</th>
                    <th>{ format!("{}", &self.reports.len()) }</th>
                </tr>
                </tfoot>
                <tbody>
                    for rep in &*self.filtered_reports {
                        <tr key={ rep.uri.clone() }>
                            <td>{ &rep.label }</td>
                            <td>{ &rep.description }</td>
                            <td>{ &rep.uri }</td>
                            <td>
                            {
                                rep.param.iter()
                                .filter(|r| r.mapped==Some(1))
                                .collect::<Vec<&InputParam>>().len()
                            }{"/"}
                            {&rep.param.len()}
                            </td>
                            <td>
                                <label class="checkbox">
                                <input type="checkbox" checked={rep.default}
                                    onchange={
                                        let mut report_checked = rep.clone();
                                        report_checked.default = match rep.default {
                                            true => false,
                                            _ => true, };
                                        ctx.link().callback(move |_| Msg::SetDefault(report_checked.clone()))
                                    }
                                />
                                </label>
                            </td>
                            <td>
                                <label class="checkbox">
                                <input type="checkbox" checked={
                                    match rep.frequency[0] {
                                        1 => true,
                                        _ => false
                                    }}
                                    onchange={
                                            let mut rc = rep.clone();
                                            rc.frequency[0] = match rep.frequency[0] {
                                                1 => 0,
                                                _ => 1,
                                            };
                                            ctx.link().callback(move |_| Msg::ChangeFrequency(rc.clone()))
                                    }
                                />
                                </label>
                            </td>
                            <td>
                                <label class="checkbox">
                                <input type="checkbox" checked={
                                    match rep.frequency[1] {
                                        1 => true,
                                        _ => false
                                    }}
                                    onchange={
                                            let mut rc = rep.clone();
                                            rc.frequency[1] = match rep.frequency[1] {
                                                1 => 0,
                                                _ => 1,
                                            };
                                            ctx.link().callback(move |_| Msg::ChangeFrequency(rc.clone()))
                                    }
                                />
                                </label>
                            </td>
                            <td>
                                <label class="checkbox">
                                <input type="checkbox" checked={
                                    match rep.frequency[2] {
                                        1 => true,
                                        _ => false
                                    }}
                                    onchange={
                                            let mut rc = rep.clone();
                                            rc.frequency[2] = match rep.frequency[2] {
                                                1 => 0,
                                                _ => 1,
                                            };
                                            ctx.link().callback(move |_| Msg::ChangeFrequency(rc.clone()))
                                    }
                                />
                                </label>
                            </td>

                        </tr>
                    }
                </tbody>
                </table>
                </div></div>
            </div>
        }
    }
}

impl Reports {
    fn fetch_reports(ctx: &Context<Self>) {

        let link = ctx.link().clone();
        spawn_local(async move {

            let fetched_reports: Vec<JS_Report> = Request::get("http://localhost:9000/jasper/fetch")
                .header("Content-Type", "application/json")
                //.expect("Failed to build request")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            // Send message back to update component state
            link.send_message(Msg::FetchJobs(fetched_reports));

        });
    }
    fn sync_reports() {
        spawn_local(async move {
            let fetched_reports: Vec<JS_Report> = Request::get("http://localhost:9000/jasper/sync")
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");
        });
    }
    fn set_default(rep: JS_Report) {
        //todo
        let json_body = serde_json::to_string(&rep).unwrap();
        spawn_local(async move {
            let _ = Request::post("http://localhost:9000/jasper/default")
                .header("Content-Type", "application/json")
                .body(json_body).expect("DRAMA")
                .send()
                .await
                .unwrap();
        });
    }
    fn set_frequency(rep: JS_Report) {
        let json_body = serde_json::to_string(&rep).unwrap();
        spawn_local(async move {
            let _ = Request::post("http://localhost:9000/jasper/frequency")
                .header("Content-Type", "application/json")
                .body(json_body).expect("DRAMA")
                .send()
                .await
                .unwrap();
        });
    }
}
