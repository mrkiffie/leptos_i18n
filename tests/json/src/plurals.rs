use crate::i18n::*;
use common::*;

#[test]
fn cardinal_plural() {
    // count = 0
    let count = move || 0;
    let en = td!(Locale::en, cardinal_plural, $ = count);
    assert_eq_rendered!(en, "0 items");
    let fr = td!(Locale::fr, cardinal_plural, $ = count);
    assert_eq_rendered!(fr, "0");

    // count = 1
    let count = move || 1;
    let en = td!(Locale::en, cardinal_plural, $ = count);
    assert_eq_rendered!(en, "one item");
    let fr = td!(Locale::fr, cardinal_plural, $ = count);
    assert_eq_rendered!(fr, "1");

    // count = 2..
    for i in [2, 5, 10, 1000] {
        let count = move || i;
        let en = td!(Locale::en, cardinal_plural, $ = count);
        assert_eq_rendered!(en, format!("{} items", i));
        let fr = td!(Locale::fr, cardinal_plural, $ = count);
        assert_eq_rendered!(fr, i.to_string());
    }
}

#[test]
fn ordinal_plural() {
    // count = 1
    let count = move || 1;
    let en = td!(Locale::en, ordinal_plural, $ = count);
    assert_eq_rendered!(en, "1st place");
    let fr = td!(Locale::fr, ordinal_plural, $ = count);
    assert_eq_rendered!(fr, "1re place");

    // count = 2
    let count = move || 2;
    let en = td!(Locale::en, ordinal_plural, $ = count);
    assert_eq_rendered!(en, "2nd place");
    let fr = td!(Locale::fr, ordinal_plural, $ = count);
    assert_eq_rendered!(fr, "2e place");

    // count = 3
    let count = move || 3;
    let en = td!(Locale::en, ordinal_plural, $ = count);
    assert_eq_rendered!(en, "3rd place");
    let fr = td!(Locale::fr, ordinal_plural, $ = count);
    assert_eq_rendered!(fr, "3e place");

    // count = 4
    let count = move || 4;
    let en = td!(Locale::en, ordinal_plural, $ = count);
    assert_eq_rendered!(en, "4th place");
    let fr = td!(Locale::fr, ordinal_plural, $ = count);
    assert_eq_rendered!(fr, "4e place");
}
