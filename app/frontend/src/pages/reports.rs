use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use common::*;

pub struct Reports {
    reports: Vec<JS_Report>,
}
pub enum Msg {
    SetDefault(JS_Report),
    FetchJobs(Vec<JS_Report>),
    ChangeFrequency(JS_Report),
}
impl Component for Reports {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Initial fetch
        Self::fetch_reports(ctx);
        Self { reports: vec![], }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
                self.reports = reports;
                true
            }
        }
    }
    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!{
            <div>
                <h1 class="title is-4">{ " Reports " }</h1>
                <table class="table">
                <thead>
                    <tr>
                        <th>{ "Label" }</th>
                        <th>{ "Description" }</th>
                        <th>{ "Uri" }</th>
                        <th>{ "#mapped / params" }</th>
                        <th>{ "To schedule" }</th>
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
                    for rep in &*self.reports {
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
