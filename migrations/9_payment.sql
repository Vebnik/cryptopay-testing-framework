create table "payment"
(
    id              uuid        primary key default gen_random_uuid(),

    sender          text        not null,
    amount          bytea       not null,
    tx_hash         text        not null,
    block_number    numeric     not null,
    status          text        not null,

    asset_id        uuid        not null references "asset" (id),
    network_id      uuid        not null references "network" (id),
    wallet_id       uuid        not null references "wallet" (id),
    user_id         uuid        not null references "user" (id),
    intent_id       uuid                 references "intent" (id),

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "payment_sender_idx" on "payment" (sender);
create index "payment_tx_hash_idx" on "payment" (tx_hash);
create index "payment_block_number_idx" on "payment" (block_number);
create index "payment_status_idx" on "payment" (status);

create index "payment_asset_id_idx" on "payment" (asset_id);
create index "payment_network_id_idx" on "payment" (network_id);
create index "payment_wallet_id_idx" on "payment" (wallet_id);
create index "payment_user_id_idx" on "payment" (user_id);
create index "payment_intent_id_idx" on "payment" (intent_id);

create index "payment_created_at_idx" on "payment" (created_at);
create index "payment_updated_at_idx" on "payment" (updated_at);
