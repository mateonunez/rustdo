table! {
    todos (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Nullable<Text>,
        completed -> Bool,
        created_at -> Timestamp,
    }
}
