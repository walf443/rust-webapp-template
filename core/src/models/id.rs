kubetsu::define_id!(
    pub struct Id<T, U>;
);
kubetsu_serde::impl_serde!(Id<T, U>);
kubetsu_fake::impl_fake!(Id<T, U>);
kubetsu_sqlx::impl_sqlx!(Id<T, U>);
