<!-- tags: databases -->
<!-- _hidden: true -->

# Postgres

<!-- START TAGS -->
[<img src="https://img.shields.io/badge/Tag-databases-brightgreen">](/tags/databases)
<!-- END TAGS -->

I'm learning Postgres for a while and while many things are so much
nicer than MySQL, the basics are lacking. I'm disappointed.


## The basics

The database should behave like a dumb API. It stores, fetches and
allow querying the data.

Business logic should stay in the application layer.

Postgres is sometimes defined as an object-relational database, as
opposed to more traditional pure-reational databases. That's because
columns in Postgres belong to an object data type. You can create your
own object types. Even tables can have hierarchical relationships


## The good

The `psql` cli is friendlier than `mysql`. `\l` is and `\d+ table_name`
are faster than `SHOW DATABASES` and `SHOW CREATE TABLE table_name`.

Also Postgres support `UUID` out of the box as a


## The ~bad~ uncoventional

Postgres won't return the last inserted id value. It allows instead to
return one or more fields from each query. It's not even limited to
`INSERT INTO table`. It can be used with a
`DELETE FROM table WHERE id = 123 or id = 456 RETURNING *` and you
loop over the fields of the 2 deleted rows. Pretty neat but it forces
generic database adapters to append that `RETURNING id` to the end of
each insert and load it separately.

Postgres also adds support for JSON and JSONB values but I find this
very weird. In my mind, why the need for a specialised JSON column?
You can put it in a TEXT field and if you need to really index those
values just


## The ugly

The ugly is related to how Postgres handles MVCC. It's a long known
problem called "write amplification" and although some proposals where
put forward, none was adopted.

This write amplification happenswhen for each new update, even to a
single field, Postgres creates a new row version, while older ones are
marked as dead. Those dead records do accumulate in disk and therefore
Postgres needs frequent garbagge collection in the form of `VACUUM`.
This operation can run in parallel and can be scheduled to run after
certain thresholds are hit.

But still it needs to happen. MySQL doesn't suffer from this
amplification as data is stored in the primary btree index and other
indexes just hold the primary key value on their leaves. Updates only
affect the primary key btree, unless of course, if you update the
secondary index value itself.


<!-- START FOOTER -->
 &nbsp;

<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-app.js"></script>
<script src="https://www.gstatic.com/firebasejs/8.10.0/firebase-database.js"></script>
<script src="https://jpedro.github.io/js/v1/data.js"></script>
<script src="https://jpedro.github.io/js/v1/comments.js"></script>
<script defer="">Comments.mount(document.body.children[0]);</script>
<!-- END FOOTER -->
