extern crate nested_list;
extern crate nested_list_macro;

use nested_list::NestedList;

use nested_list_macro::nested_list;

fn main() {
    let list = nested_list! {
        [1, 2, 3, [4, 5], 6, [7, 8, [9, 0, 1], 2], 8]
    };

    println!("{:?}", list);

    let list = list.flatten();

    println!("{:?}", list);
}
