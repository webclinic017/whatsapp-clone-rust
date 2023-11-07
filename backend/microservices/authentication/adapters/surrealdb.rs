use anyhow::{Result, anyhow};
use argon2::{Argon2, password_hash::{SaltString, rand_core::OsRng, Encoding}, PasswordHasher, PasswordVerifier, PasswordHash};
use async_trait::async_trait;
use chrono::{Duration, Utc, DateTime};
use lazy_static::lazy_static;
use surrealdb::{Surreal, engine::remote::ws::{Ws, Client}, opt::auth::Namespace, sql::Thing};
use serde::{Serialize, Deserialize};
use validator::validate_email;
use crate::{CONFIG, proto::{StartRegistrationRequest, VerifyUserRequest}, domain::ports::UsersRepository, utils::toServerError};

lazy_static! {
  static ref ARGON2: Argon2<'static> = Argon2::default( );
}

#[derive(Deserialize, Debug)]
struct User {
  id: Thing,

  name: String,
  email: String,
  username: String,
  password: String,

  #[serde(rename= "is_verified")]
  isVerified: bool,

  #[serde(rename= "verification_code")]
  verificationCode: String,

  #[serde(rename= "created_at")]
  createdAt: DateTime<Utc>
}

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
      namespace: "authentication_microservice",
      username: "authentication_microservice",
      password: &CONFIG.SURREALDB_PASSWORD
    })
      .await.expect("ERROR: Signing in to Surrealdb");

    connection.use_db("authentication_microservice")
              .await.expect("ERROR: Using namespace and database of Surrealdb");

    println!("INFO: Connected to Surrealdb");

    Self { connection }
  }

  // hasUserRecordExpired returns whether a user record (of an unverified user) has expired or not.
  fn hasUserRecordExpired(&self, id: &str, createdAt: DateTime<Utc>) -> bool {
    let hasUserRecordExpired=
      Utc::now( ).signed_duration_since(createdAt) > Duration::minutes(5);

    return hasUserRecordExpired
  }
}

#[async_trait]
impl UsersRepository for SurrealdbAdapter {

  async fn emailAndUsernameUniquenessCheck(&self, email: &str, username: &str) -> Result<Vec<String>> {
    let query=
      "SELECT email, username, created_at, is_verified FROM users WHERE email = $email OR username = $username LIMIT 2";

    #[derive(Serialize)]
    struct QueryBinding {
      email: String,
      username: String
    }

    #[derive(Deserialize)]
    struct QueriesItem {
      id: String,

      email: String,
      username: String,

      #[serde(rename= "is_verified")]
      isVerified: bool,

      #[serde(rename= "created_at")]
      createdAt: DateTime<Utc>
    }

    let existingUsers: Vec<QueriesItem> = self.connection.query(query)
      .bind(QueryBinding {
        email: email.to_string( ),
        username: username.to_string( )
      })
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    let mut errors = Vec::<String>::with_capacity(2);

    for existingUser in existingUsers {
      if !existingUser.isVerified && self.hasUserRecordExpired(&existingUser.id, existingUser.createdAt) { continue }

      if email == existingUser.email {
        errors.push("Email is already registered".to_string( ));
      }
      if username == existingUser.username {
        errors.push("Username is already taken".to_string( ));
      }
    }

    Ok(errors)
  }

  async fn createNewUser(&self, args: &StartRegistrationRequest, verificationCode: &str) -> Result<( )> {

    let salt= SaltString::generate(&mut OsRng);
    let password= ARGON2.hash_password(args.password.as_bytes( ), &salt)
                        .map_err(toServerError)?
                        .to_string( );

    #[derive(Serialize)]
    struct QueryContent {
      name: String,
      email: String,
      username: String,
      password: String,

      #[serde(rename= "verification_code")]
      verificationCode: String
    }

    let _: Vec<User> = self.connection.create("users")
      .content(QueryContent {
        name: args.name.to_string( ),
        email: args.email.to_string( ),
        username: args.username.to_string( ),
        password,
        verificationCode: verificationCode.to_string( )
      })
      .await.map_err(toServerError)?;

    Ok(( ))
  }

  async fn verifyUser(&self, args: &VerifyUserRequest) -> Result<( )> {
    let query=
      "SELECT id, verification_code, created_at FROM users WHERE email = $email LIMIT 1";

    #[derive(Serialize)]
    struct QueryBinding {
      email: String
    }

    #[derive(Deserialize)]
    struct QueryResult {
      id: Thing,

      #[serde(rename= "verification_code")]
      verificationCode: String,

      #[serde(rename= "created_at")]
      createdAt: DateTime<Utc>
    }

    let user: Option<QueryResult> = self.connection.query(query)
      .bind(QueryBinding {
        email: args.email.clone( )
      })
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    let id= match user {
      None => return Err(anyhow!("User not found")),

      Some(user) => {
        let id= user.id.to_string( );

        if self.hasUserRecordExpired(&id, user.createdAt) {
          return Err(anyhow!("User not found"))}

        if user.verificationCode != args.verification_code {
          return Err(anyhow!("Wrong verification code provided"))}

        id
      }
    };

    #[derive(Deserialize)]
    struct UpdateResult { }

    let _: Option<UpdateResult> = self.connection.query(format!("UPDATE {} SET is_verified = true", id))
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    Ok(( ))
  }

  async fn checkPassword(&self, identifier: &str, password: &str) -> Result<String> {

    let identifierType= match validate_email(identifier) {
      true => "email",
      _ => "username"
    };

    let query= format!("SELECT id, password FROM users WHERE {}= $identifier", identifierType);

    #[derive(Serialize)]
    struct QueryBinding {
      identifier: String
    }

    #[derive(Deserialize)]
    struct QueryResult {
      id: String,
      password: String
    }

    let queryResult: Option<QueryResult> = self.connection.query(query)
      .bind(QueryBinding {
        identifier: identifier.to_string( )
      })
      .await.map_err(toServerError)?
      .take(0).map_err(toServerError)?;

    match queryResult {
      None => Err(anyhow!("User not found")),

      Some(user) => {
        let passwordHash: PasswordHash= PasswordHash::parse(&user.password, Encoding::B64).map_err(toServerError)?;

        if let Err(error)= ARGON2.verify_password(&password.as_bytes( ), &passwordHash) {
          return match error {
            argon2::password_hash::errors::Error::Password => Err(anyhow!("Wrong password provided")),
            _ => Err(anyhow!(error))
          }
        }

        Ok(user.id)
      }
    }
  }

  async fn verifiedUserWithIdExists(&self, id: &str) -> Result<( )> {
    let query= "SELECT id FROM users WHERE id= $id";

    #[derive(Serialize)]
    struct QueryBinding {
      id: String
    }

    #[derive(Deserialize)]
    struct QueryResult { }

    let queryResult: Option<QueryResult> = self.connection.query(query)
      .bind(QueryBinding {
        id: id.to_string( )
      })
      .await?.take(0)?;

    match queryResult {
      None => Err(anyhow!("User not found")),

      Some(_) => Ok(( ))
    }
  }
}