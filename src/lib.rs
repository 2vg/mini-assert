#[macro_export]
macro_rules! assert_equal {
    ($left:expr, $right:expr) => ({
        $crate::assert_equal!($left, $right, "")
    });
    ($left:expr, $right:expr,) => ({
        $crate::assert_equal!($left, $right, "")
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        use console::style;
        use difference::{Difference, Changeset};

        match (&$left, &$right) {
            (left_val, right_val) => {
                let assert_name = format!($($arg)+);

                if !(*left_val == *right_val) {
                    let Changeset { diffs, .. } = Changeset::new(&format!("{:#?}", *left_val), &format!("{:#?}", *right_val), "");

                    let mut expect = String::from("");
                    for i in 0 .. diffs.len() {
                        match diffs[i] {
                            Difference::Same(ref x) => {
                                expect += x;
                            }
                            Difference::Rem(ref x) => {
                                expect += &style(format!("{}", x)).green().to_string();
                            }
                            _ => {}
                        }
                    }

                    let mut actual = String::from("");
                    for i in 0 .. diffs.len() {
                        match diffs[i] {
                            Difference::Same(ref x) => {
                                actual += x;
                            }
                            Difference::Add(ref x) => {
                                actual += &style(format!("{}", x)).red().to_string();
                            }
                            _ => {}
                        }
                    }

                    print!("\n{}", style("assertion failed").cyan().bold());
                    let assert_name_with_style = style(format!(": {}", &assert_name)).white().bold();
                    println!("{}\n", assert_name_with_style);

                    println!("{}:", style("expect").green().bold());
                    for e in expect.split("\n") {
                        println!("        {}", e);
                    }
                    println!("\n{}:", style("actual").red().bold());
                    for a in actual.split("\n") {
                        println!("        {}", a);
                    }
                    println!("");
                    panic!();
                }
            }
        }
    });
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr) => ({
        $crate::assert_equal!($left, $right)
    });
    ($left:expr, $right:expr,) => ({
        $crate::assert_equal!($left, $right)
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        $crate::assert_equal!($left, $right, $($arg)+)
    });
}
