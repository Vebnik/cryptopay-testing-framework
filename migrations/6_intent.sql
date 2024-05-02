create table "intent"
(
    id                  uuid        primary key default gen_random_uuid(),

    sender              text        not null,
    rate                numeric     not null,
    status              text        not null default 'pending',
    expiry              timestamp   not null,
    amount              bytea       not null,

    user_id             uuid        not null references "user" (id),
    network_id          uuid        not null references "network" (id),
    asset_id            uuid        not null references "asset" (id),

    created_at          timestamp   not null default now(),
    updated_at          timestamp   not null default now()
);

create index "intent_sender_idx" on "intent" (sender);
create index "intent_status_idx" on "intent" (status);
create index "intent_expiry_idx" on "intent" (expiry);
create index "intent_user_id_idx" on "intent" (user_id);
create index "intent_network_id_idx" on "intent" (network_id);
create index "intent_asset_id_idx" on "intent" (asset_id);
create index "intent_created_at_idx" on "intent" (created_at);
create index "intent_updated_at_idx" on "intent" (updated_at);
