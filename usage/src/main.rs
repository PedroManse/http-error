use http_error_macro;

fn main() {
    //se::SocError::SQLX(se::SocError::SQLX(sqlx::Error, "".to_string()));
    //se::SocError::URL(url::Error);
    //se::SocError::IntOutOfRange(se::IntOutOfRange{min: 0, max: 100, got: 200});
    println!("Hello :)");
}

struct E {
    a: i32
}

//http_error_macro::http_error!(
//    SERR {
//        SQLX=E,
//        SQLY=(i32, i32),
//        SQLZ={u:i32}
//    }
//);
//use se::*;

pub mod url {
    pub struct Error;
}
pub mod sqlx {
    pub struct Error;
}

http_error_macro::http_error! (
    SocError {
        URL=url::Error,
        SQLX=(sqlx::Error, String),
        IntOutOfRange={min:i64, max:i64, got: i64}
    }
);


