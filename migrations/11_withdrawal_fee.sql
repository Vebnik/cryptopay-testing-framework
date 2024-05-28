create table "withdrawal_fee"
(
    id                uuid        primary key default gen_random_uuid(),

    min_condition     numeric,
    max_condition     numeric,
    fee_percent       numeric     not null,
    is_default        boolean     not null default true,

    created_at        timestamp   not null default now(),
    updated_at        timestamp   not null default now()
);

create index "withdrawal_fee_min_condition_idx" on "withdrawal_fee" (min_condition);
create index "withdrawal_fee_max_condition_idx" on "withdrawal_fee" (max_condition);
create index "withdrawal_fee_fee_percent_idx" on "withdrawal_fee" (fee_percent);

create index "withdrawal_fee_created_at_idx" on "withdrawal_fee" (created_at);
create index "withdrawal_fee_updated_at_idx" on "withdrawal_fee" (updated_at);
