use yew::prelude::*;
use yew_router::{route::Route, switch::Permissive};

mod components;

mod switch;
use switch::{AppAnchor, AppRoute, AppRouter, PublicUrlSwitch};

mod pages;
use pages::{home::Home, posts::Posts, projects::Projects};

enum Msg {
    ToggleNavbar,
}

struct Model {
    // 'ComponentLink' is like a ref to a component
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            navbar_active: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                // The value has changed so we need to rerender it for it to appear on the page
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        //Should only return "true" if new properties are diff to previous received props
        //This component doesnt have any properties so always "false"
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <style>
                {".is-main {
                    color: #40a1c4 !important;
                }
                
                .is-main-dark {
                    color: #1e5062 !important;
                }
                
                .is-background-main {
                    background-color: #40a1c4 !important;
                }
                
                .is-background-main-dark {
                    background-color: #1e5062 !important;
                }
                
                .is-background-main-light {
                    background-color: #92c9dd !important;
                }
                
                .navbar {
                    border-radius: 12px;
                }

                .navbar-menu {
                    border-radius: 12px;
                }

                .navbar-item:hover{
                    background-color: #92c9dd !important;
                }

                .navbar-item:focus{
                    background-color: #1e5062 !important;
                }
                
                .box {
                    background-color: #40a1c4 !important;
                }

                .background {
                    background-image: url(assets/img/ff14-e8s-bg.png);
                    background-size: cover;
                    background-position: center;
                    height: 100%;
                    width: 100%;
                    position: fixed;
                    z-index: 0;
                    top: 0;
                    filter: blur(3px);
                }

                .center-box {
                    height: 85vh;
                    width: 80%;
                    border-radius: 12px;
                    box-shadow: rgba(0, 0, 0, 0.5) 0px 10px 30px;
                    z-index: 2;
                    background-color: #e1eff4;
                    position: relative;
                    margin: auto;
                    margin-top: 5%;
                }

                .top-border {
                    border-top: 1px solid #8b8b8b !important;
                    padding-top: 10px;
                }

                .anchor-bottom {
                    position: absolute;
                    width: 100%;
                    bottom: 0px;
                    margin: auto 0;
                }
                "}
                </style>


                
                <main>
                <div class="background"/>
                <div class=classes!("center-box")>
                    { self.view_nav() }
                    <AppRouter
                        render=AppRouter::render(Self::switch)
                        redirect=AppRouter::redirect(|route: Route| {AppRoute::PageNotFound(Permissive(Some(route.route))).into_public() })
                    />
                    { self.view_footer() }
                </div>
                </main>
            </>
        }
    }
}


impl Model {
    fn view_nav(&self) -> Html {
        let Self {
            ref link,
            navbar_active,
        } = *self;

        let active_class = if navbar_active {"is-active"} else {""};

        html!{
            <nav class="navbar is-background-main" role="navigation" aria-label="main navigation">
                //Div specifically for mobile navigation to use the navbar-menu on mobile
                <div class="navbar-brand">
                    <h1 class="is-size-3 my-2 mx-3 ">{ "Xel's Blog" }</h1>

                    <a role="button"
                        class=classes!("navbar-burger", "burger", "mt-1", active_class)
                        aria-label="menu" aria-expanded="false"
                        onclick=link.callback(|_| Msg::ToggleNavbar)
                    >
                        <span class="" aria-hidden="true"></span>
                        <span class="" aria-hidden="true"></span>
                        <span class="" aria-hidden="true"></span>
                    </a>
                </div>
                <div class=classes!("navbar-menu", active_class, "is-background-main")>
                    <div class="navbar-start">
                        <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            <a class=classes!("")>{"Home"}</a>
                        </AppAnchor>
                        <AppAnchor classes="navbar-item" route=AppRoute::PostList>
                            <a class=classes!("")>{"Posts"}</a>
                        </AppAnchor>
                        <AppAnchor classes="navbar-item" route=AppRoute::ProjectList>
                            <a class=classes!("")>{"Projects"}</a>
                        </AppAnchor>
                    </div>
                </div>
            </nav>
        }
    }

    fn view_footer(&self) -> Html {
        html!{
            <div class=classes!("tile", "is-child", "hero", "anchor-bottom")>
            <div class=classes!("tile", "is-parent", "container")>
                <p class=classes!("block", "has-text-centered", "mx-3", "top-border", "bottom-aligned")>
                    {"A website built to learn Rust and WASM. Powered by "}
                    <strong class="is-main-dark">{"Bulma"}</strong>
                    {" and "}
                    <strong class="is-main-dark">{"Yew"}</strong>
                    {"."}
                </p>
            </div>
            </div>
        }
    }

    fn switch(switch: PublicUrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Post(id) => todo!(),
            AppRoute::PostListPage(page) => todo!(),
            AppRoute::PostList => { html! {<Posts />} },
            AppRoute::Project(id) => todo!(),
            AppRoute::ProjectListPage(page) => todo!(),
            AppRoute::ProjectList => { html! {<Projects />} },
            AppRoute::Home => { html! {<Home />} },
            AppRoute::PageNotFound(Permissive(route)) => todo!(),
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}