ALTER USER postgres SET timezone='Asia/Tokyo';

create table users (
    id serial primary key,
    name varchar(255) not null,
    email varchar(255) not null unique,
    password varchar(255) not null,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp
);

create table messages (
    id serial primary key,
    user_id integer not null,
    message text not null,
    parent_id integer,
    message_time timestamptz not null default current_timestamp,
    created_at timestamptz not null default current_timestamp,
    updated_at timestamptz not null default current_timestamp,
    foreign key (user_id) references users (id)
);