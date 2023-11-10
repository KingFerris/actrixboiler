use crate::models::{NewPost, Post};
use crate::establish_connection;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use std::io::*;

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

pub async fn get_posts() -> HttpResponse {
    use crate::schema::posts::dsl::*;
    let mut connection = establish_connection();

    let posts_result = posts.load::<Post>(&mut connection);

    match posts_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn write_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post");

    post
}

pub async fn create_post() -> HttpResponse {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = write_post(connection, title, &body);
    HttpResponse::Ok().json(post)
}

pub async fn update_post(path: web::Path<(i32,)>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let post_id = path.0;
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection);
    match post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_post(path: web::Path<(String,)>) -> HttpResponse {
    use crate::schema::posts::dsl::*;

    let given_title = &path.0;
    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(given_title))).execute(connection);
    match num_deleted {
        Ok(num) => HttpResponse::Ok().json(num),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
