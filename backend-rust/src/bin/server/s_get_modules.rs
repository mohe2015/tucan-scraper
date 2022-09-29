// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::collections::HashMap;
use std::collections::VecDeque;

use crate::MyError;
use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::{
    get,
    web::{Data, Path},
};
use diesel::sql_query;
use diesel::QueryDsl;
use diesel::QueryableByName;

use diesel::sql_types::Bool;
use diesel::sql_types::Bytea;
use diesel::sql_types::Nullable;
use diesel::sql_types::Text;
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tucan_scraper::models::ModuleMenu;
use tucan_scraper::schema::module_menu_module;
use tucan_scraper::tucan::Tucan;
use tucan_scraper::tucan_user::RegistrationEnum;
use tucan_scraper::tucan_user::TucanSession;
use tucan_scraper::url::Registration;

#[derive(QueryableByName, Hash, PartialEq, Eq, Debug, Serialize, Clone)]
pub struct ModuleMenuPathPart {
    #[diesel(sql_type = Nullable<Bytea>)]
    #[serde(skip)]
    pub parent: Option<Vec<u8>>,
    #[diesel(sql_type = Bytea)]
    pub tucan_id: Vec<u8>,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Bool)]
    #[serde(skip)]
    pub leaf: bool,
}

#[derive(Serialize)]
pub struct ModuleMenuResponse {
    entries: RegistrationEnum,
    path: Vec<VecDeque<ModuleMenuPathPart>>,
}

// trailing slash is menu
#[get("/modules/{menu_id:.*}")]
pub async fn get_modules<'a>(
    session: Session,
    tucan: Data<Tucan>,
    path: Path<String>,
) -> Result<impl Responder, MyError> {
    match session.get::<TucanSession>("session").unwrap() {
        Some(session) => {
            let tucan = tucan.continue_session(session).await.unwrap();

            let value = if path.is_empty() {
                ModuleMenuResponse {
                    entries: RegistrationEnum::Submenu(vec![tucan.root_registration().await?]),
                    path: Vec::new(),
                }
            } else {
                let (module_menu, subentries) = tucan
                    .registration(Registration {
                        path: base64::decode(path.as_bytes()).unwrap(),
                    })
                    .await?;

                let mut connection = tucan.tucan.pool.get().await?;

                let path_to_root = sql_query(r#"
                        WITH RECURSIVE search_tree AS (
                            SELECT t.parent, t.tucan_id, t.name, true as leaf
                            FROM module_menu_unfinished t JOIN module_menu_module mmm ON mmm.module_menu_id = t.tucan_id WHERE mmm.module_id = '\x000154f481a77362'
                          UNION
                            SELECT t.parent, t.tucan_id, t.name, false as leaf
                            FROM module_menu_unfinished t JOIN search_tree st
                            ON t.tucan_id = st.parent
                        )
                        SELECT * FROM search_tree;
        "#).load::<ModuleMenuPathPart>(&mut connection).await?;

                let leaves = path_to_root.iter().take_while(|v| v.leaf);

                let nonleaves = path_to_root
                    .iter()
                    .rev()
                    .take_while(|v| !v.leaf)
                    .map(|v| (&v.tucan_id, v))
                    .collect::<HashMap<_, _>>();

                let paths = leaves
                    .map(|l| {
                        let mut current = Some(&l);
                        let mut path = VecDeque::new();
                        while let Some(curr) = current {
                            path.push_front(curr.to_owned().to_owned());
                            if let Some(parent) = &curr.parent {
                                current = nonleaves.get(&parent);
                            } else {
                                break;
                            }
                        }
                        path
                    })
                    .collect::<Vec<_>>();

                println!("{:?}", paths);

                ModuleMenuResponse {
                    entries: subentries,
                    path: paths,
                }
            };

            Ok(HttpResponse::Ok().content_type("text/plain").json(value))
        }
        None => Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("not logged in")),
    }
}
