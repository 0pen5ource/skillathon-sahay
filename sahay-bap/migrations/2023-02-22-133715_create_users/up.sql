CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   name VARCHAR NOT NULL,
   email VARCHAR UNIQUE NOT NULL,
   phone VARCHAR NOT NULL default '',
   telegram_handle VARCHAR NOT NULL default '',
   otp VARCHAR NOT NULL default '',
   session_token VARCHAR NOT NULL default '',
   verification_count INTEGER NOT NULL DEFAULT 0,
   is_verified BOOLEAN NOT NULL DEFAULT FALSE
);
