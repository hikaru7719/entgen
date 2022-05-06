CREATE TABLE users (
    id uuid PRIMARY KEY,
    name varchar(255) NOT NULL,
    nickname varchar(255),
    created_at timestamp NOT NULL
);

CREATE TABLE nums (
    aaa smallint NOT NULL,
    bbb integer NOT NULL,
    ccc bigint NOT NULL,
    ddd smallserial NOT NULL,
    eee serial NOT NULL,
    fff bigserial NOT NULL,
    ggg real NOT NULL,
    hhh double precision NOT NULL,
    iii decimal NOT NULL,
    jjj numeric NOT NULL
);

CREATE TABLE strs (
    aaa char(5) NOT NULL,
    bbb varchar(5) NOT NULL,
    ccc text NOT NULL,
    ddd name NOT NULL,
    eee char NOT NULL
)
