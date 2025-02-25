
fn main() {
    println!("Hello :)");
}

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
