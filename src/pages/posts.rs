use yew::prelude::*;

pub struct Posts;
impl Component for Posts {
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
                        <h1 class=classes!("title", "is-1")>{"Posts:"}</h1>
                    </div>
                </div>


            </div>
        }
    }
}

impl Posts {
}