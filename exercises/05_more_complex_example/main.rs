////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

macro_rules! for_2d {
    ($row_ident:ident <$row_ty:ty> in $row_expr:expr, $col_ident:ident <$col_ty:ty> in $col_expr:expr, $loop_body:block) => {
        for $row_ident in $row_expr {
            let $row_ident: $row_ty = $row_ident;
            for $col_ident in $col_expr {
                let $col_ident: $col_ty = $col_ident;
                $loop_body
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
