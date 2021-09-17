#![recursion_limit = "256"]

use wasm_bindgen::prelude::*;
use yew::html;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod owner;
mod pet;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {}

pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/create-owner"]
    CreateOwner,
    #[to = "/app/create-pet/{id}"]
    CreatePet(i32),
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let html: Html = html! {
            <div class=classes!("app")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::CreateOwner => {
                                    html! {
                                        <div>
                                            <owner::create::CreateForm />
                                        </div>}
                                }
                                AppRoute::CreatePet(owner_id) => {
                                    html! {
                                        <div>
                                            <pet::create::CreateForm owner_id=owner_id/>
                                        </div>}
                                }
                                AppRoute::Detail(owner_id) => {
                                    html! {
                                        <div>
                                            <owner::detail::Detail owner_id=owner_id/>
                                        </div>}
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <owner::list::List />
                                            <br />
                                            <Anchor route=AppRoute::CreateOwner>
                                            { "Create New Owner" }
                                                </Anchor>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        };
        html
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<FullStackApp>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
