use juniper::{EmptyMutation, RootNode};
use chrono::{NaiveDate};

struct DayData {
    id: i32,
    date: NaiveDate,
}

#[juniper::object(description = "A day of mood data")]
impl DayData {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn days() -> Vec<DayData> {
        vec![
            DayData {
                id: 1,
                date: NaiveDate::from_ymd(2020, 4, 13).to_owned(),
            },
            DayData {
                id: 2,
                date: NaiveDate::from_ymd(2020, 4, 14).to_owned(),
            }
        ]
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}