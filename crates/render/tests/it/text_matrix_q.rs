//! Contributor-original synthetic pages: `q`/`Q` inside a text object. The text matrices are not
//! part of the graphics state (ISO 32000-2 §8.4.1 Table 51, §9.4.1), so a `Q` restores the font
//! but not the pen (vendored hayro-interpret patch 67). Each case is paired with a page that
//! changes the font back by hand instead, which must come out the same.
use pdfcraft_render::{PageRenderer, PageText, RenderConfig, RenderRequest, RequestKind};
use std::sync::Arc;

/// `q`/`Q` around a larger font: the text after `Q` continues from the end of the "B".
const SAVED: &str = "BT /F1 12 Tf 72 300 Td (AAAA) Tj q /F2 24 Tf (B) Tj Q (SECRET) Tj ET";
/// The same text with the font set back by `Tf`.
const BY_HAND: &str = "BT /F1 12 Tf 72 300 Td (AAAA) Tj /F2 24 Tf (B) Tj /F1 12 Tf (SECRET) Tj ET";
/// A move inside `q`/`Q` moves the pen for good: `Td` is not undone by `Q`.
const MOVED_IN_SAVE: &str = "BT /F1 12 Tf 72 300 Td (AA) Tj q 100 0 Td Q (SECRET) Tj ET";
const MOVED: &str = "BT /F1 12 Tf 72 300 Td (AA) Tj 100 0 Td (SECRET) Tj ET";
/// A form that moves its own pen and ends with an unbalanced `Q`, shown inside a text object
/// (`Do` is not allowed there, but readers draw it).
const FORM_INSIDE: &str = "BT /F1 12 Tf 72 300 Td (AA) Tj /X Do (SECRET) Tj ET";
const FORM_BODY: &[u8] = b"BT /F1 12 Tf 200 0 Td ET Q";
const NO_FORM: &str = "BT /F1 12 Tf 72 300 Td (AA) Tj (SECRET) Tj ET";

/// One page per entry of `contents`, 400 × 400, with Helvetica as /F1, Courier as /F2 and the
/// form /X.
fn pdf(contents: &[&str]) -> Arc<Vec<u8>> {
    let n = contents.len();
    let kids: String = (0..n).map(|i| format!("{} 0 R ", 5 + 2 * i)).collect();
    let mut objects: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [{kids}] /Count {n} >>").into_bytes(),
        b"<< /Font << /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> /F2 << /Type /Font /Subtype /Type1 /BaseFont /Courier >> >> /XObject << /X 4 0 R >> >>".to_vec(),
        stream(b"/Type /XObject /Subtype /Form /BBox [0 0 400 400] /Resources 3 0 R", FORM_BODY),
    ];
    for (i, content) in contents.iter().enumerate() {
        objects.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 400 400] /Resources 3 0 R /Contents {} 0 R >>", 6 + 2 * i).into_bytes());
        objects.push(stream(b"", content.as_bytes()));
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(object);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes());
    for offset in offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer << /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objects.len() + 1).as_bytes());
    Arc::new(out)
}

fn stream(dict: &[u8], body: &[u8]) -> Vec<u8> {
    let mut s = b"<< ".to_vec();
    s.extend_from_slice(dict);
    s.extend_from_slice(format!(" /Length {} >>\nstream\n", body.len()).as_bytes());
    s.extend_from_slice(body);
    s.extend_from_slice(b"\nendstream");
    s
}

fn request(page: usize, kind: RequestKind) -> RenderRequest {
    RenderRequest { page, kind, tile: None, scale: 1.0, tag: 0, preview: false }
}

fn texts(contents: &[&str]) -> Vec<Arc<PageText>> {
    let mut renderer = PageRenderer::new(pdf(contents), RenderConfig::default());
    (0..contents.len())
        .map(|page| {
            let out = renderer.render(request(page, RequestKind::Text));
            assert!(out.error.is_none(), "page {} text error: {:?}", page + 1, out.error);
            out.text.expect("page text")
        })
        .collect()
}

/// The boxes of the glyphs of `word`'s one match on the page.
fn boxes(text: &PageText, word: &str) -> Vec<[f32; 4]> {
    let hits = text.find(word);
    assert_eq!(hits.len(), 1, "one {word} in {:?}", text.plain_text());
    hits[0].clone().map(|i| text.glyphs[i].rect).collect()
}

fn assert_same_boxes(a: &[[f32; 4]], b: &[[f32; 4]]) {
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b) {
        assert!(x.iter().zip(y).all(|(p, q)| (p - q).abs() < 0.01), "{x:?} != {y:?}");
    }
}

#[test]
fn text_after_q_q_continues_where_the_text_inside_left_off() {
    let pages = texts(&[SAVED, BY_HAND]);
    let secret = boxes(&pages[0], "SECRET");
    assert_same_boxes(&secret, &boxes(&pages[1], "SECRET"));
    let b = boxes(&pages[0], "B");
    assert!(secret[0][0] >= b[0][2] - 0.01, "SECRET starts at {} inside the B ending at {}", secret[0][0], b[0][2]);
}

#[test]
fn q_q_still_restores_the_font() {
    let pages = texts(&[SAVED, BY_HAND]);
    let height = |r: [f32; 4]| r[3] - r[1];
    let secret = boxes(&pages[0], "SECRET");
    let b = boxes(&pages[0], "B");
    assert!(height(secret[0]) < height(b[0]) * 0.75, "SECRET is drawn at the 12 pt font, not the B's 24 pt");
}

#[test]
fn a_move_inside_q_q_is_kept() {
    let pages = texts(&[MOVED_IN_SAVE, MOVED]);
    assert_same_boxes(&boxes(&pages[0], "SECRET"), &boxes(&pages[1], "SECRET"));
}

#[test]
fn a_forms_unbalanced_q_does_not_move_the_page_text() {
    let pages = texts(&[FORM_INSIDE, NO_FORM]);
    assert_same_boxes(&boxes(&pages[0], "SECRET"), &boxes(&pages[1], "SECRET"));
}

#[test]
fn q_q_inside_a_text_object_draws_like_the_font_set_back_by_hand() {
    let mut renderer = PageRenderer::new(pdf(&[SAVED, BY_HAND]), RenderConfig::default());
    let saved = renderer.render(request(0, RequestKind::Pixels));
    let by_hand = renderer.render(request(1, RequestKind::Pixels));
    assert!(saved.error.is_none() && by_hand.error.is_none());
    assert_eq!((saved.width, saved.height), (by_hand.width, by_hand.height));
    assert!(saved.rgba == by_hand.rgba, "the two pages draw differently");
}
