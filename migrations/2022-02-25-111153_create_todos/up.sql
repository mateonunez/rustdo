CREATE TABLE todos (
  id varchar(36) PRIMARY KEY,
  title varchar(255) NOT NULL,
  description text NULL,
  completed boolean NOT NULL DEFAULT false,
  created_at timestamp NOT NULL DEFAULT now()
);