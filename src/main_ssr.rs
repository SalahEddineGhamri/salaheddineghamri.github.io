use my_portfolio_app::app::App;
use yew::ServerRenderer;
use std::fs;

fn main() {
    let html = futures::executor::block_on(
        ServerRenderer::<App>::new().render()
    );

    fs::create_dir_all("dist").unwrap();
    fs::write("dist/index.html", html).unwrap();
}
