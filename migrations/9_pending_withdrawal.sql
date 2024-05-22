create table "pending_withdrawal"
(
    id                  uuid        primary key default gen_random_uuid(),

    status              text        not null,

    withdrawal_id       uuid        not null,
    user_id             uuid        not null references "user" (id),

    created_at          timestamp   not null default now(),
    updated_at          timestamp   not null default now()
);


create index "pending_withdrawal_withdrawal_id_idx" on "pending_withdrawal" (withdrawal_id);
create index "pending_withdrawal_user_id_idx" on "pending_withdrawal" (user_id);
create index "pending_withdrawal_created_at_idx" on "pending_withdrawal" (created_at);
create index "pending_withdrawal_updated_at_idx" on "pending_withdrawal" (updated_at);
