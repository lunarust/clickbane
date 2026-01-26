use yew::prelude::*;
use yew::suspense::{use_future};
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use common::configuration::ConfigurationBusiness;

#[function_component(ConfDisplay)]
pub fn conf_display() -> HtmlResult {
    let display_state = use_state(|| ConfigurationBusiness::default());
    let id = use_state(|| 0);
    let form_state = use_state(|| ("".to_string(),"".to_string(),"".to_string(),"".to_string(),"".to_string()));
    let handle = use_future(|| async{
        Request::get("http://localhost:9000/configuration/")
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<ConfigurationBusiness>()
        .await
    })?;

    let _display = use_state(|| match &*handle{
        Ok(v) => {
            display_state.set(v.clone());
            form_state.set((
                v.business_name.clone(), v.business_db_host.clone(),
                v.business_db_port.clone().to_string(), v.business_db_user.clone(),
                v.business_db_password.clone()
            ));
        },
        Err(_e) => {
            display_state.set(ConfigurationBusiness::default());
        }
    });

    let valueid = id.clone();
    let trash = {
        Callback::from(move |_| {
            let id = valueid.clone();
            spawn_local(async move {
                let url = format!("http://localhost:9000/configuration/{}", id.to_string());

                let _ = Request::delete(&url.to_string())
                    .header("Content-Type", "application/json")
                    .send()
                    .await;
            });
        })
    };


    let display_state = display_state.clone();
    let form_state = form_state.clone();
    let value = form_state.clone();
    let id = id.clone();

    let save = {
        Callback::from(move |_| {
            let form_state = value.clone();
            let id = id.clone();
            spawn_local(async move {
                let mut json_data = "".to_string();
                let mut url = "";
                if *id != 0 {
                    let json_build: ConfigurationBusiness = ConfigurationBusiness{
                        business_id: *id,
                        business_name: form_state.0.clone(),
                        business_db_host: form_state.1.clone(),
                        business_db_port: form_state.2.clone().parse::<i32>().expect("expect int"),
                        business_db_user: form_state.3.clone(),
                        business_db_password: form_state.4.clone(),
                    };
                    url = "http://localhost:9000/configuration/update";
                    json_data = serde_json::to_string(&json_build)
                        .expect("Some Drama");
                } else {
                    let json_build: ConfigurationBusiness = ConfigurationBusiness{
                        business_id: 0,
                        business_name: form_state.0.clone(),
                        business_db_host: form_state.1.clone(),
                        business_db_port: form_state.2.clone().parse::<i32>().expect("expect int"),
                        business_db_user: form_state.3.clone(),
                        business_db_password: form_state.4.clone(),
                    };
                    url = "http://localhost:9000/configuration/";
                    json_data = serde_json::to_string(&json_build)
                        .expect("Some Drama");
                }

                let _ = Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(json_data.to_string()).expect("More drama")
                    .send()
                    .await;
            });
        })
    };


    Ok(
        html! {
            <div>
                <h1 class="title is-4">{ "Business DB configuration" }</h1>

                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Id:" }</label></div>
                    <div class="field-body"><div class="field">
                        <p class="field">
                        { display_state.business_id.clone() }
                        </p>
                    </div></div>
                </div>
                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Name:" }</label></div>
                    <div class="field-body"><div class="field"><p class="field">
                        <input placeholder="Name"
                         class="input"
                            value={ form_state.0.clone() }
                            oninput={Callback::from({
                            let form_state = form_state.clone();
                            move |e: InputEvent| {
                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                form_state.set(
                                    (input.value(), form_state.1.clone(), form_state.2.clone(),
                                        form_state.3.clone(), form_state.4.clone())
                                );
                            }
                            })}
                        />
                        </p></div></div>
                </div>

                <div class="field is-horizontal">
                      <div class="field-label is-normal"><label class="label">{ "DB Host:" }</label></div>
                      <div class="field-body"><div class="field"><p class="field">
                          <input placeholder="DB Host"
                          class="input"
                          value={ form_state.1.clone() }
                              oninput={Callback::from({
                              let form_state = form_state.clone();
                              move |e: InputEvent| {
                                  let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                  form_state.set(
                                      (form_state.0.clone(), input.value(), form_state.2.clone(), form_state.3.clone(),
                                          form_state.4.clone())
                                  );
                              }})}
                          />
                          </p></div></div>
                  </div>

                <div class="field is-horizontal">
                    <div class="field-label is-normal"><label class="label">{ "Port:" }</label></div>
                    <div class="field-body"><div class="field"><p class="field">
                    <input placeholder="Port"
                     class="input"
                    value={ form_state.2.clone() }
                        oninput={Callback::from({
                        let form_state = form_state.clone();
                        move |e: InputEvent| {
                            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                            form_state.set(
                                (form_state.0.clone(), form_state.1.clone(), input.value(), form_state.3.clone(),
                                    form_state.4.clone())
                            );
                        }})}

                    />
                    </p></div></div>
                </div>


            <div class="field is-horizontal">
                <div class="field-label is-normal"><label class="label">{ "Username:" }</label></div>
                <div class="field-body"><div class="field"><p class="field">

                <input placeholder="Username"
                 class="input"
                    value={ form_state.3.clone() }
                    oninput={Callback::from({
                    let form_state = form_state.clone();
                    move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        form_state.set(
                            (form_state.0.clone(), form_state.1.clone(), form_state.2.clone(),
                                input.value(), form_state.4.clone())
                        );
                    }})}
                />
                </p></div></div>
            </div>

            <div class="field is-horizontal">
                <div class="field-label is-normal"><label class="label">{ "Password:" }</label></div>
                <div class="field-body"><div class="field"><p class="field">
                <input placeholder="Password"
                 class="input"
                 type="password"
                value={ form_state.4.clone() }
                    oninput={Callback::from({
                    let form_state = form_state.clone();
                    move |e: InputEvent| {
                        let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
                        form_state.set(
                            (form_state.0.clone(), form_state.1.clone(), form_state.2.clone(),
                                form_state.3.clone(), input.value())
                        );
                    }})}
                />
                </p></div></div>
             </div>

          <button class="button is-success" onclick={save}>
            <span class="icon is-small">
              <i class="fas fa-check"></i>
            </span>
            <span>{ "Save" }</span>
          </button>

          <button class="button is-danger is-outlined" onclick={trash}>
            <span>{"Delete"}</span>
            <span class="icon is-small">
              <i class="fas fa-times"></i>
            </span>
          </button>

        </div>
        }
    )
}
#[function_component(Conf)]
pub fn loading() -> Html {
    let fallback = html! { <div>{ "Loading configuration..." }</div> };
    html!{
        <div id="content">
            <Suspense {fallback}>
            <ConfDisplay />
            </Suspense>
        </div>
    }
}
