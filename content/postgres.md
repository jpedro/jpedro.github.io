<!-- tags: databases -->
<!-- hidden -->

# Postgres

I'm learning Postgres for a while and while many things are
_**so much nicer**_ than MySQL. I mean, come on! How long until
there's a datetime type with a timezoned component?

But Postgres misses on some basics. I'm disappointed.


## TL;DR

Postgres feels like a Citroën, with a sleek design and full
of  gadgetry. MySQL is a truck that just keeps running and
replicates better.

<!--
[1] Ok, fine. This was before all cars start to look the same
due to manufacturing consolidation.
-->

## The Good

Postgres is sometimes referred as an object-relational database. That's
because columns in Postgres belong to an object data type and you can
expand on them by creating your custom types from their 43 primitives
types. Yes. 43! `CREATE TYPE Person AS (name VARCHAR, skills TEXT[])`.
You can also create DOMAINS, which basically work as aliases to
types with `CHECK` constraints. All very neat. Opaque but neat.

Tables can have hierarchical relationships, which sounds very OOP'ish.
But not sure how this can be a good thing. Plus, in my naive opinion
database schema tools can handle data hierarchies with more flexibily.
But we'll see, with more use.

There're also sequences, better view support, native support for
`UUID`s, JSONPath functionality and much more candy out of the box.
And it's also a more compliant RDBMS.

It's superficial but the out-of-the-box `psql` cli is friendlier
than `mysql`. `\l` is and `\d+ table_name` are faster than
`SHOW DATABASES` and `SHOW CREATE TABLE table_name` (this though
goes away with `pgcli`, `mycli` and `litecli` tools available).


## The Different

Postgres won't return the last inserted id value by default. It's a
small annoyance. It forces me to add ifs and buts to my SQL adapters.

It _does_ allow instead to return one or more fields from each write.
It's not even limited to `INSERT INTO table ... RETURN <field>`. It can
be used with a `DELETE FROM table WHERE id = 123 OR id = 456 RETURNING *`
and you can loop over those now 2 deleted rows. So that saves you
another round trip to the database. But most of the times, you just
need that new serial id and redirect the page to the `GET /blah/id`.

Some ORMs actually return the whole row by default. Just in case. It
feels a bit wasteful but this is a row oriented system, right? The other
fields are already in memory anyway.

With [the right extension](https://pgt.dev/) you can even write stored
procedures in python, ruby, rust, call `curl` and even
[render html templates](https://postgrest.org) from a pgsql connection.
Some peole don't know the difference between "can" and "should".
Good luck [testing that code](https://pgtap.org/).

Postgres also adds support for JSON and JSONB values but I find this
very weird. In my mind, why the need for a specialised JSON column or
its space optimised version? You can put it in a TEXT field and
if you need to really index some values just create a separate table
pointing to them. Or use a proper search engine for this. Doing
searches on JSON fields comes with a performance penalty. But I do get
that you can shrink the amount of data significantly enough that
searching the records doesn't hurt performance.

And over time people added extensions. Cache storages. Cron schedulers.
Pub/sub mechanisms. Even Cypher graph support.


## The Ugly

The ugly is related to how Postgres handles MVCC. It's a long known
problem called "write amplification" and although some proposals where
put forward, none was adopted so far.

This write amplification happens when **for each new update**, even to
a single field, Postgres creates a new row version, while older ones
are marked as dead. Those tombstoned records accumulate in disk and
therefore Postgres needs to garbage collect them frequently enough,
in the form of `VACUUM`. This operation can run in parallel and can
be scheduled to run after certain thresholds are hit.

This is because indexes in Postgres store **the whole damned row**.
Another way to say it is: Postgres itself doesn't follow the Normal
Forms. I get it. This is a trade between more space and saving one
extra disk searchs (to hidrate Row IDs) but in write intensive
applications, with mutating data, this shows.

MySQL doesn't suffer from this amplification as data is stored in the
primary btree and other indexes just hold the primary key value on their
leaves. Updates only affect the primary key btree. Unless of course,
if you update the secondary index fields. But the extra lookup has't
hurt MySQL performance.

[Uber migrated away from Postgres](https://www.uber.com/en-NO/blog/postgres-to-mysql-migration/)
due to this.

Plus, the heavier and weirder replication, the use of caches and a
whole OS process per connection (solution: install `pgbouncer`
everywhere) leaves a sour taste.


## The Design

At core there's a fundamental fork how Postgres and MySQL were designed.

MySQL just wanted to be the fastest. For a long time, screw referential
integrity. Use the application and build your own triggers. Stored
procedures? You mean, schmore procedures?!... Only added after strong
demand.

To be fair, 99.9% of the time you don't need _and_ you definitely do not
want application logic in the database. I'll fight
[anyone](https://sive.rs/pg) on this. I found the hill I'll die for.
The database is the most critical tech component. The one that's
non-trivial to horizontally scale. So keep it lean, keep it dumb, stupid!

Because the database _should_ be dumb. Just safely store and retrieve my
data. And let me to run some adhoc queries over unindexed fields 😱,
that on other, wiser systems is strictly forbudt.

So, do **that** and do it well. And please, for the love of god,
make the WAL _impossible_ to turn off. Don't make me opt' it in
(_looking at SQLite_).

So I can't see how extending the database type system as added value.
It's not keeping it dumb. It's definitely not keeping it portable.

Unless, of course, you are building your own data store atop Postgres.
Then this all makes sense. Maybe those features and that opacity is
exactly [what pays the bills](https://www.timescale.com/).


## The Conclusion

Is this enough to stop me from using it? Hell no. Sometimes, you gotta
swallow something whole. Then you can judge how it really tastes.

I'm migrating apps to Postgres but for the small fish I'm using
SQLite. It statically links as a library. Yes, storage is a stupid
single file that no one other thread can write into but it's a bliss.
And the performance is as good as it gets for a relational storage.
