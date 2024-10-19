use html_extractor::html;
use scraper::Html;

use crate::{
    common::head::html_head_2, html_handler::Root, login::LoginResponse, MyClient, TucanError,
};

pub async fn redirect_after_login(
    client: &MyClient,
    login_response: LoginResponse,
) -> Result<(), TucanError> {
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N{},-N000019,-N000000000000000", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    println!("{response:#?}");
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    println!("{}", document.root_element().html());
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html!(
        <html>
        <head>_
    );
    let html_handler = html_head_2(html_handler);
    html!(
        </head>_
        <body class="redirect">_
        <div id="wrapper">_
                    <a href="http://http://www.tu-darmstadt.de" title="extern http://www.tu-darmstadt.de">_
                            <img border="0" id="logo" src="/gfx/tuda/logo.png" alt="Logo Technische Universität Darmstadt"></img>_
                    </a>_
                    <!-- "MA-hDUoCrkYqlM3RsS9EUjq0y_UcuN1AB82k4O5O8YU" -->_
                    <h2><a href=href_link_1>"Sie werden zur Startseite weitergeleitet ..."</a></h2>_
                    <a style="text-decoration: underline;" href=href_link_2>"Startseite"</a>_
            </div>_
            <div id="sessionId" style="display: none;">session_id</div>_
            <!-- "zhJ3t6XNo2cfpZZEFiqxHJQ9udSXk5D418ej5lEytG8" -->_
            <script>
            script_contents
            </script>
    );
    println!("href_link_1 {href_link_1} href_link_2 {href_link_2} session_id {session_id} script_contents {script_contents}");
    Ok(())
}