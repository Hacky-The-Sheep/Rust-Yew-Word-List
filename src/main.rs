use yew::{html::Scope, prelude::*};

enum Move {
    Previous,
    Next,
}

struct WordComponent {
    word: String,
    pointer: usize,
    list: Vec<&'static str>,
}

impl Component for WordComponent {
    type Message = Move;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            word: "FIRST".to_string(),
            pointer: 0,
            list: vec![
                "FIRST",
                "Hello",
                "Goodbye",
                "Whatever",
                "nothing",
                "HUUUUUUUUGE LIST",
                "This",
                "Is",
                "A",
                "Test",
                "LAST WORD",
            ],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Move::Previous => {
                if self.pointer == 0 {
                    true // re-render component
                } else {
                    self.pointer -= 1;
                    self.word = self.list[self.pointer].to_string();
                    true // re-render component
                }
            }
            Move::Next => {
                if self.pointer == self.list.len() - 1 {
                    true // re-render component
                } else {
                    self.pointer += 1;
                    self.word = self.list[self.pointer].to_string();
                    true // re-render component
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link: &Scope<WordComponent> = ctx.link();
        html! {
            <div class="container">
                <p class="Main_P">{ &self.word }</p>
                <button class="custom-btn buttons" onclick={link.callback(|_| Move::Previous)}>{"Previous Word"}</button>
                <button class="custom-btn buttons" onclick={link.callback(|_| Move::Next)}>{"Next Word"}</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<WordComponent>();
}
