use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlInputElement, InputEvent};
use gloo_net::http::Request;
use common::*;
use common::jasper::{JS_Scheduled_Job,CustomerJobRequest};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub customer: Customer,
    pub reload: i32,
}
pub struct Jobs {
    jobs: Vec<JS_Scheduled_Job>,
    filtered_jobs: Vec<JS_Scheduled_Job>,
    filter: String,
    failed: bool,
}

pub enum Msg {
    SetJobs(Vec<JS_Scheduled_Job>),
    ReplayJob(String),
    TrashJob(i32),
    FilterJobs(String),
    FailedJobs(),
}

impl Component for Jobs {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // Initial Fetch
        Self::fetch_jobs(ctx);
        Self { jobs: vec![], filtered_jobs: vec![], filter: "".to_string(), failed: false }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetJobs(jobs) => {
                // Trigger the async call
                self.filtered_jobs = jobs.clone();
                self.jobs = jobs;
                true
            }
            Msg::ReplayJob(jn) => {
                // Trigger the async call
                Self::replay_job(&jn);
                false
            }
            Msg::TrashJob(jn) => {
                // Trigger the async call
                Self::trash_job(jn);
                let js: Vec<JS_Scheduled_Job> = self.filtered_jobs.clone()
                    .into_iter()
                    .filter(|p| p.id != jn).collect();

                self.filtered_jobs = js;
                true
            }
            Msg::FilterJobs(filter) => {
                self.filter = filter.clone();
                let mut js: Vec<JS_Scheduled_Job> = vec![];
                match self.failed {
                    true =>
                        js = self.jobs.clone()
                            .into_iter()
                            .filter(|j| j.label.to_lowercase().contains(&filter.to_lowercase().as_str()))
                            .filter(|j| j.err_message != "")
                            .collect(),
                    _ =>
                        js = self.jobs.clone()
                            .into_iter()
                            .filter(|j| j.label.to_lowercase().contains(&filter.to_lowercase().as_str()))
                            .collect(),
                }
                self.filtered_jobs = js;

                true
            }
            Msg::FailedJobs() => {
                if self.failed { self.failed = false; }
                else {  self.failed = true; }
                let link = ctx.link().clone();
                link.send_message(Msg::FilterJobs(self.filter.clone()));
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().customer != _old_props.customer {
            Self::fetch_jobs(ctx);
            return true; // Re-render if needed
        }
        if ctx.props().reload != _old_props.reload {
            Self::fetch_jobs(ctx);
            return true; // Re-render if needed
        }

        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();


        let mut show_filter = false;
        if *&self.jobs.len() > 0 { show_filter = true; }

        let fltfilter = &self.filter;
        let flttoggle = &self.failed;

        let oninput = link.callback(|e: InputEvent| {
            let event: Event = e.dyn_into().unwrap_throw();
            let event_target = event.target().unwrap_throw();
            let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
            Msg::FilterJobs(target.value())
        });

        let ontoggle = {
            ctx.link().callback(move |_| Msg::FailedJobs())
        };


        html!{
            <div>
            <h1 class="title is-6">{ "Scheduled Jobs" }</h1>
            <table class="table">
            <thead>
            <tr>
                <th>
                <div class="field is-horizontal"><div class="field-label is-normal"><label class="label">{"Label"}</label></div>
                if show_filter {
                <div class="field-body"><div class="field"><p class="field">
                    <input
                    class="input is-small"
                    placeholder="Filter..."
                    {oninput} />
                </p></div></div>
                }</div>
                </th>
                <th>{"name"}</th>
                <th>{"description"}</th>
                <th>{"trigger type"}</th>
                <th>{"state"}</th>
                <th>{"previous"}</th>
                <th>{"next"}</th>
                <th>
                <div class="field is-horizontal"><div class="field-label is-normal"><label class="label">{"Error"}</label></div>
                if show_filter {
                <div class="field-body"><div class="field"><p class="field">
                    <input type="checkbox"
                    checked={ self.failed }
                    onclick={ontoggle}
                    />
                </p></div></div>
                }

                </div></th>
                <th>{"last failed"}</th>
                <th>{"Replay"}</th>
                <th>{"Trash"}</th>
            </tr>
            </thead>
            <tfoot>
            <tr>
                <th colspan="10">{ "Total:" }</th>
                <th>{ format!("{}/{}",&self.filtered_jobs.len(), &self.jobs.len()) }</th>
            </tr>
            </tfoot>
            <tbody>
                   for job in &*self.filtered_jobs {

                    <tr key={job.job_name.clone()} >
                            <td>{ &job.label }</td>
                            <td>{ &job.job_name }</td>
                            <td>{ &job.description }</td>
                            <td>{ &job.trigger_type }</td>
                            <td>{ &job.trigger_state }</td>
                            <td>{ &job.prev_fire }</td>
                            <td>{ &job.next_fire }</td>
                            <td>{ &job.occurrence_date }</td>
                            <td>{ &job.err_message }</td>
                            <td><button
                                class="button is-primary is-success"
                                onclick={
                                    let job_name = job.job_name.clone();
                                    ctx.link().callback(move |_| Msg::ReplayJob(job_name.clone()))
                                }
                                disabled={if job.failed {false} else {true}}
                                >{ "⟳" }</button></td>
                            <td><button
                                class="button is-warning is-dark"
                                onclick={
                                        let job_id = job.id.clone();
                                        ctx.link().callback(move |_| Msg::TrashJob(job_id.clone()))
                                }
                                >{ "🗑" }</button></td>
                            </tr>
                }
            </tbody>
            </table>

        </div>
     }
    }
}

impl Jobs {
    fn trash_job(jb: i32) {
        let job_to_delete = jb.clone();
        spawn_local(async move {
            let url = format!("http://localhost:9000/jasper/{}", job_to_delete);
            let _ = Request::delete(&url)
                .header("Content-Type", "application/json")
                .send().await;
        });
    }
    fn replay_job(jb: &String) {
        let job_to_replay = jb.clone();
        spawn_local(async move {
            let url = format!("http://localhost:9000/jasper/replay/{}", job_to_replay);
            let _ = Request::get(&url)
                .header("Content-Type", "application/json")
                .send().await;
        });
    }
    fn fetch_jobs(ctx: &Context<Self>) {
        let link = ctx.link().clone();

        let cname = ctx.props().customer.customerName.clone();

        spawn_local(async move {
            let req = CustomerJobRequest { customer_name: cname };
            let json_body = serde_json::to_string(&req).unwrap();

            let fetched_jobs: Vec<JS_Scheduled_Job> = Request::post("http://localhost:9000/jasper")
                .header("Content-Type", "application/json")
                .body(json_body)
                .expect("Failed to build request")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .expect("Failed to parse JSON");

            // Send message back to update component state
            link.send_message(Msg::SetJobs(fetched_jobs));
        });
    }
}
