use super::super::Int;
use super::super::black::Black;
use super::super::iso_max::IsoMax;
use super::super::iso_min::IsoMin;
use super::super::white::White;
use super::Bounds;
use super::Sensor;
use super::iso_list;
use super::iso_list::IsoValues;
use super::xmlparser;

type T<'a> = Sensor;

#[expect(non_snake_case)]
fn Err(str: &'static str) -> Result<T<'static>, String> {
    Result::Err(str.to_owned())
}

#[test]
#[expect(clippy::too_many_lines)]
fn parse_test() {
    let inputs: Vec<&'static str> = vec![
        "",
        "<",
        "<Sensor",
        "<Sensor ",
        "<NotSensor ",
        "<Sensor black",
        "<Sensor black ",
        "<Sensor not_black ",
        "<Sensor black=",
        "<Sensor black=Foo",
        "<Sensor black=\"Foo\"",
        "<Sensor black=\"42\"",
        "<Sensor black=\"42\"white",
        "<Sensor black=\"42\" white ",
        "<Sensor black=\"42\" not_white ",
        "<Sensor black=\"42\" white=",
        "<Sensor black=\"42\" white=Bar",
        "<Sensor black=\"42\" white=\"Bar\"",
        "<Sensor black=\"42\" white=\"24\"",
        "<Sensor black=\"42\" white=\"24\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"100\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_min=\"12\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_max=\"12\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_max=\"25\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_min=\"12\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_min=\"12\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_max=\"25\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_min=\"12\" iso_max=\"25\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_max=\"25\" iso_min=\"12\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_list=\"1 2\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_list=\"1 2\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_max=\"25\" iso_list=\"1 2\"/>",
        "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_min=\"12\" iso_list=\"1 2\"/>",
    ];
    let expected: Vec<(&str, xmlparser::Result<T<'_>>)> = vec![
        (
            "",
            Err("While trying to match `\"Lt\"`, encountered end of stream"),
        ),
        (
            "<",
            Err(
                "While trying to match `\"ElementName\"`, encountered end of stream",
            ),
        ),
        (
            "<Sensor",
            Err(
                "While trying to match `\"ElementName\"`, but the following was encountered instead: `Garbage(\"Sensor\")`",
            ),
        ),
        (
            "<Sensor ",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<NotSensor ",
            Err(
                "Error while parsing element, expected `\"Sensor\"`, but instead found: `\"NotSensor\"`",
            ),
        ),
        (
            "<Sensor black",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"black\")`",
            ),
        ),
        (
            "<Sensor black ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Sensor not_black ",
            Err(
                "Error while parsing attribute, expected `\"black\"`, but instead found: `\"not_black\"`",
            ),
        ),
        (
            "<Sensor black=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Sensor black=Foo",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Foo\")`",
            ),
        ),
        (
            "<Sensor black=\"Foo\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Foo\" }` as an integer",
            ),
        ),
        (
            "<Sensor black=\"42\"",
            Err(
                "While trying to match `\"ElementAttributeName\"`, encountered end of stream",
            ),
        ),
        (
            "<Sensor black=\"42\"white",
            Err(
                "While trying to match `\"ElementAttributeName\"`, but the following was encountered instead: `Garbage(\"white\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white ",
            Err(
                "While trying to match `\"ElementAttributeEq\"`, but the following was encountered instead: `Garbage(\" \")`",
            ),
        ),
        (
            "<Sensor black=\"42\" not_white ",
            Err(
                "Error while parsing attribute, expected `\"white\"`, but instead found: `\"not_white\"`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, encountered end of stream",
            ),
        ),
        (
            "<Sensor black=\"42\" white=Bar",
            Err(
                "While trying to match `\"ElementAttributeValue\"`, but the following was encountered instead: `Garbage(\"Bar\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"Bar\"",
            Err(
                "Unable to parse `ElementAttributeValue { buf: \"Bar\" }` as an integer",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\"",
            Err(
                "While trying to match `\"ElementSlash\"`, encountered end of stream",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::Unbounded,
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"100\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::Enumerated(iso_list::IsoList {
                    val: IsoValues { values: vec![100] },
                }),
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::Enumerated(iso_list::IsoList {
                    val: IsoValues { values: vec![1, 2] },
                }),
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_min=\"12\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::LowerBounded(IsoMin {
                    val: Int { val: 12 },
                }),
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_max=\"12\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::UpperBounded(IsoMax {
                    val: Int { val: 12 },
                }),
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_max=\"25\"/>",
            Ok(Sensor {
                black: Black {
                    val: Int { val: 42 },
                },
                white: White {
                    val: Int { val: 24 },
                },
                bounds: Bounds::Range((
                    IsoMin {
                        val: Int { val: 12 },
                    },
                    IsoMax {
                        val: Int { val: 25 },
                    },
                )),
            }),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_min=\"12\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_min\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_min=\"12\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_min\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_max=\"25\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_max\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_min=\"12\" iso_max=\"25\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_min\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_list=\"1 2\" iso_max=\"25\" iso_min=\"12\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_max\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_list=\"1 2\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_list\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_list=\"1 2\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_list\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_min=\"12\" iso_max=\"25\" iso_list=\"1 2\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_list\")`",
            ),
        ),
        (
            "<Sensor black=\"42\" white=\"24\" iso_max=\"25\" iso_min=\"12\" iso_list=\"1 2\"/>",
            Err(
                "While trying to match `\"ElementSlash\"`, but the following was encountered instead: `ElementAttributeName(\"iso_min\")`",
            ),
        ),
    ];
    let mut results = vec![];
    for input in inputs {
        results.push((input, xmlparser::parse_str::<T<'_>>(input)));
    }
    assert_eq!(results, expected);
}
