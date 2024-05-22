create table "hot_wallet"
(
    id              uuid        primary key default gen_random_uuid(),

    address         text        unique not null,
    key             bytea       unique not null,
    nonce           bytea       unique not null,

    network_id      uuid        not null references "network" (id),

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "hot_wallet_network_id_idx" on "hot_wallet" (network_id);
create index "hot_wallet_created_at_idx" on "hot_wallet" (created_at);
create index "hot_wallet_updated_at_idx" on "hot_wallet" (updated_at);
