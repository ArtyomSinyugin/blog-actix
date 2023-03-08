use crate::schema::{users, posts, comments};
use diesel::prelude::*;
use serde::Serialize;

pub(super) mod comment_action;
pub(super) mod post_action;
pub(super) mod user_action;

#[derive(Debug, Queryable, Identifiable, Serialize, PartialEq)]
pub struct User {
    pub id: i32, 
    pub username: String,
}

pub enum UserKey<'a> {
    Username(&'a str),
    ID(i32),
}

#[derive(Debug, Queryable, Identifiable, Serialize, Associations)]
#[diesel(belongs_to(User))]
pub struct Post {
    pub id: i32, 
    pub user_id: i32,
    pub title: String, 
    pub body: String, 
    pub published: bool,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Post))]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub post_id: i32,
    pub body: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct PostWithComment {
    pub id: i32,
    pub title: String, 
    pub published: bool,
}
