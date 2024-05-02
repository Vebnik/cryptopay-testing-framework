create table "wallet"
(
    id              uuid        primary key default gen_random_uuid(),

    address         text        unique not null,
    key             bytea       unique not null,
    nonce           bytea       unique not null,

    user_id         uuid        not null references "user" (id),
    network_id      uuid        not null references "network" (id),

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "wallet_user_id_idx" on "wallet" (user_id);
create index "wallet_network_id_idx" on "wallet" (network_id);
create index "wallet_created_at_idx" on "wallet" (created_at);
create index "wallet_updated_at_idx" on "wallet" (updated_at);
