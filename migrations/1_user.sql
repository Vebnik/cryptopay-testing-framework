create table "user"
(
    id              uuid        primary key default gen_random_uuid(),

    name            text        not null,
    email           text        unique not null,
    password        text        not null,
    currency        text        not null,
    is_admin        boolean     not null default false,

    is_verified     boolean     not null default false,
    email_token     text,

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "user_currency_idx" on "user" (currency);
create index "user_created_at_idx" on "user" (created_at);
create index "user_updated_at_idx" on "user" (updated_at);