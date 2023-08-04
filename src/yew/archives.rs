use crate::{
    comics_error::try_or_report,
    data_recovery::{Archive, ArchiveStatus},
    diesel_helpers::db,
    nas_path, schema, ComicsResult,
};

use {diesel::prelude::*, yew::prelude::*};

#[function_component(Archives)]
pub fn html() -> Html {
    let mut db = db().expect("DB should be accessible");
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
        .get_results(&mut db)
        .expect("Problem with diesel query");

    html! {
        <div >
            {archives.len()}
        </div>
    }
}
