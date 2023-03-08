use crate::{
    errors::AppError,
    schema::users
};
use diesel::prelude::*;
use super::*;

type Result<T> = std::result::Result<T, AppError>;

pub fn create_user(conn: &mut SqliteConnection, username: &str) -> Result<User> {
    // стр. 116
    conn.transaction(|conn| {
        diesel::insert_into(users::table)
            .values((users::username.eq(username),))
            .execute(conn)?;
 
        users::table   // я думаю, что смысл этого блока вернуть нам только что записанного пользователя из базы данных в нашу оперативную память (для дальнейшей работы с ним)
            .order(users::id.desc())  // сортировка по убыванию. Зачем это?
            .select((users::id, users::username))
            .first(conn)           // сделал соединение мутабельным, потому что этого требовал этот метод!!!
            .map_err(Into::into)
    })
}

pub fn find_user<'a>(conn: &mut SqliteConnection, key: UserKey<'a>) -> Result<User> {
    match key {
        UserKey::Username(name) => users::table
            .filter(users::username.eq(name))
            .select((users::id, users::username))
            .first::<User>(conn) // эквивалент load, только загружает первый элемент в списке, а не все отфильтрованные
            .map_err(AppError::from),  // почему здесь не Into::into? интересно... 
        UserKey::ID(id) => users::table
            .filter(users::id.eq(id))
            .select((users::id, users::username))
            .first::<User>(conn)
            .map_err(Into::into),      
    }
}
