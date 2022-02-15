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
            <div class=classes!("columns")>
                <div class=classes!("column", "is-two-thirds") >
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
                    </div>
                </div>

                <div class=classes!("column")>
                    <figure class=classes!("image", "is-square")>
                        <img class=classes!("profile-picture") src="/assets/img/me.jpg" alt="Myself and Coheed the Samoyed."/>
                    </figure>
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
                        <h1 class=classes!("has-text-white")>{"LinkedIn"}</h1>
                    </div>                    
                </div>

                <div class=classes!("column", "is-narrow")>
                    <div class=classes!("box")>
                        <h1 class=classes!("has-text-white")>{"Github"}</h1>
                    </div> 
                </div>

                <div class=classes!("column", "is-narrow")>
                    <div class=classes!("box")>
                        <h1 class=classes!("has-text-white")>{"Twitch"}</h1>
                    </div> 
                </div>
            </div>
        }
    }
}