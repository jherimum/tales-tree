CREATE TYPE fragment_state AS ENUM ('draft', 'published', 'waiting_review', 'rejected', 'rejected', 'waiting_changes');
CREATE TYPE review_action AS ENUM ('approve', 'reject', 'request_changes');

create table users(
    id                  uuid            not null,
    constraint users_pk primary key (id),
);

create table fragments(
    id                  uuid            not null,
    author_id           uuid            not null,
    content             varchar         not null,
    state               fragment_state  not null,
    parent_id           uuid            null,
    path                uuid[]          not null,
    created_at          timestamp       not null,
    last_modified_at    timestamp       not null,
    
    constraint fragments_pk primary key (id),
    constraint fragments_fk_author foreign key (author_id) references users(id),
    constraint fragments_fk_parent foreign key (parent_id) references fragments(id)
);

create table reviews(
    id                  uuid            not null,
    fragment_id         uuid            not null,
    reviewer_id         uuid            not null,
    comment             varchar         null,
    created_at          timestamp       not null,
    action              review_action   not null,

    constraint reviews_pk primary key (id),
    constraint reviews_fk_fragment foreign key (fragment_id) references fragments(id),
    constraint reviews_fk_reviewer foreign key (reviewer_id) references users(id)
);


create table likes(
    fragment_id         uuid            not null,
    user_id             uuid            not null,
    created_at          timestamp       not null,

    constraint likes_pk primary key (fragment_id, user_id),
    constraint likes_fk_fragment foreign key (fragment_id) references fragments(id),
    constraint likes_fk_user foreign key (user_id) references users(id)
);


create table follows(
    follower_id         uuid            not null,
    followee_id         uuid            not null,
    created_at          timestamp       not null,
    constraint follows_pk primary key (follower_id, followee_id),
    constraint follows_fk_follower foreign key (follower_id) references users(id),
    constraint follows_fk_followee foreign key (followee_id) references users(id)
);
