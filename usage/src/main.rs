use http_error_macro;

fn main() {
    let a = SQLX{a:0};
    let b = SQLY(1, 2);
    let c = SQLZ{u:3};
    println!("Hello :)");
}

struct E {
    a: i32
}

http_error_macro::http_error!(
    SQLX=E,
    SQLY=(i32, i32),
    SQLZ={u:i32}
);

//http_error! ( mod se // optinaly put into a module
//    SocError {
//        URL=url::Error, // depends on type's Display
//        SQLX=(sqlx::Error, String), // impl Debug and Display
//        IntOutOfRange={min:i64, max:i64, got: i64}, // impl Debug and Display
//    }
//);
//
//se::SocError::SQLX(SocError::SQLX(sqlx::Error, String));
//se::SocError::URL(url::Error);
//se::SocError::IntOutOfRange(se::IntOutOfRange{min: 0, max: 100, got: 200});
