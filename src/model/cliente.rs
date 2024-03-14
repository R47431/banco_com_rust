use sqlx::{FromRow};


#[derive(FromRow,Debug)]
pub struct Cliente {
    pub id: i32,
    pub nome: String,
}


