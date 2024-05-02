create table "asset"
(
    id              uuid        primary key default gen_random_uuid(),

    name            text        not null,
    symbol          text        not null,
    address         text        not null,
    decimals        int         not null,
    min_withdrawal  bytea       not null,
    is_disabled     boolean     not null default true,

    network_id      uuid        not null references "network" (id),

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "asset_created_at_idx" on "asset" (created_at);
create index "asset_updated_at_idx" on "asset" (updated_at);
create index "asset_network_id_idx" on "asset" (network_id);