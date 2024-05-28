create table "user_withdrawal_fee"
(
    id                    uuid        primary key default gen_random_uuid(),

    user_id               uuid        not null references "user" (id),
    withdrawal_fee_id     uuid        not null references "withdrawal_fee" (id),

    created_at            timestamp   not null default now(),
    updated_at            timestamp   not null default now()
);

create index "user_withdrawal_user_id_idx" on "user_withdrawal_fee" (user_id);
create index "user_withdrawal_withdrawal_fee_id_idx" on "user_withdrawal_fee" (withdrawal_fee_id);

create index "user_withdrawal_fee_created_at_idx" on "user_withdrawal_fee" (created_at);
create index "user_withdrawal_fee_updated_at_idx" on "user_withdrawal_fee" (updated_at);
