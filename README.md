# About

alpha-counter is a library providing the [`AlphaCounter`] iterator representing
an alphabetic counter as would be used for numbering appendices.

It provides convenience methods to create [`upper`][AlphaCounter::upper] and
[`lower`][AlphaCounter::lower] ASCII alphabets as well as any alphabet via the
[`custom`][AlphaCounter::custom] method.
All three constructors enable starting the iterator at any point; although one
could also manipulate it via regular Iterator methods.

# Example

```
use alpha_counter::AlphaCounter;

assert_eq!(
    AlphaCounter::lower(0).take(30).collect::<Vec<String>>(),
    vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n",
        "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "aa", "ab",
        "ac", "ad",
    ],
);

assert_eq!(
    AlphaCounter::upper(0).take(30).collect::<Vec<String>>(),
    vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N",
        "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "AA", "AB",
        "AC", "AD",
    ],
);

assert_eq!(
    AlphaCounter::custom(0, "abc")
        .take(30)
        .collect::<Vec<String>>(),
    vec![
        "a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc",
        "aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa",
        "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc",
    ],
);
```

