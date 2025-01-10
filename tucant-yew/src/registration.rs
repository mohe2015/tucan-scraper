use std::ops::Deref as _;

use tucant_types::{
    registration::{AnmeldungRequest, AnmeldungResponse, RegistrationState},
    LoginResponse, Tucan,
};
use wasm_bindgen_futures::spawn_local;
use yew::{
    function_component, html, use_context, use_effect_with, use_state, Html, HtmlResult,
    Properties, UseStateHandle,
};
use yew_router::{hooks::use_navigator, prelude::Link};

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AnmeldungRequestProps {
    pub registration: AnmeldungRequest,
}

#[function_component(Registration)]
pub fn registration<TucanType: Tucan + 'static>(
    AnmeldungRequestProps { registration }: &AnmeldungRequestProps,
) -> HtmlResult {
    let data = use_state(|| {
        Ok(AnmeldungResponse {
            path: vec![],
            submenus: vec![],
            entries: vec![],
            additional_information: vec![],
        })
    });
    let loading = use_state(|| false);
    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");
    {
        let data = data.clone();
        let loading = loading.clone();
        let current_session = current_session.clone();
        use_effect_with(registration.clone(), move |anmeldung_request| {
            loading.set(true);
            let anmeldung_request = anmeldung_request.clone();
            let data = data.clone();
            spawn_local(async move {
                match TucanType::anmeldung(
                    current_session.deref().clone().unwrap(),
                    anmeldung_request,
                )
                .await
                {
                    Ok(response) => {
                        data.set(Ok(response));
                        loading.set(false);
                    }
                    Err(error) => {
                        data.set(Err(error.to_string()));
                        loading.set(false);
                    }
                }
            })
        });
    }
    let navigator = use_navigator().unwrap();

    let data = match data.deref() {
        Ok(data) => data,
        Err(error) => {
            return Ok(html! {
                <div class="container">
                    <div class="alert alert-danger d-flex align-items-center mt-2" role="alert">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="bi bi-exclamation-triangle-fill flex-shrink-0 me-2"
                            width="16"
                            height="16"
                            viewBox="0 0 16 16"
                            role="img"
                            aria-label="Error:"
                        >
                            <path
                                d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"
                            />
                        </svg>
                        <div>{ error }</div>
                    </div>
                </div>
            })
        }
    };

    if data.submenus.len() == 1
        && data.additional_information.is_empty()
        && data.entries.is_empty()
        && !*loading
    {
        navigator.replace(&Route::Registration {
            registration: data.submenus[0].1.arguments.clone().to_string(),
        });
        return Ok(html! { <></> });
    }

    #[expect(unused_parens)]
    Ok(html! {
        <div class="container">
            <h2 class="text-center">{ "Registration" }</h2>
            <nav style="min-height: 5.5rem" aria-label="breadcrumb">
                <ol class="breadcrumb">
                    { data.path.iter().map(|entry| {
                            html!{<li class="breadcrumb-item"><Link<Route> to={Route::Registration { registration: entry.1.arguments.clone().to_string()}}>{entry.0.clone()}</Link<Route>></li>}
                        }).collect::<Html>() }
                </ol>
            </nav>
            <h2 class="text-center">{ "Submenus" }</h2>
            <ul class="list-group">
                { data.submenus.iter().map(|entry| {
                        html!{<Link<Route> to={Route::Registration { registration: entry.1.arguments.clone().to_string()}} classes="list-group-item list-group-item-action">{ format!("{}", entry.0) }</Link<Route>>}
                    }).collect::<Html>() }
            </ul>
            <h2 class="text-center">{ "Modules and courses" }</h2>
            <ul class="list-group">
                { for data.entries.iter().map(|entry| {
                        let module = entry.module.as_ref();
                        html!{
                            <li class="list-group-item">
                                <div class="d-flex w-100 justify-content-between">
                                    <h5 class="mb-1"><Link<Route> to={Route::ModuleDetails { module: module.map(|module| module.url.clone().arguments).unwrap_or("/notfound".to_owned())}}>{ format!("Modul {} {}", module.map(|module| module.id.clone()).unwrap_or_default(), module.map(|module| module.name.clone()).unwrap_or_default())}</Link<Route>></h5>
                                    <small class="text-body-secondary">{ format!("Anmeldung bis {}", module.map(|module| module.date.clone()).unwrap_or_default()) }</small>
                                </div>
                                <div class="d-flex w-100 justify-content-between">
                                    <h6 class="mb-1">{ format!("{}", module.map(|module| module.lecturer.clone().unwrap_or_default()).unwrap_or_default()) }</h6>
                                    <small class="text-body-secondary">{ module.map(|module| "Teilnehmerlimit ".to_owned() + &module.limit_and_size).unwrap_or_default() }</small>
                                </div>

                                {
                                    module.map(|module| {
                                        match &module.registration_button_link {
                                            RegistrationState::Unknown => html! { },
                                            RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Vom Modul abmelden"}</a> },
                                            RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}", register_link.clone())}>{"Zum Modul anmelden"}</a> },
                                        }
                                    })
                                }
                                <ul class="list-group">
                                {
                                    for entry.courses.iter().map(|course|
                                     {
                                        html! {
                                            <li class="list-group-item">
                                                <div class="d-flex w-100 justify-content-between">
                                                    <h5 class="mb-1"><Link<Route> to={Route::CourseDetails { course: format!(":N{:015}{}", current_session.as_ref().map(|s| s.id.to_string()).unwrap_or("1".to_owned()), course.1.url.clone()) }}>{ format!("Kurs {} {}", course.1.id, course.1.name) }</Link<Route>></h5>
                                                    <small class="text-body-secondary">{ format!("Anmeldung bis {}", course.1.registration_until) }</small>
                                                </div>

                                                <div class="d-flex w-100 justify-content-between">
                                                    <h6 class="mb-1">{ format!("{}", course.1.lecturers.clone().unwrap_or_default()) }</h6>
                                                    // needing the parentheses is a yew bug
                                                    <small class="text-body-secondary">{ ("Teilnehmerlimit ".to_owned() + &course.1.limit_and_size) }</small>
                                                </div>

                                                <h6 class="mb-1">{ format!("{}", course.1.begin_and_end.clone().unwrap_or_default()) }</h6>

                                                {
                                                    match &course.1.registration_button_link {
                                                        RegistrationState::Unknown => html! { },
                                                        RegistrationState::Registered { unregister_link } => html! { <a class="btn btn-danger mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",unregister_link.clone())}>{"Vom Kurs abmelden"}</a> },
                                                        RegistrationState::NotRegistered { register_link } => html! { <a class="btn btn-outline-success mb-1" role="button" href={format!("https://www.tucan.tu-darmstadt.de{}",register_link.clone())}>{"Zum Kurs anmelden"}</a> },
                                                    }
                                                }
                                            </li>
                                        }
                                    })
                                }
                                </ul>
                            </li>
                        }
                    }) }
            </ul>
            if *loading {
                <div
                    style="z-index: 10000"
                    class="position-fixed top-50 start-50 translate-middle"
                >
                    <div class="spinner-grow" role="status">
                        <span class="visually-hidden">{ "Loading..." }</span>
                    </div>
                </div>
            }
        </div>
    })
}
