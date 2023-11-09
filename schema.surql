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

    DEFINE USER outboxer ON DATABASE
      PASSWORD '1yp5m]NOU-da+s#'
      ROLES EDITOR;

    DEFINE TABLE outboxer SCHEMAFULL;

      DEFINE FIELD message ON TABLE outboxer TYPE bytes;

      DEFINE FIELD locked ON TABLE outboxer TYPE bool;
      DEFINE FIELD locked_at ON TABLE outboxer TYPE datetime;

### Profile Microservice ###
DEFINE NAMESPACE profile_microservice;
USE NAMESPACE profile_microservice;

  DEFINE USER profile_microservice ON NAMESPACE
    PASSWORD 'nZbVXGHp3U98'
    ROLES EDITOR;

  DEFINE DATABASE profile_microservice;
  USE DB profile_microservice;

    DEFINE TABLE profiles SCHEMAFULL;

      DEFINE FIELD user_id ON TABLE profiles TYPE string;

      DEFINE FIELD name ON TABLE profiles TYPE string;
      DEFINE FIELD username ON TABLE profiles TYPE string;

      // An analyzer plays a crucial role in text processing and searching.
      DEFINE ANALYZER default
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

      DEFINE INDEX name_index ON TABLE profiles
        FIELDS name
        SEARCH
        ANALYZER default
        BM25 // Search algorithm used.
        HIGHLIGHTS;

      DEFINE INDEX username_index ON TABLE profiles
        FIELDS username
        SEARCH
        ANALYZER default
        BM25
        HIGHLIGHTS;

### Chat Microservice ###
DEFINE NAMESPACE chat_microservice;
USE NAMESPACE chat_microservice;

  DEFINE USER chat_microservice ON NAMESPACE
    PASSWORD 'n}Zb!VXGHp3U98'
    ROLES EDITOR;

  DEFINE DATABASE chat_microservice;
  USE DB chat_microservice;

    DEFINE TABLE chats SCHEMAFULL;