extern crate dotenv;

use diesel::prelude::*;
use juniper::RootNode;
use chrono::NaiveDate;

use crate::db::PgPool;
use crate::schema::day_data;

#[derive(Queryable)]
struct DayData {
    pub id: i32,
    pub date: NaiveDate,
    pub mood_id: Option<i32>,
    //Extension options: Energy, Sick
}

#[derive(juniper::GraphQLInputObject, Insertable)]
#[table_name = "day_data"]
pub struct NewDay {
    pub date: NaiveDate,
    pub mood_id: Option<i32>,
}

#[juniper::object(Context = Context, description = "A day of mood data")]
impl DayData {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn mood_id(&self) -> Option<i32> {
        self.mood_id
    }

    pub fn mood_str(&self, context: &Context) -> Option<String> {
        use crate::schema::moods::dsl::*;
        let connection = context.db.get().unwrap();
        match self.mood_id {
            None => None,
            Some(mood_extracted) => {
                let mood = moods
                    .filter(id.eq(mood_extracted))
                    .limit(1)
                    .load::<Mood>(&connection)
                    .expect("Error loading mood");
                Some(mood[0].string.clone())
            }
        }
    }
}

#[derive(Queryable)]
pub struct Mood {
    pub id: i32,
    pub string: String,
}

#[juniper::object(description = "Mood value enumeration")]
impl Mood {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn string(&self) -> &str {
        self.string.as_str()
    }
}

#[derive(Clone)]
pub struct Context {
    pub db: PgPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    fn days(context: &Context) -> Vec<DayData> {
        use crate::schema::day_data::dsl::*;
        let connection = context.db.get().unwrap();;
        day_data
            .limit(100)
            .load::<DayData>(&connection)
            .expect("Error loading day data")
    }

    fn mood(context: &Context) -> Vec<Mood> {
        use crate::schema::moods::dsl::*;
        let connection = context.db.get().unwrap();;
        moods
            .limit(100)
            .load::<Mood>(&connection)
            .expect("Error loading moods")
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn create_day(context: &Context, data: NewDay) -> DayData {
        let connection = context.db.get().unwrap();;
        diesel::insert_into(day_data::table)
            .values(&data)
            .get_result(&connection)
            .expect("Error saving new day data")
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}