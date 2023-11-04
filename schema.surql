### Authentication Microservice ###
DEFINE NAMESPACE authentication_microservice;
USE NS authentication_microservice;

  DEFINE USER authentication_microservice ON NAMESPACE
    PASSWORD 'U%xVZ=HSQ8Z=F8='
    ROLES EDITOR; // Can view and edit any resource on the user's level or below, but not users or
                  // token (IAM) resources.

  DEFINE DATABASE authentication_microservice;
  USE DB authentication_microservice;

    DEFINE TABLE users SCHEMAFULL;

      DEFINE FIELD name ON TABLE users TYPE string;

      DEFINE FIELD email ON TABLE users TYPE string;
      DEFINE INDEX email_index ON TABLE users COLUMNS email UNIQUE;

      DEFINE FIELD username ON TABLE users TYPE string;
      DEFINE INDEX username_index ON TABLE users COLUMNS username UNIQUE;

      DEFINE FIELD password ON TABLE users TYPE string;

      DEFINE FIELD is_verified ON TABLE users TYPE bool DEFAULT false;
      DEFINE FIELD verification_code ON TABLE users TYPE string;

      DEFINE FIELD created_at ON TABLE users TYPE datetime
        DEFAULT time::now( );

### Profile Microservice ###
DEFINE NAMESPACE profile_microservice;
USE NAMESPACE profile_microservice;

  DEFINE USER profile_microservice ON NAMESPACE
    PASSWORD 'n}Zb!VXGHp3U98'
    ROLES EDITOR;

  DEFINE DATABASE profile_microservice;
  USE DB profile_microservice;

    DEFINE TABLE profiles SCHEMAFULL;

      DEFINE FIELD name ON TABLE profiles TYPE string;
      DEFINE FIELD username ON TABLE profiles TYPE string;

      // An analyzer plays a crucial role in text processing and searching.
      DEFINE ANALYZER name_username
        // Tokenizers are responsible for breaking down a given text into individual tokens based on
        // a set of instructions. The blank tokenizer breaks down a text into tokens by creating a
        // new token each time it encounters a space, tab, or newline character.
        TOKENIZERS blank
        FILTERS
          ascii, // Responsible for processing tokens by replacing or removing diacritical marks
                 // (accents and special characters) from the text. It helps standardize text by
                 // converting accented characters to their basic ASCII equivalents.
          lowercase,
          uppercase;

      DEFINE INDEX name_username ON TABLE profiles
        FIELDS username, name
        SEARCH
        ANALYZER name_username
        BM25 // Search algorithm used.
        HIGHLIGHTS;

      DEFINE FIELD picture_uri ON TABLE profiles TYPE option<string>;

### Followships Microservice ###
DEFINE NAMESPACE followships_microservice;
USE NAMESPACE followships_microservice;

  DEFINE USER followships_microservice ON NAMESPACE
    PASSWORD '(10}_-r@1NA%qt$'
    ROLES EDITOR;

  DEFINE DATABASE followships_microservice;
  USE DB followships_microservice;

    DEFINE TABLE followships SCHEMAFULL;

      DEFINE FIELD followee_id ON TABLE followships TYPE string;
      DEFINE FIELD follower_id ON TABLE followships TYPE string;

### Posts Microservice ###
DEFINE NAMESPACE posts_microservice;
USE NAMESPACE posts_microservice;

  DEFINE USER posts_microservice ON NAMESPACE
    PASSWORD 'Mj489fF5FBlisKV'
    ROLES EDITOR;

  DEFINE DATABASE posts_microservice;
  USE DB posts_microservice;

    DEFINE TABLE posts SCHEMAFULL;

      DEFINE FIELD creator_id on TABLE posts TYPE string;
      DEFINE FIELD picture_uri on TABLE posts TYPE string;
      DEFINE FIELD caption on TABLE posts TYPE option<string>;

### Likes Microservice ###
DEFINE NAMESPACE likes_microservice;
USE NAMESPACE likes_microservice;

  DEFINE USER likes_microservice ON NAMESPACE
    PASSWORD '@zJcRKZ&MLl=6CT'
    ROLES EDITOR;

  DEFINE DATABASE likes_microservice;
  USE DB likes_microservice;

    DEFINE TABLE likes SCHEMAFULL;

      DEFINE FIELD post_id on TABLE likes TYPE string;
      DEFINE FIELD user_id on TABLE likes TYPE string;