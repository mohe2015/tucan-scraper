use tucant_types::{LoginResponse, Tucan};
use yew::{function_component, html, use_context, Html, UseStateHandle};

use crate::{LoginComponent, LogoutComponent};

#[function_component(Navbar)]
pub fn navbar<TucanType: Tucan + 'static>() -> Html {
    let current_session =
        use_context::<UseStateHandle<Option<LoginResponse>>>().expect("no ctx found");

    html! {
        <nav class="navbar navbar-expand-xl bg-body-tertiary">
            <div class="container-fluid">
                <a class="navbar-brand" href="#">{ "TUCaN't" }</a>
                <button
                    class="navbar-toggler"
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarSupportedContent"
                    aria-controls="navbarSupportedContent"
                    aria-expanded="false"
                    aria-label="Toggle navigation"
                >
                    <span class="navbar-toggler-icon" />
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-xl-0">
                        if current_session.is_some() {
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Aktuelles" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Aktuelles" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Nachrichten" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "VV" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Lehrveranstaltungssuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Raumsuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Aktuell - Wintersemester 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis Gasthörer_innen WiSe 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des SoSe 2024" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Vorlesungsverzeichnis des WiSe 2023/24" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Archiv" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Stundenplan" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Stundenplan" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Tagesansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Wochenansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Monatsansicht" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Export" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Veranstaltungen" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Veranstaltungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Module" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Veranstaltungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Wahlbereiche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#/registration/,-N000311,-A"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Anmeldung" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Mein aktueller Anmeldestatus" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Prüfungen" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Prüfungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Prüfungen" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Mein Prüfungsplan" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Semesterergebnisse" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Leistungsspiegel" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Service" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Service" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Persönliche Daten" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Dokumente" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Anträge" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Sperren" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "Bewerbung" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Bewerbung" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Bewerbung" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="#"
                                            data-bs-toggle="collapse"
                                            data-bs-target=".navbar-collapse.show"
                                        >
                                            { "Meine Dokumente" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="#"
                                    data-bs-toggle="collapse"
                                    data-bs-target=".navbar-collapse.show"
                                >
                                    { "Hilfe" }
                                </a>
                            </li>
                        } else {
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome"
                                >
                                    { "Startseite" }
                                </a>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "VV" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AN07PBvMn59bWIkwI5kPIrV6ttS-nQO52gY48WnmIDWTv9PQsRceJIEekBMsiG7XrGxJxL6WmWMRCgv6ZdqcqJvgDTJ41d1yHBN12FkxT2-2R1XLasNa7As0AF4mdh2AohuT~wrzHUbQsFAkJJF23tlDnGaVBwg3B7S2UW-GrR0DSb24IOCR8EhR1~A__"
                                        >
                                            { "Vorlesungsverzeichnis" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-ApmupJef8fipOyK7bl09ygBDteYMod2NhGXau-cCDtwF~ggV1owM8UGwi5QASFA9535nG6r-P3aBHfQ37AdKwI9XU3o5lAqbiph8OKOZdmIc2aQDE2rraXrtfetu-R36DT08Ilu6r2Xk5AWdXIpsk2mXXrhm2"
                                        >
                                            { "Lehrveranstaltungssuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=SEARCHROOM&ARGUMENTS=-N000000000000001,-N000385,"
                                        >
                                            { "Raumsuche" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-A1UOJsw28R2k1y54teJLFI0AmVf~SynJG0lPJT4hWx3Pd2pgi2uLxxMBsS2cjb~FIP7H1vkneIgJXzeLZHmQnsV2wfuTkuB4oW5p-MMXJPojkAa33-gGvV1gbDS6A1OPsznmo~6xd~GSRxk3YFVK7crzHm1Yf8HKyXwC78ZIxnS-9~tB4L3Ul6uEkJQ__"
                                        >
                                            { "Aktuell - Wintersemester 2024/25" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-Avg592S~LGYOfjtSndrtNxlVR2J9kIS7xmljoREqXTgcF6XcTMM0rTR5IBrCNWEp5sJpEsOljcv8c5N2aCILh9N6kOUfFFfGbQ~qQEtepIFPZO98~n7G8X0qpH2kWNuRmVW~qMPTY-HcdOhKUVDhM1X4owra8caR3S7MnROHEOzLqdWa5Zm2awq3Qag__"
                                        >
                                            { "Vorlesungsverzeichnis des SoSe2024" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=ACTION&ARGUMENTS=-AzMwKIbEl8fKxOyceOeUbGpsq~PU6GV3dKdLTh7n2lINJBcr7xxS4gwjhqFXEyi0GARw-A1oAVkgvWBa7dbFXV18fXBZ9oj3cco18MY5ZKrU7wq7~6IDot4aIipfyrvFSvGOTXmx3Me~ft-AjiQIqbQBGUtsEdzIeC64v3j3UqYqCSV2wj1JtwsguBw__"
                                        >
                                            { "Vorlesungsverzeichnis des WiSe 2023/24" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000463,-Avvarchivstart%2Ehtml"
                                        >
                                            { "Archiv" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    role="button"
                                    data-bs-toggle="dropdown"
                                    aria-expanded="false"
                                >
                                    { "TUCaN-Account" }
                                </a>
                                <ul class="dropdown-menu">
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000410,-Atucan%5Faccount%2Ehtml"
                                        >
                                            { "TUCaN-Account" }
                                        </a>
                                    </li>
                                    <li>
                                        <hr class="dropdown-divider" />
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=CREATEACCOUNT&ARGUMENTS=-N000000000000001,-N000425,"
                                        >
                                            { "Account anlegen" }
                                        </a>
                                    </li>
                                    <li>
                                        <a
                                            class="dropdown-item"
                                            href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOSTPASS&ARGUMENTS=-N000000000000001,-N000426,-A"
                                        >
                                            { "Passwort vergessen (nur für Bewerber/innen!)" }
                                        </a>
                                    </li>
                                </ul>
                            </li>
                            <li class="nav-item">
                                <a
                                    class="nav-link"
                                    href="https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000340,-Ahilfe%2Ehtml"
                                >
                                    { "Hilfe" }
                                </a>
                            </li>
                        }
                    </ul>
                    if !current_session.is_some() {
                        <LoginComponent<TucanType> />
                    } else {
                        <LogoutComponent />
                    }
                </div>
            </div>
        </nav>
    }
}
