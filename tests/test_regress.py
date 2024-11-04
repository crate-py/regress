import pytest

import regress


def test_string_contains_match():
    """
    https://docs.rs/regress/latest/regress/#example-test-if-a-string-contains-a-match
    """
    regex = regress.Regex(r"\d{4}")
    assert regex.find("2020-20-05") is not None


def test_iterating_over_matches():
    """
    https://docs.rs/regress/latest/regress/#example-iterating-over-matches
    """

    regex = regress.Regex(r"(\w)\1")
    text = "Frankly, Miss Piggy, I don't give a hoot!"
    result = [text[m.range()] for m in regex.find_iter(text)]
    assert result == ["ss", "gg", "oo"]


def test_capture_groups():
    """
    https://docs.rs/regress/latest/regress/#example-using-capture-groups
    """

    regex = regress.Regex(r"(\d{4})")
    text = "Today is 2020-20-05"

    match = regex.find(text)
    assert match is not None

    group = match.group(1)
    assert group

    assert text[group] == "2020"


def test_named_group():
    regex = regress.Regex(r"(?<animal>fox|dog)")

    text = "The quick brown fox jumped over the lazy dog's back"

    match = regex.find(text)
    assert match is not None

    group = match.named_group("animal")
    assert group

    assert text[group] == "fox"


def test_error_handling():
    try:
        regress.Regex(r"(")
    except regress.RegressError:
        pass
    else:
        pytest.fail("error not reached")


def test_with_flags():
    # "L" for letters, "Z" for spaces, "N" for numerics
    pattern = r"^\p{L}\p{Z}\p{N}$"

    regex = regress.Regex(pattern, flags="u")
    flagless_regex = regress.Regex(pattern)

    match = regex.find("a 0")
    flagless_match = flagless_regex.find("a 0")
    assert match is not None
    assert flagless_match is None
