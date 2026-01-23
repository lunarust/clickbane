use yew::prelude::*;
use yew::suspense::{use_future};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use common::*;

#[derive(Properties, PartialEq)]
pub struct ShowListProps {
    pub on_click: Callback<JS_Scheduled_Job>
}

#[function_component(JobsListDisplay)]
pub fn jobs_display( ShowListProps { on_click }: &ShowListProps ) -> HtmlResult {
    let selected_job_id = use_state(|| None::<i32>);

    let jobs_handle = use_future(|| async {
        Request::get("http://localhost:9000/jasper/failed_jobs")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Vec<JS_Scheduled_Job>>()
        .await
    })?;

    let jobs_all = use_state(|| match &*jobs_handle {
        Ok(v) => v.clone(),
        Err(_) => vec![],
    });

    if let Err(e) = &*jobs_handle {
        return Ok(html! { <p class="footer">{ "Error loading jobs:" }{e}</p> });
    }
    let show_failed = use_state(|| true);

    let displayed_jobs = jobs_all.iter().filter(|v| {
        if *show_failed {
            v.failed
        } else {
            !v.failed
        }
    });

    let on_toggle = {
        let show_failed = show_failed.clone();
        Callback::from(move |_| show_failed.set(!*show_failed))
    };

    let on_select = |job: &JS_Scheduled_Job| {
        let on_click = on_click.clone();
        let job = job.clone();
        Callback::from(move |_| {
            on_click.emit(job.clone())
        })
    };

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
    Ok(
        html! {
            <div>
            <h1>{ "Scheduled Jobs" }</h1>
            <h3>{ format!("Total {}", jobs_all.len()) }</h3>
            <ul>
                <li>
                <span class="data_title">{"Label"}</span>
                <span class="data_title">{" name"}</span>
                <span class="data_title">{"description"}</span>
                <span class="data_title">{"trigger type"}</span>
                <span class="data_title">{"state"}</span>
                <span class="data_title">{"previous"}</span>
                <span class="data_title">{"next"}</span>
                <span class="data_title">{"last failed"}</span>
                <span class="data_title">{"error"}</span></li>
                { for displayed_jobs.map(|job| {

                    let mut button_enable = true;
                    if job.failed { button_enable = false; }
                    let on_replay_click = {
                        let on_replay = on_replay.clone();
                        //let on_replay_click = on_replay.clone();
                        let jn = job.job_name.clone();
                        Callback::from(move |_| on_replay.emit((*jn).to_string()))};
                    html! {
                        <li key={job.job_name.clone()}
                            onclick={on_select(job)}
                        >
                        <span class="data">{ &job.label }</span>
                        <span class="data">{ &job.job_name }</span>
                        <span class="data">{ &job.description }</span>
                        <span class="data">{ &job.trigger_type }</span>
                        <span class="data">{ &job.trigger_state }</span>
                        <span class="data">{ &job.prev_fire }</span>
                        <span class="data">{ &job.next_fire }</span>
                        <span class="data">{ &job.occurrence_date }</span>
                        <span class="data">{ &job.err_message }</span>
                        <button onclick={ on_replay_click } disabled={button_enable}>{ "⟳" }</button>
                        </li>
                    }
                })}
            </ul>
            <input
                type="checkbox"
                id="toggle_all"
                checked={*show_failed}
                onclick={on_toggle} />
            <label for="toggle_all">{ " Show Failed " }</label>
        </div>
       })
}

#[function_component(Loading)]
pub fn loading() -> Html {

    let selected_job = use_state(|| None);
    let on_job_select = {
        let selected_job = selected_job.clone();
        Callback::from(move |job: JS_Scheduled_Job| {
            selected_job.set(Some(job))
        })
    };

    let fallback = html! { <div>{ "Loading Jobs ... " }</div> };

    html! {
        <div id="content">
        // The list of scheduled jobs loaded from the backend via api call
        <Suspense {fallback}>
            <JobsListDisplay on_click={on_job_select} />
        </Suspense>


        </div>
    }
}
