CREATE TABLE projects
( id BIGINT PRIMARY KEY DEFAULT UUID_SHORT(),
  gh_owner VARCHAR(39),
  gh_repo VARCHAR(100),
  gh_l8st_rel VARCHAR(11),
  dh_owner VARCHAR(255),
  dh_repo VARCHAR(255),
  dh_l8st_tag VARCHAR(255),
  CONSTRAINT gh_owner_repo_unique UNIQUE (gh_owner, gh_repo),
  CONSTRAINT dh_owner_repo_unique UNIQUE (dh_owner, dh_repo)
);
