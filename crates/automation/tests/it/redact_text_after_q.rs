//! Search-and-redact of text that follows a `q`/`Q` inside a text object. Find's boxes come from
//! the renderer's text and redaction measures the content itself, so the two must agree on where
//! the text after `Q` is (vendored hayro-interpret patch 67); when they disagreed, `redact_apply`
//! refused to change anything.

use pdfcraft_automation::{Automation, Content};
use serde_json::{Value, json};

const CONTENT: &str = "BT /F1 12 Tf 72 700 Td (AAAA) Tj q /F2 24 Tf (B) Tj Q (SECRET) Tj ET";

fn pdf() -> Vec<u8> {
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_owned(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents 4 0 R /Resources << /Font << \
         /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> \
         /F2 << /Type /Font /Subtype /Type1 /BaseFont /Courier >> >> >> >>"
            .to_owned(),
        format!("<< /Length {} >>\nstream\n{CONTENT}\nendstream", CONTENT.len()),
    ];
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{object}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes());
    for offset in offsets {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer << /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objects.len() + 1).as_bytes());
    out
}

fn ok(a: &mut Automation, tool: &str, args: Value) -> Value {
    match a.call(tool, &args) {
        Ok(mut c) => match c.remove(0) {
            Content::Json(v) => v,
            other => panic!("{tool}: expected JSON, got {other:?}"),
        },
        Err(e) => panic!("{tool} {args}: {e}"),
    }
}

fn text(a: &mut Automation, doc: &Value) -> String {
    ok(a, "text_extract", json!({ "doc": doc }))["pages"][0]["text"].as_str().unwrap_or_default().to_owned()
}

#[test]
fn find_and_redact_removes_text_after_a_q_q_inside_its_text_object() {
    let dir = std::env::temp_dir().join(format!("pdfcraft-redact-after-q-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("in.pdf"), pdf()).unwrap();
    let mut a = Automation::new().with_root(&dir).unwrap();
    let doc = ok(&mut a, "doc_open", json!({ "path": "in.pdf" }))["doc"].clone();
    assert_eq!(ok(&mut a, "redact_mark", json!({ "doc": doc, "find": "SECRET" }))["marked"], 1);
    ok(&mut a, "redact_apply", json!({ "doc": doc }));
    ok(&mut a, "doc_save", json!({ "doc": doc, "path": "out.pdf", "full": true }));
    let saved = ok(&mut a, "doc_open", json!({ "path": "out.pdf" }))["doc"].clone();
    let after = text(&mut a, &saved);
    assert!(!after.contains("SECRET"), "{after:?}");
    assert!(after.contains("AAAAB"), "the text before the mark stays: {after:?}");
    let _ = std::fs::remove_dir_all(&dir);
}
