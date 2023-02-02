create table users(
    id      serial primary key,
    email    text    not null,
    password    text    not null
);