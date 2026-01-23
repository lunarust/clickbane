use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

use common::*;
use crate::pages::jobs::JobsListDisplay;


#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CustDetailsProps {
    pub cust: Customer
}

#[function_component(CustDetails)]
pub fn CustomerDetails(CustDetailsProps { cust }: &CustDetailsProps) -> HtmlResult {

   // let fallback = html! { <div>{ "Loading ..." }</div> };
    Ok(

        html! {
        <h1>
        {"Loading for ..."}{ &cust.customerNumber }{"-"}{ &cust.customerName.to_string() }

       // if let jobs = &*jobs_display {
           // <Suspense { fallback }>
           //     <JobsListDisplay cname={ &*cust.customerName } />
           // </Suspense>

       // }

        </h1>
    })
}
