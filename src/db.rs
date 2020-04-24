use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use std::env;
    use diesel::prelude::*;
    use diesel::result::Error;
    use chrono::NaiveDate;

    fn test_establish_connection() -> PgConnection {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }
    
    #[test]
    fn test_diesel_day_data() {
        use crate::schema::day_data::dsl::*;
        use crate::graphql_schema::{NewDay, DayData};

        let conn = test_establish_connection();
        let new_day = NewDay {
            date: NaiveDate::from_ymd(1990, 02, 05),
            mood_id: Some(1),
        };

        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(day_data)
                .values(&new_day)
                .execute(&conn)?;
            
            let days = day_data
                .filter(date.eq(NaiveDate::from_ymd(1990, 02, 05)))
                .limit(1)
                .load::<DayData>(&conn)
                .expect("Error loading day data");
            assert_eq!(new_day.date, days[0].date);
            assert_eq!(new_day.mood_id, days[0].mood_id);
            Ok(())
        });
    }
}