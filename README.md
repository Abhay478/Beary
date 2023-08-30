# Beary

It's a simple CLI that does some things that Bear standard (not pro) doesn't do.

- Export notes to pdf
- Bundle multiple notes into one pdf, provided you've kept up the right tag structure. it's a bit of a pain, and I'll probably streamline it with(?) backward compatibility in the future.
- More features soon, I hope.

## The indexing scheme

- Tree-like organisation, where a book-level index file, named after the title of the book (or really, the name of this index note is the name of the book) contains links to multiple part-level index notes, which contain links to leaf-level chapter notes. 
- If a chapter is not a part of a part (heh), that's also fine. It will be treated as a 'part' with one chapter.
- Every index must be tagged as such. The last tag in the note should be **#"something"Index** (`ends_with()` method used.).
- I will put up a pictorial representation of this structure soon.

### Note: Do **not** try and write to the Bear database, they have this stringent naming convention and delicate relations between tables. If you want to, back the database up first. This CLI only adds the `diesel_schema` table necessary to operate with the ORM, nothing that ships with Bear is touched.

### Note 2: This only works on macOS, as I could not find where the database is supposed to be on Linux or Windows. If someone knows, please raise an issue.