use yew::prelude::*;

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!();
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("tile", "is-ancestor", "is-vertical")>

                <div class=classes!("tile", "is-child", "hero")>
                    <div class=classes!("hero-body", "container", "pb-0")>
                        <h1 class=classes!("title", "is-1")>{"Welcome to Xel's home"}</h1>
                    </div>
                </div>

                <div class=classes!("tile", "is-child")>
                    <h1 class=classes!("hero-body", "pb-5")>{"Here's links to some socials:"}</h1>
                    {self.view_socials()}
                </div>

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
}

impl Home {
    fn view_socials(&self) -> Html {
        html!{
            <div class=classes!("columns", "mx-5", "is-centered")>
                <div class=classes!("column", "is-narrow")>
                    <div class=classes!("box")>
                        <h1 class=classes!("has-text-white")>{"Discord"}</h1>
                    </div>                    
                </div>

                <div class=classes!("column", "is-narrow")>
                    <div class=classes!("box")>
                        <h1 class=classes!("has-text-white")>{"Twitter"}</h1>
                    </div> 
                </div>

                <div class=classes!("column", "is-narrow")>
                    <div class=classes!("box")>
                        <h1 class=classes!("has-text-white")>{"LinkedIn"}</h1>
                    </div> 
                </div>
            </div>
        }
    }
}