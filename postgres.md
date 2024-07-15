<!-- tags: databases -->
<!-- hidden -->

# Postgres

<!-- START TAGS -->
[<img src="https://img.shields.io/badge/Tag-databases-brightgreen">](/tags/databases)
<!-- END TAGS -->

I'm learning Postgres for a while and while many things are
_**so much nicer**_ than MySQL (I mean, come on! How long until there's
a timezoned datetime field?), some basics are lacking.
I'm disappointed.


## The good

Postgres is sometimes referred as an object-relational database, as
opposed to what now looks like "static" data types systems. That's
because columns in Postgres belong to an object data type and you can
expand on them by defining your own types.

But the database should behave like a dumb API. It stores and retrieves
data and allows performing simple queries. I can't see how extending
the object type system as necessarily a great thing. Unless, of course,
you are building your own data store atop Postgres. Which then, this
all makes sense. (On the other hand, except in a few places, custom
types reduces database portability and introduces opacity into this
layer.)

Tables can have hierarchical relationships. There're sequences, better
view support, native support for `UUID`s out of the box and many other
goodies. And overall, it's just a better, more compliant RDBMS.

It's superficial but the out-of-the-box `psql` cli is friendlier
than `mysql`. `\l` is and `\d+ table_name` are faster than
`SHOW DATABASES` and `SHOW CREATE TABLE table_name`. All of this though
goes away with `pgcli`, `mycli` and `litecli` tools available.


## The different

Postgres won't return the last inserted id value. It allows instead to
return one or more fields from each query. It's not even limited to
`INSERT INTO table`. It can be used with a
`DELETE FROM table WHERE id = 123 OR id = 456 RETURNING *` and you
loop over the fields of the 2 deleted rows. Pretty neat but it forces
generic database adapters to append that `RETURNING id` to the end of
each insert and load it separately. Some ORMs actually return the whole
row by default, just in case.

Postgres also adds support for JSON and JSONB values but I find this
very weird. In my mind, why the need for a specialised JSON column?
You can put it in a TEXT field and if you need to really index those
values just create a separate table pointing to them. Or use a proper
search engine for this. Doing searches on JSON fields comes with a
performance penalty.

But I get it. It's convenient and for small datasets very useful to use
these sparse indexes.


## The ugly

The ugly is related to how Postgres handles MVCC. It's a long known
problem called "write amplification" and although some proposals where
put forward, none was adopted so far.

This write amplification happens when **for each new update**, even to
a single field, Postgres creates a new row version, while older ones
are marked as dead. Those dead records accumulate in disk and therefore
Postgres needs frequent garbagge collection in the form of `VACUUM`.
This operation can run in parallel and can be scheduled to run after
certain thresholds are hit.

This is because indexes in Postgres store **the whole damned row**.
Another way to say it is: Postgres itself doesn't follow the Normal
Forms. I get it. Normally this would be a trade between more space and
saving one extra disk search (to hidrate Row IDs) but in write
intensive applications mutating data, this costs.

MySQL doesn't suffer from this amplification as data is stored in the
primary btree index and other indexes just hold the primary key value
on their leaves. Updates only affect the primary key btree. Unless of
course, if you update the secondary index value(s) themselves.

[Uber migrated away from Postgres](https://www.uber.com/en-NO/blog/postgres-to-mysql-migration/)
due to this. Plus, the heavier (and weirder) replication, the use of
caches and a whole OS process per connection (solution: install
`pgbouncer` everywhere) leaves a bitter taste.


## The conclusion

Despite all this, I'm migrating some apps to Postgres but for smaller
non-HA stuff I will use SQLite statically linked as a library. This has
to be the best performance, as one skips the whole TCP/IP overheads.
