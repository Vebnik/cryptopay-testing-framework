create table "network"
(
    id              uuid        primary key default gen_random_uuid(),

    name            text        not null,
    kind            text        not null,
    endpoint        text        not null,
    start_block     numeric     not null,
    last_synced     numeric     not null,
    is_disabled     boolean     not null default true,

    created_at      timestamp   not null default now(),
    updated_at      timestamp   not null default now()
);

create index "network_name_idx" on "network" (name);
create index "network_kind_idx" on "network" (kind);
create index "network_start_block_idx" on "network" (start_block);
create index "network_last_synced_idx" on "network" (last_synced);
create index "network_created_at_idx" on "network" (created_at);
create index "network_updated_at_idx" on "network" (updated_at);