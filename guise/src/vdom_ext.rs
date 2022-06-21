use crate::vdom::ElementBuilder;
use arwa::event::EventTarget;
use arwa::html::*;

macro_rules! known_element_fn {
    ($fn_name:ident, $element:ident) => {
        fn $fn_name<F>(&mut self, f: F)
        where
            F: FnOnce(ElementBuilder<$element>),
        {
            ChildKnownElementExt::child_known_element::<$element, F>(self, f)
        }
    };
}
pub(crate) mod child_known_element_ext_seal {
    pub trait Seal {}
}

pub trait ChildKnownElementExt: child_known_element_ext_seal::Seal {
    fn child_known_element<T, F>(&mut self, f: F)
    where
        T: KnownElement + EventTarget,
        F: FnOnce(ElementBuilder<T>);

    known_element_fn!(child_base, HtmlBaseElement);
    known_element_fn!(child_head, HtmlHeadElement);
    known_element_fn!(child_link, HtmlLinkElement);
    known_element_fn!(child_meta, HtmlMetaElement);
    known_element_fn!(child_style, HtmlStyleElement);
    known_element_fn!(child_title, HtmlTitleElement);
    known_element_fn!(child_body, HtmlBodyElement);
    known_element_fn!(child_address, HtmlAddressElement);
    known_element_fn!(child_article, HtmlArticleElement);
    known_element_fn!(child_aside, HtmlAsideElement);
    known_element_fn!(child_footer, HtmlFooterElement);
    known_element_fn!(child_header, HtmlHeaderElement);
    known_element_fn!(child_h1, HtmlH1Element);
    known_element_fn!(child_h2, HtmlH2Element);
    known_element_fn!(child_h3, HtmlH3Element);
    known_element_fn!(child_h4, HtmlH4Element);
    known_element_fn!(child_h5, HtmlH5Element);
    known_element_fn!(child_h6, HtmlH6Element);
    known_element_fn!(child_main, HtmlMainElement);
    known_element_fn!(child_nav, HtmlNavElement);
    known_element_fn!(child_section, HtmlSectionElement);
    known_element_fn!(child_blockquote, HtmlBlockquoteElement);
    known_element_fn!(child_dd, HtmlDdElement);
    known_element_fn!(child_div, HtmlDivElement);
    known_element_fn!(child_dl, HtmlDlElement);
    known_element_fn!(child_dt, HtmlDtElement);
    known_element_fn!(child_figcaption, HtmlFigcaptionElement);
    known_element_fn!(child_hr, HtmlHrElement);
    known_element_fn!(child_li, HtmlLiElement);
    known_element_fn!(child_menu, HtmlMenuElement);
    known_element_fn!(child_ol, HtmlOlElement);
    known_element_fn!(child_p, HtmlPElement);
    known_element_fn!(child_pre, HtmlPreElement);
    known_element_fn!(child_ul, HtmlUlElement);
    known_element_fn!(child_a, HtmlAElement);
    known_element_fn!(child_abbr, HtmlAbbrElement);
    known_element_fn!(child_b, HtmlBElement);
    known_element_fn!(child_bdi, HtmlBdiElement);
    known_element_fn!(child_bdo, HtmlBdoElement);
    known_element_fn!(child_br, HtmlBrElement);
    known_element_fn!(child_cite, HtmlCiteElement);
    known_element_fn!(child_code, HtmlCodeElement);
    known_element_fn!(child_data, HtmlDataElement);
    known_element_fn!(child_dfn, HtmlDfnElement);
    known_element_fn!(child_em, HtmlEmElement);
    known_element_fn!(child_i, HtmlIElement);
    known_element_fn!(child_kbd, HtmlKbdElement);
    known_element_fn!(child_mark, HtmlMarkElement);
    known_element_fn!(child_q, HtmlQElement);
    known_element_fn!(child_rp, HtmlRpElement);
    known_element_fn!(child_rt, HtmlRtElement);
    known_element_fn!(child_ruby, HtmlRubyElement);
    known_element_fn!(child_s, HtmlSElement);
    known_element_fn!(child_samp, HtmlSampElement);
    known_element_fn!(child_small, HtmlSmallElement);
    known_element_fn!(child_span, HtmlSpanElement);
    known_element_fn!(child_strong, HtmlStrongElement);
    known_element_fn!(child_sub, HtmlSubElement);
    known_element_fn!(child_time, HtmlTimeElement);
    known_element_fn!(child_u, HtmlUElement);
    known_element_fn!(child_var, HtmlVarElement);
    known_element_fn!(child_wbr, HtmlWbrElement);
    known_element_fn!(child_area, HtmlAreaElement);
    known_element_fn!(child_audio, HtmlAudioElement);
    known_element_fn!(child_img, HtmlImgElement);
    known_element_fn!(child_map, HtmlMapElement);
    known_element_fn!(child_track, HtmlTrackElement);
    known_element_fn!(child_video, HtmlVideoElement);
    known_element_fn!(child_embed, HtmlEmbedElement);
    known_element_fn!(child_iframe, HtmlIframeElement);
    known_element_fn!(child_object, HtmlObjectElement);
    known_element_fn!(child_picture, HtmlPictureElement);
    known_element_fn!(child_source, HtmlSourceElement);
    known_element_fn!(child_del, HtmlDelElement);
    known_element_fn!(child_ins, HtmlInsElement);
    known_element_fn!(child_caption, HtmlCaptionElement);
    // TODO (missing in Arwa): known_element_fn!(child_col, HtmlColElement);
    known_element_fn!(child_colgroup, HtmlColgroupElement);
    known_element_fn!(child_table, HtmlTableElement);
    known_element_fn!(child_tbody, HtmlTbodyElement);
    known_element_fn!(child_td, HtmlTdElement);
    known_element_fn!(child_tfoot, HtmlTfootElement);
    known_element_fn!(child_th, HtmlThElement);
    known_element_fn!(child_thead, HtmlTheadElement);
    known_element_fn!(child_tr, HtmlTrElement);
    known_element_fn!(child_button, HtmlButtonElement);
    known_element_fn!(child_datalist, HtmlDatalistElement);
    known_element_fn!(child_fieldset, HtmlFieldsetElement);
    known_element_fn!(child_form, HtmlFormElement);
    known_element_fn!(child_input, HtmlInputElement);
    known_element_fn!(child_label, HtmlLabelElement);
    known_element_fn!(child_legend, HtmlLegendElement);
    known_element_fn!(child_meter, HtmlMeterElement);
    known_element_fn!(child_optgroup, HtmlOptgroupElement);
    known_element_fn!(child_option, HtmlOptionElement);
    known_element_fn!(child_output, HtmlOutputElement);
    known_element_fn!(child_progress, HtmlProgressElement);
    known_element_fn!(child_select, HtmlSelectElement);
    known_element_fn!(child_textarea, HtmlTextareaElement);
    known_element_fn!(child_details, HtmlDetailsElement);
    known_element_fn!(child_dialog, HtmlDialogElement);
    known_element_fn!(child_summary, HtmlSummaryElement);
    known_element_fn!(child_slot, HtmlSlotElement);
    known_element_fn!(child_template, HtmlTemplateElement);
}
