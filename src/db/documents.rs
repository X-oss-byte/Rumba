use crate::db::model::{DocumentInsert, DocumentMetadata, DocumentQuery};
use crate::db::schema;

use crate::diesel::NullableExpressionMethods;
use diesel::expression_methods::ExpressionMethods;

use crate::db::error::DbError;
use diesel::r2d2::ConnectionManager;
use diesel::RunQueryDsl;
use diesel::{insert_into, PgConnection};
use diesel::{QueryDsl, QueryResult};
use r2d2::PooledConnection;

use crate::settings::SETTINGS;

pub fn create_or_update_document(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    document: DocumentMetadata,
    uri: String,
) -> QueryResult<i64> {
    let absolute_uri = format!("{}{}", SETTINGS.application.document_base_url, uri);
    let title = document.title.clone();
    let metadata = serde_json::to_value(&document).ok();
    let paths = document.paths;
    let insert = DocumentInsert {
        title,
        absolute_uri,
        uri,
        metadata,
        paths,
        updated_at: chrono::offset::Utc::now().naive_utc(),
    };

    insert_into(schema::documents::table)
        .values(&insert)
        .on_conflict(schema::documents::uri)
        .do_update()
        .set(&insert)
        .returning(schema::documents::id)
        .get_result(conn)
}

pub fn get_document_by_path(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    path: String,
) -> Result<DocumentQuery, DbError> {
    let doc = schema::documents::table
        .filter(schema::documents::paths.nullable().eq(vec![path]))
        .select((
            schema::documents::id,
            schema::documents::created_at,
            schema::documents::updated_at,
            schema::documents::absolute_uri,
            schema::documents::uri,
            schema::documents::metadata,
            schema::documents::title,
            schema::documents::paths,
        ))
        .first::<DocumentQuery>(conn)?;
    Ok(doc)
}

pub fn get_document_by_url(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    url: &str,
) -> Result<DocumentQuery, DbError> {
    let doc = schema::documents::table
        .filter(schema::documents::uri.eq(url))
        .select((
            schema::documents::id,
            schema::documents::created_at,
            schema::documents::updated_at,
            schema::documents::absolute_uri,
            schema::documents::uri,
            schema::documents::metadata,
            schema::documents::title,
            schema::documents::paths,
        ))
        .first::<DocumentQuery>(conn)?;
    Ok(doc)
}
