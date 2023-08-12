<!-- tags: databases -->
<!-- hidden: true -->

# Schemaless

<!-- START TAGS -->
[<img src="https://img.shields.io/badge/Tag-databases-brightgreen">](/tags/databases)
<!-- END TAGS -->

Schemaless databases are in fashion for a reason. On relational
databases, out of the box schema changes sometimes involve locking and
copying an entire table from the old to the new structure.

When a table copy is required and it holds millions of rows that table
will be unavailable, sometimes for hours.


## Things

In theory, only 2 tables are needed to hold a schemaless schema:

- `item`, each item in the database has an `id` and a `type`.
- `data`, holds the `field=value` for each for each of the item's `id`.
- `find`, holds full text index data.

Say you are looking for the 10 most recent posts.

```sql
SELECT
        i.id,
        d.field,
        d.value
FROM
        item i
JOIN
        data d
        ON d.id = i.id
WHERE
        i.type = "post"
        AND i.status = "published"
ORDER BY
        i.created DESC
LIMIT 10
```
