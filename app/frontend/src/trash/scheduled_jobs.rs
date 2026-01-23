use yew::prelude::*;
use gloo_net::http::Request;
use common::*;

use crate::pages::scheduled_jobs_list::Loading;


#[function_component]
pub fn Jobs() -> Html {
    /*
    let selected_job = use_state(|| None);

    let on_job_select = {
        let jb = selected_job.clone();
        Callback::from(move |job: JS_Scheduled_Job| {
            jb.set(Some(job));
        })
    };
    */
    html!{
        <div id="main">

            <Loading
            /*
                on_click={
                    let cb = on_job_select.clone();
                    cb.reform(|job| (job))
                }
                */
                />
        </div>
    }
}
