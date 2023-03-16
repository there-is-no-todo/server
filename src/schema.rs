// @generated automatically by Diesel CLI.

diesel::table! {
    plans (id) {
        id -> Nullable<Integer>,
        title -> Text,
        from_hr -> Nullable<Integer>,
        from_min -> Nullable<Integer>,
        to_hr -> Nullable<Integer>,
        to_min -> Nullable<Integer>,
        started -> Nullable<Bool>,
    }
}
