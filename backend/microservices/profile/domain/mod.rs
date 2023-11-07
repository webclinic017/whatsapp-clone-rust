pub mod ports {
  use anyhow::Result;
  use async_trait::async_trait;
  use mockall::automock;
  use surrealdb::sql::Thing;
  use serde::Deserialize;
  use crate::proto::UserRegisteredEvent;

  #[async_trait]
  #[automock]
  pub trait ProfilesRepository: Send + Sync {

    // createProfile creates a profile.
    async fn createProfile(&self, args: UserRegisteredEvent) -> Result<( )>;

    // searchProfiles takes in a search query and searches the profiles repository by the name and
    // username fields. The search results are returned.
    async fn searchProfiles(&self, query: &str) -> Result<Vec<Profile>>;

    // getProfileByUserId returns the profile with the given user-id.
    async fn getProfileByUserId(&self, userId: &str) -> Result<Profile>;

  }

  #[derive(Deserialize, Debug)]
  pub struct Profile {
    pub id: Thing,

    #[serde(rename= "user_id")]
    pub userId: String,

    pub name: String,
    pub username: String
  }
}

pub mod usecases {
  use anyhow::Result;
  use derive_more::Constructor;
  use super::ports::{ProfilesRepository, Profile};
  use crate::proto::UserRegisteredEvent;

  #[derive(Constructor)]
  pub struct Usecases {
    db: Box<dyn ProfilesRepository>
  }

  impl Usecases {

    pub async fn createProfile(&self, args: UserRegisteredEvent) -> Result<( )> {
      self.db.createProfile(args).await
    }

    pub async fn searchProfiles(&self, query: &str) -> Result<Vec<Profile>> {
      self.db.searchProfiles(query).await
    }

    pub async fn getProfileByUserId(&self, userId: &str) -> Result<Profile> {
      self.db.getProfileByUserId(userId).await
    }
  }
}