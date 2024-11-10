use html_extractor::html;
use scraper::Html;

use crate::{
    common::head::{html_head, logged_in_head},
    html_handler::Root,
    login::LoginResponse,
    MyClient, TucanError,
};

pub async fn veranstaltungen(
    client: &MyClient,
    login_response: LoginResponse,
) -> Result<(), TucanError> {
    let response = client.get(format!("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N{:015},-N000273,-Astudveranst%2Ehtml", login_response.id))
                .header("Cookie", format!("cnsc={}", login_response.cookie_cnsc))
                .send()
                .await?
                .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html!(
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de">
        <head>_
    );
    let html_handler = html_head(html_handler);
    html_extractor::html!(
        <style type="text/css">
            "Z8Nk5s0HqiFiRYeqc3zP-bPxIN31ePraM-bbLg_KfNQ"
        </style>_
        </head>_
        <body class="external_pages">_
    );
    let html_handler = logged_in_head(html_handler, login_response.id);
    html_extractor::html!(
    <!--"Ur30ahmaXh5XzV5xIHsTj20h-0qX1_GS1SR0QttvqB0"-->_
        <script type="text/javascript"></script>_
        <!-- "n-Z4H5FHWoHAA64FTQESwvBoOya6RXPFuEU50U1jCJQ"-->_
        <div id="inhalt"><h1>"Das Menü Veranstaltungen"</h1>_
        <div style="padding:0px; width:650px; margin:0px; background-color:#ffffff;" >_
        <p>"In diesem Bereich können Sie sich zu Modulen und Lehrveranstaltungen anmelden."</p>_
        <p><strong>"Benötigen Sie Hilfe im Umgang mit TUCaN? "<a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/index.de.jsp" target="_blank">"Hier"</a>" finden Sie zahlreiche Anleitungen und FAQs."</strong></p>_
        <p>_</p>_
        <h3>"Meine Module / Meine Veranstaltungen"</h3>_
        <p style="line-height: 140%;">"Im Untermenü "<em>"Meine Module"</em> " sehen Sie Module, die Sie im Laufe Ihres Studiums belegt haben. Im Untermenü " <em>"Meine Veranstaltungen"</em> " sehen Sie, in welchem Semester Sie welche Veranstaltungen belegt haben."</p>_
        <br></br>_
        <h3>"Meine Wahlbereiche"</h3>_
        <p style="line-height: 140%;">"Sofern Ihre Prüfungsordnung Nebenfächer, Schwerpunkte oder Wahlbereiche vorsieht, legen Sie diese im Untermenü " <em>"Meine Wahlbereiche"</em> " fest. Bitte halten Sie die Regeln Ihrer  Prüfungsordnung ein. " </p>_
        <br></br>_
        <h3>"Anmeldung"</h3>_
        <p style="line-height: 140%;">"Im Untermenü " <em>"Anmeldung"</em> " melden Sie sich zu Modulen und Lehrveranstaltungen an."</p>_<br></br>_
        <h3>"Mein aktueller Anmeldestatus"</h3>_
        <p style="line-height: 140%;">"Im Untermenü " <em>"Mein aktueller Anmeldestatus"</em> " sehen Sie, zu welchen Veranstaltungen Sie sich im aktuellen Semester angemeldet haben. Hier erfahren Sie, welche Anmeldungen akzeptiert, welche abgelehnt wurden.\n"
        </p>
        <p>_</p>_
        </div>_
        <div style="padding:10px; width:650px; border:thin solid #f8f9ed; margin:0px; background-color:#f8f9ed;" >_
        <h1>"Wichtige Hinweise zur Anmeldung"</h1>_
        <p style="line-height: 140%;">"Die Anmeldephase für die meisten Lehrveranstaltungen beginnt mit der Publikation des Vorlesungsverzeichnisses im\n"
        </p>
        <ul>_
        <li style="line-height: 140%;">"Sommersemester: 1. Werktag im März"</li>_
        <li style="line-height: 140%;">"Wintersemester: 1. Werktag im September"</li>_
        </ul>
         "\n Die Fristen können individuell abweichen – bitte beachten Sie die Terminierungen Ihres Fachbereichs! Die jeweilige Anmeldephase ist in TUCaN bei jeder Lehrveranstaltung individuell aufgeführt."
        <p></p>_
        <p style="line-height: 140%;">"Bei Veranstaltungen mit Teilnahmebeschränkung wird in manchen Studiengängen die Anmeldung für den Zeitraum der Gruppeneinteilung ausgesetzt. " </p>_
        <p style="line-height: 140%;"><strong>"Bitte beachten Sie:"</strong>_</p>
        <ul>_
        <li style="line-height: 140%;">"Wenn Sie sich  zu einer Lehrveranstaltung anmelden, melden Sie sich damit nicht automatisch  zur Prüfung an. \nZu Prüfungen melden Sie sich im Menü " <em>"Prüfungen"</em> " an. " </li>_
        </ul>_
        <ul>_
        <li style="line-height: 140%;">"Nur wenn Sie  zum Modul bzw. der Veranstaltung angemeldet sind, können Sie sich zur  dazugehörigen Prüfung anmelden. " </li>_
        </ul>_
        <ul>_
          <li style="line-height: 140%;">"Einige Prüfungsordnungen sehen  vor, dass bestimmte Module und Veranstaltungen nur im Nebenfach/ im Schwerpunkt/ in Ihrer Vertiefung  belegt werden können. Für Sie bedeutet  das: Sie müssen Ihr Nebenfach/ Ihren Schwerpunkt/ Ihre Vertiefung festlegen,  bevor Sie sich zu diesen Modulen und Lehrveranstaltungen anmelden können. Bitte beachten Sie die Fristen, in denen Sie sich Ihr Nebenfach/ Ihren Schwerpunkt/ Ihren Wahlbereich aussuchen  können. Falls für Ihren Studiengang Fristen festgelegt sind, stehen diese in  Klammern hinter den Titeln der Wahlbereiche.\n  "
          </li>_
        </ul><p></p>_
        <p style="line-height: 140%;">"FAQ zu Problemen bei der Anmeldung finden Sie " <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/index.de.jsp" target="_blank">"hier"</a>". Bei  weiteren Fragen hilft Ihnen " <a href="https://www.tu-darmstadt.de/studienbueros" target="_blank">"Ihr Studienbüro"</a>"."</p>
        );
    let _html_handler = html_handler;
    Ok(())
}
