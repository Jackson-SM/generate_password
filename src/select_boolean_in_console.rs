pub fn select_boolean_in_console(option: String) -> bool {
    if option.trim() == "N" || option.trim() == "n" {
        return false;
    }

    return true;
}
