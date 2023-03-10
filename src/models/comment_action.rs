use crate::{
    errors::AppError,
    schema::{users, posts, comments}
};
use diesel::prelude::*;
use super::*;

type Result<T> = std::result::Result<T, AppError>;

pub fn create_comment(
    conn: &mut SqliteConnection, 
    user_id: i32,
    post_id: i32,
    body: &str,
) -> Result<Comment> {
    conn.transaction(|conn| {
        diesel::insert_into(comments::table)
            .values((
                comments::user_id.eq(user_id),
                comments::post_id.eq(post_id),
                comments::body.eq(body),
            ))
            .execute(conn)?;

        comments::table
            .order(comments::id.desc())
            .select(comments::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn post_comments(conn: &mut SqliteConnection, post_id: i32) -> Result<Vec<(Comment, User)>> {
    comments::table
        .filter(comments::post_id.eq(post_id))
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)
        .map_err(Into::into)
}

pub fn user_comments(
    conn: &mut SqliteConnection,
    user_id: i32,
) -> Result<Vec<(Comment, PostWithComment)>> {
    comments::table
        .filter(comments::user_id.eq(user_id))
        .inner_join(posts::table)
        .select((
            comments::all_columns, 
            (posts::id, posts::title, posts::published),
        ))
        .load::<(Comment, PostWithComment)>(conn)
        .map_err(Into::into)
}
