drop table if exists book;

create table book
(
    id serial primary key,
    author_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default new()
)

insert into book (id, author_id, name, time) values(1, 1, 'First book', NOW());

insert into book (id, author_id, name, time)
values(2,
       1,
       'Second book',
       NOW());