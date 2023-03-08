use crate::{
    errors::AppError,
    schema::{users, posts, comments}
};
use diesel::prelude::*;
use super::*;

type Result<T> = std::result::Result<T, AppError>;

pub fn create_post(conn: &mut SqliteConnection, user: &User, title: &str, body: &str) -> Result<Post> {
    conn.transaction(|conn|{
        diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user.id),
                posts::title.eq(title),
                posts::body.eq(body),
            ))
            .execute(conn)?;

        posts::table
            .order(posts::id.desc())
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn publish_post(conn: &mut SqliteConnection, post_id: i32)-> Result<Post> {
    conn.transaction (|conn| {
        diesel::update(posts::table.filter(posts::id.eq(post_id)))
            .set(posts::published.eq(true)) 
            //.set((posts::published.eq(true), posts::title.eq("Mark".to_string()))) кортеж для апдейта нескольких полей
            .execute(conn)?;

        posts::table
            .find(post_id)
            .select(posts::all_columns)
            .first(conn)
            .map_err(Into::into)
    })
}

pub fn all_posts(conn: &mut SqliteConnection) -> Result<Vec<((Post, User), Vec<(Comment, User)>)>> {   // было Result<Vec<(Post, User)>>
    let query = posts::table
        .order(posts::id.desc())
        .filter(posts::published.eq(true))
        .inner_join(users::table)
        .select((posts::all_columns, (users::id, users::username)))
         .load::<(Post, User)>(conn)?;
    //let posts_with_user = query.load::<(Post, User)>(conn)?;
    let (posts, post_users): (Vec<_>, Vec<_>) = query.into_iter().unzip();

    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)?
        .grouped_by(&posts);

    Ok(posts.into_iter().zip(post_users).zip(comments).collect())
}

pub fn user_posts(conn: &mut SqliteConnection, user_id: i32)-> Result<Vec<(Post, Vec<(Comment, User)>)>> {
    let posts = posts::table   
        .filter(posts::user_id.eq(user_id))
        .order(posts::id.desc())
        .select(posts::all_columns)
        .load::<Post>(conn)?;
        //.map_err(Into::into)

    let comments = Comment::belonging_to(&posts)
        .inner_join(users::table)
        .select((comments::all_columns, (users::id, users::username)))
        .load::<(Comment, User)>(conn)?
        .grouped_by(&posts);

    Ok(posts.into_iter().zip(comments).collect())
}