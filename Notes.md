## Testing Readme via rustdoc
 
rustdoc --test README.md



## Look at test_statics.rs

* ergomic filters
* Rust doesn't allow to override the use of ==, !=, >=, <=, which could otherwise be use to have an ergomic expression in filtering
* Solution
    * Use macro to convert
        filter!("name" >= "11") into Filter{"name", GTE, 11}

    * Global variable to store the filters called from the override