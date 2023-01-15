create table organization
(
    id         integer primary key,
    name       text not null,
    created_at date
);

create table project
(
    id              integer primary key,
    name            text not null,
    organization_id bigint
        constraint project_organization_id_fk
            references organization
);

create table "user"
(
    id       integer primary key,
    email    text,
    password text,
    name     text
);

create unique index email_idx
    on "user" (email);
