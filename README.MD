# rsbrf
### rust book redirect fixer
Rust script for fixing reditect URLs in the old rust book editions.

The 2018 edition of the Book had all the links pointing at index.html. This script traverses edition files, checks if the same chapters exist in the new version of the book (the root `src` directory) and points reditect links at the updated pages.

## launching

```
cargo run -- ~/Downloads/book/first-edition/
rsbrf ~/Downloads/book/first-edition/
```
