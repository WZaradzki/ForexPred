// @generated automatically by Diesel CLI.

diesel::table! {
    currencies (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        iso -> Varchar,
    }
}

diesel::table! {
    currency_rates (id) {
        #[max_length = 255]
        id -> Varchar,
        #[max_length = 255]
        from_currency_id -> Varchar,
        #[max_length = 255]
        to_currency_id -> Varchar,
        rate -> Decimal,
        date -> Date,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    currencies,
    currency_rates,
);
