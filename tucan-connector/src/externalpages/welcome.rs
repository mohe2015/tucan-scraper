use scraper::Html;

use crate::{
    common::head::{footer, html_head, logged_out_head, page_start, vv_something},
    html_handler::Root,
    MyClient, TucanError,
};

pub async fn welcome(client: &MyClient) -> Result<(), TucanError> {
    let response = client.get("https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome")
    .send()
    .await?
    .error_for_status()?;
    let content = response.text().await?;
    let document = Html::parse_document(&content);
    let html_handler = Root::new(document.tree.root());
    let html_handler = html_handler.document_start();
    let html_handler = html_handler.doctype();
    html_extractor::html! {
        <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="de" lang="de" xmlns:msdt="uuid:C2F41010-65B3-11d1-A29F-00AA00C14882" xmlns:mso="urn:schemas-microsoft-com:office:office">
            <head>_
    };
    let html_handler = html_head(html_handler);
    html_extractor::html! {
            <style type="text/css">
                "oiK6m4ZNKQoGD_x_6V3-YFNSsLMUaXrX5lQwN4Q88fc"
            </style>_
        </head>_
        <body class="external_pages">_
    };
    let html_handler = logged_out_head(html_handler);
    html_extractor::html! {
        <div id="pageContentContainer" class="pageElementTop">_
            <!--"kZd6CmmgS-q3ZJsbi_QXJmy4uIhbl0Pt05ddWHx3vcs"-->_
            <div id="pageLeft" class="pageElementLeft">_
                <!--"bhHbWVACRyHBE-MoOAfeLy6SUZbsJmGyCbT94cYBHHI"-->_
                <div id="pageLeftTop">
                </div>_
            </div>_
            <div id="pageContent" class="pageElementLeft">_
                <div id="featureBanner">
                </div>_
                <a name="mainContent" class="hidden">_
                </a>_
                <!--"up1YWWVw7bFlV69jn_wheiJ5MLDQ9_KdGWCUZ5gGeuw"-->_
                <div id="pageContentTop" class="pageElementTop">_
                </div>_
                <div id="contentSpacer_IE" class="pageElementTop">
                    <!--"WVhEeLYGpyH0bXmFoofJIUMWxdfkLBe5aUmIdmUfqiM"-->_
                    <!--"CKcFISCJjRLw3ii080mSqvobpMA3Z3OFHiqwurhqzcI"-->_
                    <!--"Ur30ahmaXh5XzV5xIHsTj20h-0qX1_GS1SR0QttvqB0"-->_
                    <script type="text/javascript">
                    </script>_
                    <!--"1SdyF9DDr8Z_kEcqcOdFHDujurFGmYcPovwfandPimw"-->_
                    <meta http-equiv="content-type" content="text/html; charset=windows-1252"></meta>
                    <div id="inhalt" style="padding:0px; width:650px; margin:0px; background-color:#ffffff;">_
                        <h1>
                            "Herzlich willkommen bei TUCaN, dem Campus-Management-System der "
                            <br></br>
                            "TU Darmstadt! "
                        </h1>_
                        <!--"rjV7X6SdGjjerKiAcwXSu6am9MFlzsqzZJpMF0QGvyc"-->_
                        <!--"QZYtNUT0elp2c-JwCE6e-d0tQPEo53cyPn2Gq13180w"-->_
                        <br></br>_
                        <!--"Ha9yU5aVvqveCwalKN4D9fNhg1O3MnuK8ck8kat0mAo"-->_
                        <p style="line-height: 140%;">
                            <strong>
                                "Studierende, Lehrende, Stellvertretungen und Mitarbeitende der TU Darmstadt"
                            </strong>
                            <br></br>
                            "\nmelden sich mit ihrer TU-ID an, um das System zu nutzen."
                        </p>_
                        <ul>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/index.de.jsp" target="_blank">
                                    "FAQ für Studierende"
                                </a>
                            </li>_
                            <li>
                                <a href="https://www.intern.tu-darmstadt.de/dez_ii/campusmanagement/cm_tucan/infos_fuer_lehrende/index.de.jsp" target="_blank">
                                    "FAQ für Lehrende"
                                </a>
                            </li>
                        </ul>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <p style="line-height: 140%;">
                            <strong>
                                "Bewerber:innen und Gasthörer:innen"
                            </strong>
                            <br></br>
                            "\nlegen sich zunächst ein "
                            <a href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml">
                                "TUCaN-Account"
                            </a>
                            "\n an,\n um ihre Zugangsdaten zu erhalten und melden sich anschließend mit \ndiesen Zugangsdaten an, bis sie ihre endgültige TU-ID erhalten."
                        </p>_
                        <ul>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/studieren/studieninteressierte/bewerbung_zulassung_tu/online_bewerbung/index.de.jsp" target="_blank">
                                    "FAQ für Bewerber:innen"
                                </a>
                            </li>_
                            <li>
                                <a href="https://www.tu-darmstadt.de/gasthoerer" target="_blank">
                                    "FAQ für Gasthörer:innen"
                                </a>
                            </li>_
                        </ul>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <p style="line-height: 140%;">
                            <strong>
                                "Promovierende zur Registrierung / Einschreibung"
                            </strong>
                            <br></br>
                            "\nbeachten bitte die Informationen auf den "
                            <a href="http://www.tu-darmstadt.de/promotion-registrierung" target="_blank">
                                "Webseiten"
                            </a>
                            ". "
                        </p>_
                        <p style="line-height: 40%;">_
                        </p>_
                        <!--"DdY7X0SUBoVh1HeLdKUt8ZGyIAO6W4ecYeXtgEC_uu8"-->_
                        <!--"MwqVejdKVpytAu5wfbJoIpGUOEeu3mBFVtYsoeJVZck"-->_
                        <div style="padding:10px; width:650px; border:thin solid grey; margin:0px; background-color:#f8f9ed;">_
                            <p style="line-height: 140%;">_
                                <strong>
                                    "Aktuelles: Vorlesungsverzeichnis Wintersemester 2024/25"
                                </strong>_
                                <br></br>
                                "\n\n\nDas Vorlesungsverzeichnis für das Wintersemester 2024/25 ist jetzt in TUCaN verfügbar. Auch die \nAnmeldung zu Modulen und Lehrveranstaltungen ist freigeschaltet. "
                            </p>_
                            <p style="line-height: 140%;">
                                <strong>
                                    "Hinweis für Erstsemester"
                                </strong>
                                <br></br>
                                "\n  In der Regel sollte unmittelbar nach \nIhrer Aktivierung der TU-ID auch Ihr Zugang zu TUCaN freigeschaltet \nsein. Sollte es ausnahmsweise zu Verzögerungen kommen, bitten wir Sie um\n etwas Geduld. Spätestens nach zwei Arbeitstagen können Sie sich in \nTUCaN einloggen. "
                                <br></br>
                                "→ "
                                <a href="https://www.tu-darmstadt.de/einfachstudieren" target="_blank">
                                    "Alle wichtigen Infos für Erstsemester"
                                </a>
                            </p>_
                            <p style="line-height: 140%;">
                                <strong>
                                    "Weiterleitung von Nachrichten"
                                </strong>_
                                <br></br>
                                "\nDa wir sehr wichtige aktuelle Semesterinformationen  über TUCaN versenden, empfehlen wir Studierenden und Lehrenden dringend die Einrichtung der Weiterleitung der TUCaN-Nachrichten an Ihre E-Mail-Adresse - so bleiben Sie immer auf dem Laufenden!"
                                <br></br>
                                "\n→ Zur Anleitung "
                                <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_17153.de.jsp" target="_blank">
                                    "für Studierende"
                                </a>
                                " / "
                                <a href="https://www.intern.tu-darmstadt.de/dez_ii/campusmanagement/cm_tucan/infos_fuer_lehrende/faq_lehrende/details_18368.de.jsp" target="_blank">
                                    "für Lehrende"
                                </a>
                            </p>_
                            <p style="line-height: 140%;">
                                <strong>
                                    "Studienbescheinigung"
                                </strong>
                                <br></br>
                                "\n  Studierende können ihre Studienbescheinigung  selbst im TUCaN Webportal herunterladen. Die Bescheinigungen finden Sie nach dem Login unter "
                                <em>
                                    "Service"
                                </em>
                                " / "
                                <em>
                                    "Meine Dokumente"
                                </em>
                                ".\n  "
                                <br></br>
                                "→ "
                                <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_55040.de.jsp" target="_blank">
                                    "Weitere Informationen"
                                </a>
                            </p>_
                        </div>_
                        <!--"BXzL23o8zv_UsgZKGj3HBD_UH7DC2AqQtnCWdvcspHA"-->_
                        <!--"kxxdx9oC13X6nNfsroMEL83B9YcEzTaGRyJ7fJawlxs"-->_
                        <!--"0ggjGWdS9Efor0TAZW47IDLOzuE8oVxnUw6tFSZe_Is"-->_
                        <p>_
                        </p>
                        "\n→ "
                        <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_37312.de.jsp" target="_blank">
                            "TUCaN Wartungszeit: Dienstag um 6 - 9 Uhr"
                        </a>_
                        <br></br>
                        <br></br>
                        "\n→ "
                        <a href="https://www.tu-darmstadt.de/studieren/studierende_tu/studienorganisation_und_tucan/hilfe_und_faq/artikel_details_de_en_344192.de.jsp" target="_blank">
                            "Hinweise zum Datenschutz"
                        </a>_
                        <!--"Diq-FIUkmF-JjcTgujrkufLubS6eenSQeBajtbBaVPw"-->_
                        <p>_
                        </p>_
                        <!--"IecUhiUBkSqz3ZJqC7gry_m5yl8ydiVd5GKzGwpO-ns"-->_
                        <title>
                        </title>
                    </div>_
                </div>_
            </div>_
        </div>_
    };
    let _html_handler = footer(html_handler, 1, 19);
    Ok(())
}
