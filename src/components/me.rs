use yew::{function_component, html};

#[function_component(Me)]
pub fn me() -> Html {
    html! {
        <div>
            <h1>{"Shiyui"}</h1>
            <img class="logo"  src="https://avatars.githubusercontent.com/u/66453922?v=4" alt="my icon" width="200" height="200"/>
            <br /><small>{"GitHub Icon"}</small><br />
            <a>{"学生です。"}</a><br />
            <a>{"サーバーサイドが好きです。"}</a><br />
        </div>
    }
}
