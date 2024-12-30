diesel::table! {
    bills (id) {
        id -> Integer,
        title -> Varchar,
        amount -> Decimal,
        created_at -> Timestamp,
    }
}