use rocket::fairing::AdHoc;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};

use rocket_sync_db_pools::diesel;

use self::diesel::prelude::*;

include!("schema.rs");

#[database("diesel")]
struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
#[table_name = "plans"]
struct Plan {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    from_hr: Option<i32>,
    from_min: Option<i32>,
    to_hr: Option<i32>,
    to_min: Option<i32>,
    started: Option<bool>,
}

#[post("/", data = "<plan>")]
async fn create(db: Db, plan: Json<Plan>) -> Result<Created<Json<Plan>>> {
    let plan_value = plan.clone();
    db.run(move |conn| {
        diesel::insert_into(plans::table)
            .values(&*plan_value)
            .execute(conn)
    })
    .await?;

    Ok(Created::new("/").body(plan))
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<Plan>>> {
    let plans: Vec<Plan> = db.run(move |conn| plans::table.load(conn)).await?;

    Ok(Json(plans))
}

#[get("/<id>")]
async fn read(db: Db, id: i32) -> Option<Json<Plan>> {
    db.run(move |conn| plans::table.filter(plans::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[delete("/<id>")]
async fn delete(db: Db, id: i32) -> Result<Option<()>> {
    let affected = db
        .run(move |conn| {
            diesel::delete(plans::table)
                .filter(plans::id.eq(id))
                .execute(conn)
        })
        .await?;

    Ok((affected == 1).then(|| ()))
}

#[delete("/")]
async fn destroy(db: Db) -> Result<()> {
    db.run(move |conn| diesel::delete(plans::table).execute(conn))
        .await?;

    Ok(())
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run` that runs the migrations in the
    // specified directory, initializing the database.
    embed_migrations!();

    let conn = Db::get_one(&rocket).await.expect("database connection");
    conn.run(|c| embedded_migrations::run(c))
        .await
        .expect("diesel migrations");

    rocket
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
            .mount("/", routes![list, read, create, delete, destroy])
    })
}
