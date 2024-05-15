use crate::model::CreateNotification;
use crate::schema::notifications;
use chrono::{Local, Utc};
use diesel::prelude::*;
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use dotenvy::dotenv;
use std::env;
use tokio_schedule::{every, Job};

async fn get_connection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_result = AsyncPgConnection::establish(&database_url).await;
    match connection_result {
        Ok(conn) => return conn,
        Err(e) => {
            println!("Error connecting to database: {:?}", e);
            panic!("Error connecting to database");
        }
    }
}

fn get_notification() -> CreateNotification {
    let data = CreateNotification {
        title: "New Notification".to_string(),
        body: "This is a new notification".to_string(),
        created_at: Utc::now().naive_utc(),
        published: true,
    };
    return data;
}

pub async fn start_scheduler() {
    let interval = every(15).seconds().in_timezone(&Utc).perform(|| async {
        let mut dbconn = get_connection().await;

        let transaction_result = dbconn
            .transaction::<_, diesel::result::Error, _>(|conn| {
                async move {
                    let total_notifications = notifications::table.count().execute(conn).await?;
                    println!("Total notifications: {:?}", total_notifications);

                    let n_data = get_notification();
                    diesel::insert_into(notifications::table)
                        .values(n_data)
                        .execute(conn)
                        .await?;
                    Ok(())
                }
                .scope_boxed()
            })
            .await;

        match transaction_result {
            Ok(_) => println!("Notification created - {:?}", Local::now()),
            Err(e) => println!("Error creating notification: {:?}", e),
        }

        println!("schedule_task event - {:?}", Local::now())
    });
    interval.await;
}
