<!-- tags: databases -->
<!-- hidden  -->

# Schemaless

Schemaless databases are in fashion for a reason. On relational
databases, out of the box schema changes sometimes involve locking and
copying an entire table from the old to the new structure.

When a table copy is required and it holds millions of rows that table
will be unavailable, sometimes for hours.


## Things

In theory, only 2 tables are needed to hold a schemaless schema:

- `item`, basically holds an `id` and a `type` and some metadata.
- `data`, holds the `field=value` for each for each of the items.
- `find`, holds indexed data (optional).

The `data` table hydrates `item`.


## Schema

Using sqlite3 as example:

```sql
DROP TABLE IF EXISTS "item";
DROP TABLE IF EXISTS "data";
CREATE TABLE "item" (
    "id"        INTEGER PRIMARY KEY AUTOINCREMENT,
    "type"      TEXT,
    "created"   TEXT DEFAULT CURRENT_TIMESTAMP NOT NULL,
    "updated"   TEXT,
    "status"    TEXT,
    "meta"      TEXT
);
CREATE TABLE "data" (
    "id"        INTEGER NOT NULL,
    "field"     TEXT NOT NULL,
    "value"     TEXT NOT NULL
);
CREATE UNIQUE INDEX "data_id_field" ON "data" ("id", "field");
```

Now, let's pump some data and review it:

```sql
INSERT INTO "item" ("type") VALUES ('post');
INSERT INTO "item" ("type") VALUES ('post');

INSERT INTO "data" VALUES (1, 'title', 'First post');
INSERT INTO "data" VALUES (2, 'title', 'Second post');
INSERT INTO "data" VALUES (2, 'content', 'Some stuff');

UPDATE "item" SET "status" = 'published' WHERE id = 2;

SELECT * FROM "item";
SELECT * FROM "data";
```

## Queries
Say you are looking for the 10 most recent posts.

```sql
SELECT
        d.id,
        d.field,
        d.value
FROM
        "item" i
JOIN
        "data" d
        ON d.id = i.id
WHERE
        i.type = "post"
        AND i.status = "published"
ORDER BY
        i.created DESC
LIMIT 10
;
```
