CREATE DATABASE IF NOT EXISTS authentication_microservice;

  CREATE TABLE IF NOT EXISTS authentication_microservice.users (
    -- Each table automatically has a primary index called <table name>_pkey, which indexes either
    -- its primary key or, if there is no primary key, a unique value for each row as rowid.
    -- We recommend always defining a primary key because the index it creates provides much better
    -- performance than letting CockroachDB use rowid.
    id UUID DEFAULT gen_random_uuid( ) PRIMARY KEY,

    name VARCHAR(50) NOT NULL,

    -- CockroachDB automatically creates secondary indexes for columns with a UNIQUE constraint.

    email VARCHAR(254) UNIQUE NOT NULL, -- According to RFC 5321, max length of an email can be 254.
    username VARCHAR(50) UNIQUE NOT NULL,

    password VARCHAR(50) NOT NULL,

    UNIQUE INDEX email_username (email, username) STORING(password),

    created_at TIMESTAMP DEFAULT now( ),

    is_verified BOOL DEFAULT false,
    verification_code VARCHAR(6),

    -- Once rows are expired, they are eligible to be deleted. However, eligible rows may not be
    -- deleted right away. Instead, they are scheduled for deletion using a background job that is
    -- run at the interval defined by the ttl_job_cron storage parameter.
    expires_at TIMESTAMPTZ DEFAULT (now( ) + INTERVAL '5 minutes')
  ) WITH (ttl_expiration_expression = 'expires_at');

  CREATE TABLE IF NOT EXISTS authentication_microservice.outboxer (
    id UUID DEFAULT gen_random_uuid( ) PRIMARY KEY,

    data BYTES NOT NULL,
    to_queue VARCHAR(50) NOT NULL,

    is_locked BOOL DEFAULT FALSE
  );

CREATE DATABASE IF NOT EXISTS profiles_microservice;

  CREATE TABLE IF NOT EXISTS profiles_microservice.profiles (
    id UUID PRIMARY KEY,

    name VARCHAR(50) NOT NULL,
    username VARCHAR(50) UNIQUE NOT NULL,

    profile_picture_url VARCHAR(300)
  );

  CREATE TABLE IF NOT EXISTS profiles_microservice.outboxer (
    id UUID DEFAULT gen_random_uuid( ) PRIMARY KEY,

    data BYTES NOT NULL,
    to_queue VARCHAR(50) NOT NULL,

    is_locked BOOL DEFAULT FALSE
  );

CREATE DATABASE IF NOT EXISTS followships_microservice;

  CREATE TABLE IF NOT EXISTS followships_microservice.profiles (
    followee UUID NOT NULL,
    follower UUID NOT NULL
  );

CREATE DATABASE IF NOT EXISTS posts_microservice;

  CREATE TABLE IF NOT EXISTS posts_microservice.profiles (
    id UUID DEFAULT gen_random_uuid( ) PRIMARY KEY,

    creator_id UUID NOT NULL,
    INDEX creator_id (creator_id),

    picture_url VARCHAR(300) NOT NULL,
    caption VARCHAR(250),

    created_at TIMESTAMP DEFAULT current_timestamp( )
  );

  CREATE TABLE IF NOT EXISTS posts_microservice.outboxer (
    id UUID DEFAULT gen_random_uuid( ) PRIMARY KEY,

    data BYTES NOT NULL,
    to_queue VARCHAR(50) NOT NULL,

    is_locked BOOL DEFAULT FALSE
  );