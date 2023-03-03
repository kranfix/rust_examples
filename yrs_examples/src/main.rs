use std::collections::HashMap;

use yrs::types::text::Diff;

fn main() {
    // sticky_index_example();
    //text_diff();
    text_test();
}

fn text_test() {
    use lib0::any::Any;
    use yrs::{Assoc, GetString, IndexedSequence};
    use yrs::{Doc, Text, Transact};

    let doc = Doc::new();
    let txt = doc.get_or_insert_text(""); // Only one text field per Property right now
    let mut txn = doc.transact_mut();
    txt.insert(&mut txn, 0, "the  jumped");
    assert_eq!(txt.get_string(&txn), "the  jumped");

    let pos = txt.sticky_index(&mut txn, 5, Assoc::Before).unwrap();

    // insert [dog] at 4
    let lozenge: HashMap<String, Any> = HashMap::from([(
        "edge_id".to_string(),
        "fasldkmfasldkmfasldkfalskdjfasdf".into(),
    )]);

    txt.insert_embed(&mut txn, 4, Any::Map(Box::new(lozenge.clone())));

    let a = pos.get_offset(&txn).unwrap();
    assert_eq!(a.index, 6);

    let chunks = txt.diff(&txn, yrs::types::text::YChange::identity);
    println!("{chunks:?}");

    assert_eq!(
        chunks,
        [
            Diff::new("the ".into(), None),
            Diff::new(lozenge.into(), None),
            Diff::new(" jumped".into(), None)
        ]
    );

    // [
    //     Diff { insert: Any(String("the ")), attributes: None, ychange: None },
    //     Diff { insert: Any(Map({"lz": String("edge_id_for_this_lozengefasldkmfasldkmfasldkfalskdjfasdf")})), attributes: None, ychange: None },
    //     Diff { insert: Any(String(" jumped")), attributes: None, ychange: None }
    // ]

    // Interleaving situation:
    // <i>the <b>dog</i> jumped</b>
    // Chunks:
    // "the " Some([i])
    // "dog" Some([b,i])
    // " jumped" Some([b])

    // // txt.format(&mut txn, 0, 5, bold.clone());
    // let a = pos.get_offset(&txn).unwrap();
    // assert_eq!(a.index, 9);

    // assert_eq!(txt.get_string(&txn), "<b>hello</b> <i>world</i>");
    // let a = pos.get_offset(&txn).unwrap();
    // assert_eq!(a.index, 9);

    // // remove formatting
    // let remove_italic = Attrs::from([("i".into(), Any::Null)]);
    // txt.format(&mut txn, 6, 5, remove_italic);

    // assert_eq!(txt.get_string(&txn), "<b>hello</b> world");
    // let a = pos.get_offset(&txn).unwrap();
    // assert_eq!(a.index, 9);

    // // insert binary payload eg. images
    // let image = b"deadbeaf".to_vec();
    // txt.insert_embed(&mut txn, 1, image);
    // let a = pos.get_offset(&txn).unwrap();
    // assert_eq!(a.index, 10);
    // assert_eq!(
    //     txt.get_string(&txn),
    //     "<b>h</b><b>0x6465616462656166</b><b>ello</b> world"
    // );

    // let lozenge_content = "Some text";
    // let lozenge_tag = {
    //     let map: HashMap<String, Any> = HashMap::from([("ref".into(), "value".into())]);
    //     let props: Any = map.into();
    //     Attrs::from([("Lozenge".into(), props)])
    // };
    // txt.insert_embed_with_attributes(&mut txn, 6, lozenge_content, lozenge_tag.clone());

    // let a = pos.get_offset(&txn).unwrap();
    // assert_eq!(a.index, 11);
    // assert_eq!(
    //     txt.get_string(&txn),
    //     r#"<b>h</b><b>0x6465616462656166</b><b>ello</b><Lozenge ref="value">Some text</Lozenge> world"#
    // );

    // let chunks = txt.diff(&txn, yrs::types::text::YChange::identity);
    // assert_eq!(
    //     chunks,
    //     [
    //         Diff::new("h".into(), Some(Box::new(bold.clone()))),
    //         Diff::new(
    //             Any::Buffer([100, 101, 97, 100, 98, 101, 97, 102].into()).into(),
    //             Some(Box::new(bold.clone()))
    //         ),
    //         Diff::new(Any::String("ello".into()).into(), Some(Box::new(bold))),
    //         Diff::new(
    //             Any::String("Some text".into()).into(),
    //             Some(Box::new(lozenge_tag))
    //         ),
    //         Diff::new(Any::String(" world".into()).into(), None)
    //     ]
    // );
    // let are_lozenge: Vec<bool> = chunks.iter().map(is_lozenge).collect();
    // assert_eq!(are_lozenge, [false, false, false, true, false]);
}

// fn is_lozenge<T>(diff: &Diff<T>) -> bool {
//     match &diff.attributes {
//         Some(attr) => attr.get("Lozenge").is_some(),
//         None => false,
//     }
// }

// fn is_text_embed<T>(diff: &Diff<T>) -> bool {
//     match &diff.attributes {
//         Some(attr) => attr.get("text-embed").is_some(),
//         None => false,
//     }
// }

// fn insert_text_embed<V>(xml: &XmlTextRef, txn: &mut TransactionMut) -> V::Return
// where
//     V: Into<EmbedPrelim<V>> + Prelim,
// {
// }

pub fn sticky_index_example() {
    use yrs::{Assoc, Doc, GetString, IndexedSequence, Text, Transact};

    let doc = Doc::new();
    let txt = doc.get_or_insert_text("text");
    let mut txn = doc.transact_mut();
    txt.insert(&mut txn, 0, "abcdef");

    // create position tracker (marked as . in the comments)
    let start_pos = txt.sticky_index(&mut txn, 2, Assoc::After).unwrap();
    let text = txt.get_string(&txn);
    println!("added start: {text}");

    let end_pos = txt.sticky_index(&mut txn, 4, Assoc::After).unwrap();
    let text = txt.get_string(&txn);
    println!("added end: {text}");

    // modify text
    txt.insert(&mut txn, 3, "mnp"); // => 'adefb.c'
    {
        let a = start_pos.get_offset(&txn).unwrap();
        assert_eq!(a.index, 2);
        let b = end_pos.get_offset(&txn).unwrap();
        assert_eq!(b.index, 7);
        assert_eq!(a.branch, b.branch)
    }

    txt.remove_range(&mut txn, 1, 1); // => 'adef.c'
    {
        let a = start_pos.get_offset(&txn).unwrap();
        assert_eq!(a.index, 1);
        let b = end_pos.get_offset(&txn).unwrap();
        assert_eq!(b.index, 6);
    }

    // get current offset index within the containing collection

    let text = txt.get_string(&txn);
    println!("{text}");
}
