use anyhow::{Result, anyhow};
use async_trait::async_trait;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Namespace};
use crate::{CONFIG, domain::ports::{ProfilesRepository, Profile}, utils::toServerError, proto::UserRegisteredEvent};
use serde::Serialize;

pub struct SurrealdbAdapter {
  connection: Surreal<Client>
}

impl SurrealdbAdapter {
  // new instantiates SurrealdbAdapter and establishes connection with the database. The instance is
  // returned.
  pub async fn new( ) -> Self {
    let url= &CONFIG.SURREALDB_URL;

    let connection= Surreal::new::<Ws>(url)
      .await.expect(&format!("ERROR: Connecting to Surrealdb at {}", url));

    connection.signin(Namespace {
      namespace: "profile_microservice",
      username: "profile_microservice",
      password: &CONFIG.SURREALDB_PASSWORD
    })
      .await.expect("ERROR: Signing in to Surrealdb");

    connection.use_db("profile_microservice")
              .await.expect("ERROR: Using namespace and database of Surrealdb");

    println!("INFO: Connected to Surrealdb");

    Self { connection }
  }
}

#[async_trait]
impl ProfilesRepository for SurrealdbAdapter {

  async fn createProfile(&self, args: UserRegisteredEvent) -> Result<( )> {

    #[derive(Serialize)]
    struct QueryContent {

      #[serde(rename= "user_id")]
      userId: String,

      name: String,
      username: String
    }

    let _: Vec<Profile> = self.connection.create("profiles")
      .content(QueryContent {
        userId: args.user_id.to_string( ),
        name: args.name.to_string( ),
        username: args.username.to_string( ),
      })
      .await.map_err(toServerError)?;

    Ok(( ))
  }

  async fn searchProfiles(&self, searchQuery: &str) -> Result<Vec<Profile>> {
    let query=
      "SELECT * FROM profiles WHERE name @@ $query OR username @@ $query";

    #[derive(Serialize)]
    struct QueryBinding {
      query: String
    }

    let profiles: Vec<Profile> = self.connection.query(query)
      .bind(QueryBinding {
        query: searchQuery.to_string( )
      })
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    Ok(profiles)
  }

  async fn getProfileByUserId(&self, userId: &str) -> Result<Profile> {
    let query=
      "SELECT * FROM profiles WHERE userId= $user_id";

    #[derive(Serialize)]
    struct QueryBinding {

      #[serde(rename= "user_id")]
      userId: String
    }

    let result: Option<Profile> = self.connection.query(query)
      .bind(QueryBinding {
        userId: userId.to_string( )
      })
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    match result {
      None => Err(anyhow!("Profile not found")),
      Some(profile) => Ok(profile)
    }
  }
}