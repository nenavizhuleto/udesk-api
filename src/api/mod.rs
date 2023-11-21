mod mw_auth;
mod routes_auth;
mod routes_tickets;
mod routes_users;
mod routes_companies;

pub use routes_auth::routes as auth;
pub use routes_tickets::routes as tickets;
pub use routes_users::routes as users;
pub use routes_companies::routes as companies;
pub use mw_auth::mw_require_auth as auth_mw;


const AUTH_TOKEN: &str = "authtoken";
const JWT_KEY: [u8; 6] = *b"secret";
const AUTH_TEST_TOKEN: &str = "token";
