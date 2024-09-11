fn main() {
    let mut first_compare_str: &str = "hallo";
    let mut second_compare_str: &str = "hallo";

    if first_compare_str == second_compare_str {
        println!("match");
    } else {
        println!("NOT match");
    }

    first_compare_str = "hallo";
    second_compare_str = "ollah";

    if first_compare_str == second_compare_str {
        println!("match");
    } else {
        println!("NOT match");
    }

    first_compare_str = "ACTION_CLICK";
    second_compare_str = "ACTION_CLICK";

    if first_compare_str == second_compare_str {
        println!("match {}<=>{}", first_compare_str, second_compare_str);
    } else {
        println!("NOT match {}<=>{}", first_compare_str, second_compare_str);
    }

    const ACTION_CLICK: &str = "action_click";
    const ACTION_FORM_FILL_FIELD: &str = "action_form_fill_field";
    #[allow(dead_code)]
    const ACTION_SELECT_BUTTON: &str = "action_select_button";

    first_compare_str = ACTION_CLICK;
    second_compare_str = ACTION_CLICK;

    if first_compare_str == second_compare_str {
        println!("match {}<=>{}", first_compare_str, second_compare_str);
    } else {
        println!("NOT match {}<=>{}", first_compare_str, second_compare_str);
    }

    first_compare_str = ACTION_FORM_FILL_FIELD;
    second_compare_str = ACTION_FORM_FILL_FIELD;

    if first_compare_str == second_compare_str {
        println!("match {}<=>{}", first_compare_str, second_compare_str);
    } else {
        println!("NOT match {}<=>{}", first_compare_str, second_compare_str);
    }

    first_compare_str = ACTION_FORM_FILL_FIELD;
    second_compare_str = "false";

    if first_compare_str == second_compare_str {
        println!("match {}<=>{}", first_compare_str, second_compare_str);
    } else {
        println!("NOT match {}<=>{}", first_compare_str, second_compare_str);
    }
}
