table! {
    accounts (accountnmb) {
        id -> Int8,
        firstname -> Varchar,
        lastname -> Varchar,
        username -> Varchar,
        email -> Varchar,
        mobile -> Varchar,
        accountnmb -> Varchar,
        opening_balance -> Int8,
        current_balance -> Int8,
        password -> Varchar,
        ip_address -> Varchar,
        isactive -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Varchar>,
    }
}

table! {
    transactions (id) {
        id -> Int8,
        accountnmb -> Nullable<Varchar>,
        recepient_accnt_nmb -> Varchar,
        recepient_name -> Varchar,
        amount -> Int8,
        trans_type -> Varchar,
        trans_mode -> Varchar,
        createdat -> Nullable<Timestamp>,
        updatedat -> Nullable<Timestamp>,
    }
}

joinable!(transactions -> accounts (accountnmb));

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
