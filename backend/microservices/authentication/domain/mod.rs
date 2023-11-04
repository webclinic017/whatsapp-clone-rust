pub mod ports {
  use anyhow::Result;
  use async_trait::async_trait;
  use mockall::automock;
  use crate::proto::{StartRegistrationRequest, VerifyUserRequest};

  #[async_trait] // It generates a synchronous wrapper function for each async trait method. This
                 // wrapper function calls the actual async method and awaits its result.
  #[automock]
  pub trait UsersRepository: Send + Sync {

    // emailAndUsernameUniquenessCheck checks whether the user provided email and username are
    // unique (available for registration) or not.
    async fn emailAndUsernameUniquenessCheck(&self, email: &str, username: &str) -> Result<Vec<String>>;

    // createNewUser creates a new unverified temporary user. If the user doesn't verify himself /
    // herself within 5 minutes, then the database record will expire.
    async fn createNewUser(&self, args: &StartRegistrationRequest, verificationCode: &str) -> Result<( )>;

    // verifyUser checks whether the registration verification code provided by the user is correct
    // or not.
    async fn verifyUser(&self, args: &VerifyUserRequest) -> Result<( )>;

    // checkPassword checks whether the user provided password is correct or not (identifier can be
    // email / username). If right, then the user id is returned.
    async fn checkPassword(&self, identifier: &str, password: &str) -> Result<String>;

    // verifiedUserWithIdExists checks whether a verified user with the given id exists or not.
    async fn verifiedUserWithIdExists(&self, id: &str) -> Result<( )>;
  }
}

pub mod usecases {
  use anyhow::{Result, anyhow};
  use crate::{
    proto::{StartRegistrationRequest, VerifyUserRequest, SigninResponse, SigninRequest},
    utils::{generateOtp, createJwt, decodeJwt}
  };
  use super::ports::UsersRepository;
  use derive_more::Constructor;

  #[derive(Constructor)]
  pub struct Usecases {
    db: Box<dyn UsersRepository>
  }

  impl Usecases {

    pub async fn startRegistration(&self, args: &StartRegistrationRequest) -> Result<( )> {
      match self.db.emailAndUsernameUniquenessCheck(&args.email, &args.username).await {
        Err(error) => return Err(error),

        Ok(errors) =>
          if errors.len( ) > 0 {
            return Err(anyhow!(errors.join(" | ")))}
      };

      let verificationCode= generateOtp( );

      self.db.createNewUser(args, &verificationCode).await
    }

    pub async fn verifyUser(&self, args: &VerifyUserRequest) -> Result<( )> {
      self.db.verifyUser(args).await
    }

    pub async fn signin(&self, args: SigninRequest) -> Result<SigninResponse> {
      let id= self.db.checkPassword(&args.identifier, &args.password).await?;

      let jwt= createJwt(id)?;
      Ok(SigninResponse{ jwt })
    }

    // verifyJwt verifies a JWT.
    pub async fn verifyJwt(&self, jwt: &str) -> Result<( )> {
      let id= decodeJwt(jwt)?;
      self.db.verifiedUserWithIdExists(&id).await
    }
  }
}