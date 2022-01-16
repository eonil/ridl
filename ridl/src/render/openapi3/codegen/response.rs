use crate::model::*;
use super::oa;

use crate::prelude::*;
use super::*;

/// Renders OAS Response object if appropriate.
fn render_item(item:&KItem) -> Result<Option<oa::Response>> {
    let sum_type = if let KItem::Sum(sum_type) = item { sum_type } else { return Ok(None) };
    if sum_type.attrs.rest.contains(&KAttrREST::MessageOut) {} else { return Ok(None) }

    let mut oa_content_map = Map::<oa::MIMEType, oa::MediaType>::new();
    for variant in sum_type.variants.iter() {
        let oa_mime = if let Some(mime) = mime(variant) { mime } else { return err(variant.span, "missing MIME-Type attribute") };
        let mut oa_media_type = oa::MediaType::default();
        oa_media_type.schema = Some(variant.content.render(variant.span)?);
        oa_content_map.insert(oa_mime.to_string(), oa_media_type);
    }

    let mut oa_response = oa::Response::default();
    oa_response.content = Some(oa_content_map);
    Ok(Some(oa_response))
}

fn status(x:&KSumTypeVariant) -> Option<&i64> {
    for a in x.attrs.rest.iter() {
        if let KAttrREST::Status(z) = a { return Some(z) }
    }
    return None
}
fn mime(x:&KSumTypeVariant) -> Option<&str> {
    for a in x.attrs.rest.iter() {
        if let KAttrREST::MIME(z) = a { return Some(z) }
    }
    return None
}