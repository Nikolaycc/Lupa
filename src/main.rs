#![allow(non_snake_case)]
// Modules
mod lupa;

use lupa::*;
use poppler::*;
use std::env::args;
use web_view::*;

const WINDOW_TITLE: &str = "Lupa";
const SPACER_SIZE: f64 = 8.0;

fn main() -> Result<(), Errno> {
    let mut lupa = Lupa::new(args().nth(1).unwrap_or("~/ah.txt".to_string()), None);
    let mut lupaui = LupaUi::new();
    lupa.build();

    let html_content = lupaui.get_html(lupa).unwrap();

    WebViewBuilder::new()
        .title(WINDOW_TITLE)
        .content(Content::Html(html_content.as_str()))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap()
        .run()
        .unwrap();


    Ok(())
}
