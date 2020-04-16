table! {
    day_data (id) {
        id -> Int4,
        date -> Date,
        mood_id -> Nullable<Int4>,
    }
}

table! {
    moods (id) {
        id -> Int4,
        string -> Text,
    }
}

joinable!(day_data -> moods (mood_id));

allow_tables_to_appear_in_same_query!(
    day_data,
    moods,
);
