create table if not exists users (
    id serial primary key,
    username varchar(255) not null unique,
    password varchar(255)
);

create table if not exists todos (
    id serial primary key,
    user_id integer references users(id) on delete cascade not null,
    todo varchar(255) not null,
    category varchar(100),
    deadline date
);