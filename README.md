## Rust cli simplistic key-value database project

This cli is a key-value database *mono table* written in rust.


*3 operations are possible (for the moment) : *
  - INSERT : insert a key associated with a value in the database ( be careful, a key can only be in the database once or it'll be overwritten by the next same key ).
    usage: INSERT key value
  - REMOVE : remove a key and a value from the database using only the key.
    usage: REMOVE key
  - SELECT : you can either choose to select from a key existent or not or you can choose to use the star operator (*), for this one you will need to use a backslash behind it so the shell does not recognize it as his metacaracter.
    (usage: SELECT key) or (usage: SELECT \*)
  
  TODO: make a drop command so it clears the database. If time make the database multi tables --> multiples txt files.
