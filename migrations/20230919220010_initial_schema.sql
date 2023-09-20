CREATE TYPE fragment_state AS ENUM ('draft', 'published', 'submitted', 'rejected', 'changes_Requested');

create table fragments(
    id          uuid        not null,
    author_id   uuid        not null,
    content     varchar   not null,
    state       fragment_state not null,
    parent_id  uuid       null,
    path       uuid[]   not null,
    created_at     timestamp     not null,
    last_modified_at timestamp not null
);


