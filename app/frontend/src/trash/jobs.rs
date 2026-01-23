use yew::prelude::*;
use yew::suspense::{use_future};
use log::info;
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use common::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub customer: Customer,
    //pub jobs: Vec<JS_Scheduled_Job>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct JobDetailsProps {
    jobs: Vec<JS_Scheduled_Job>,
//    on_click: Callback<JS_Scheduled_Job>,
}

#[component]
fn Jobs_display( JobDetailsProps { jobs }: &JobDetailsProps ) -> Html {
    let on_replay = {
        Callback::from(move |jn: String| {
            spawn_local(async move {
                let url = format!("http://localhost:9000/jasper/replay/{}", jn);
                let response = Request::get(&url)
                    .header("Content-Type", "application/json")
                    .send().await;
            })
        })
    };

    html!{
            <div>
            <h1>{ "Scheduled Jobs" }</h1>
            <ul>
                <li>
                <span class="data_title_large">{"Label"}</span>
                <span class="data_title_small">{" name"}</span>
                //<span class="data_title">{"description"}</span>
                //<span class="data_title">{"trigger type"}</span>
                <span class="data_title_small">{"state"}</span>
                <span class="data_title">{"previous"}</span>
                <span class="data_title">{"next"}</span>
                <span class="data_title_large">{"last failed"}</span>
                //<span class="data_title_small">{"error"}</span>
                </li>
                 if let jobs = &*jobs {
                    for job in jobs {


                    <li key={job.job_name.clone()} >
                            <span class="data_large">{ &job.label }</span>
                            <span class="data_small">{ &job.job_name }</span>
                            //<span class="data">{ &job.description }</span>
                            //<span class="data">{ &job.trigger_type }</span>
                            <span class="data_small">{ &job.trigger_state }</span>
                            <span class="data">{ &job.prev_fire }</span>
                            <span class="data">{ &job.next_fire }</span>
                            //<span class="data">{ &job.occurrence_date }</span>
                            <span class="data_large">{ &job.err_message }</span>
                            <button
                                onclick={self.link.callback(move |_| Msg::ReplayJob(job.job_name))}
/*
                                {
                                    let on_replay = on_replay.clone();
                                    let jn = &job.job_name;
                                    Callback::from(move |_| on_replay.emit((*jn).to_string()))
                                }
                                */
                                // onclick=self.link.callback(move |_| Msg::MakeDeleteAllMerchantJobReq(merchant_id))
                                disabled={if job.failed {false} else {true}}
                                >{ "⟳" }</button>
                            </li>
                }
                }
            </ul>
            <h3>{ format!("Total {}", jobs.len()) }</h3>

        </div>
     }
}

pub struct Jobs {
    jobs: Vec<JS_Scheduled_Job>,
}

pub enum Msg {
    SetJobs(Vec<JS_Scheduled_Job>),
    ReplayJob(String),
}

impl Component for Jobs {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // Initial Fetch
        Self::fetch_jobs(ctx);
        Self { jobs: vec![] }

    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetJobs(jobs) => {
                self.jobs = jobs;
                true
            }
            Msg::ReplayJob(job) => {
                true
            }
        }
    }
    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().customer != _old_props.customer {
            let customer = ctx.props().customer.clone();
            // Trigger your fetch logic here
            Self::fetch_jobs(ctx);
            //info!(">>>>changed {:?}", cname);

            return true; // Re-render if needed
        }
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let fallback = html! { <div>{ "Loading Jobs ... " }</div> };

        let mut form_display = false;
        if Some(ctx.props().customer.clone()) != None { form_display = true; }

        html! {
            <div>
                <Jobs_display jobs={self.jobs.clone()} />

                <br />
                <br />

            </div>
        }
    }
}

impl Jobs {
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
