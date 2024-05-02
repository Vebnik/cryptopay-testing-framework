create table "balance"
(
    id              uuid        primary key default gen_random_uuid(),

    asset_id        uuid        not null references "asset" (id),
    network_id      uuid        not null references "network" (id),
    wallet_id       uuid        not null references "wallet" (id),

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "balance_asset_id_idx" on "balance" (asset_id);
create index "balance_network_id_idx" on "balance" (network_id);
create index "balance_wallet_id_idx" on "balance" (wallet_id);
create index "balance_created_at_idx" on "balance" (created_at);
create index "balance_updated_at_idx" on "balance" (updated_at);
