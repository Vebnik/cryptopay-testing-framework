create table "withdrawal"
(
    id                  uuid        primary key default gen_random_uuid(),

    recipient           text        not null,
    amount              bytea       not null,
    tx_hash             text        not null,
    block_number        numeric     not null,
    status              text        not null,

    fee                 bytea       not null,
    fee_tx_hash         text        not null,
    fee_block_number    numeric     not null,
    fee_status          text        not null,

    asset_id            uuid        not null references "asset" (id),
    network_id          uuid        not null references "network" (id),
    wallet_id           uuid        not null references "wallet" (id),
    user_id             uuid        not null references "user" (id),

    created_at          timestamp   not null default now(),
    updated_at          timestamp   not null default now()
);

create index "withdrawal_recipient_idx" on "withdrawal" (recipient);
create index "withdrawal_tx_hash_idx" on "withdrawal" (tx_hash);
create index "withdrawal_block_number_idx" on "withdrawal" (block_number);
create index "withdrawal_fee_tx_hash_idx" on "withdrawal" (fee_tx_hash);
create index "withdrawal_fee_block_number_idx" on "withdrawal" (fee_block_number);
create index "withdrawal_status_idx" on "withdrawal" (status);
create index "withdrawal_fee_status_idx" on "withdrawal" (fee_status);
create index "withdrawal_asset_id_idx" on "withdrawal" (asset_id);
create index "withdrawal_network_id_idx" on "withdrawal" (network_id);
create index "withdrawal_wallet_id_idx" on "withdrawal" (wallet_id);
create index "withdrawal_user_id_idx" on "withdrawal" (user_id);
create index "withdrawal_created_at_idx" on "withdrawal" (created_at);
create index "withdrawal_updated_at_idx" on "withdrawal" (updated_at);
