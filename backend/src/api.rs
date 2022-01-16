pub mod users;

pub fn router() -> tide::Server<()> {
  let mut router = tide::Server::new();

  router.at("/users").nest(users::router());

  router
}
