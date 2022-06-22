use std::fmt::Debug;

use arwa::dom::Name;
use arwa::event::EventTarget;
use arwa::html::*;
use arwa::ui::*;
use futures::Sink;

use crate::vdom::ElementBuilder;

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

macro_rules! ui_event_sink_fn {
    ($fn_name:ident, $event:ident) => {
        fn $fn_name<S>(&mut self, sink: S)
        where
            E: EventTarget + 'static,
            S: Sink<$event<E>> + 'static,
            S::Error: Debug,
        {
            sink_ui_event_ext_seal::Seal::sink_event(self, sink);
        }
    };
}

pub(crate) mod sink_ui_event_ext_seal {
    use arwa::event::{EventTarget, TypedEvent};
    use futures::Sink;
    use std::fmt::Debug;

    pub trait Seal<E> {
        fn sink_event<T, S>(&mut self, sink: S)
        where
            E: EventTarget,
            T: TypedEvent<CurrentTarget = E> + 'static,
            S: Sink<T> + 'static,
            S::Error: Debug;
    }
}

pub trait SinkUIEventExt<E>: sink_ui_event_ext_seal::Seal<E> {
    ui_event_sink_fn!(sink_input, InputEvent);
    ui_event_sink_fn!(sink_before_input, BeforeInputEvent);
    ui_event_sink_fn!(sink_focus_in, FocusInEvent);
    ui_event_sink_fn!(sink_focus_out, FocusOutEvent);
    ui_event_sink_fn!(sink_click, ClickEvent);
    ui_event_sink_fn!(sink_dbl_click, DblClickEvent);
    ui_event_sink_fn!(sink_aux_click, AuxClickEvent);
    ui_event_sink_fn!(sink_context_menu, ContextMenuEvent);
    ui_event_sink_fn!(sink_pointer_cancel, PointerCancelEvent);
    ui_event_sink_fn!(sink_pointer_down, PointerDownEvent);
    ui_event_sink_fn!(sink_pointer_move, PointerMoveEvent);
    ui_event_sink_fn!(sink_pointer_up, PointerUpEvent);
    ui_event_sink_fn!(sink_pointer_out, PointerOutEvent);
    ui_event_sink_fn!(sink_pointer_over, PointerOverEvent);
    ui_event_sink_fn!(sink_pointer_enter, PointerEnterEvent);
    ui_event_sink_fn!(sink_pointer_leave, PointerLeaveEvent);
    ui_event_sink_fn!(sink_got_pointer_capture, GotPointerCaptureEvent);
    ui_event_sink_fn!(sink_lost_pointer_capture, LostPointerCaptureEvent);
    ui_event_sink_fn!(sink_drag, DragEvent);
    ui_event_sink_fn!(sink_drag_end, DragEndEvent);
    ui_event_sink_fn!(sink_drag_enter, DragEnterEvent);
    ui_event_sink_fn!(sink_drag_leave, DragLeaveEvent);
    ui_event_sink_fn!(sink_drag_over, DragOverEvent);
    ui_event_sink_fn!(sink_drag_start, DragStartEvent);
    ui_event_sink_fn!(sink_drop, DropEvent);
    ui_event_sink_fn!(sink_key_down, KeyDownEvent);
    ui_event_sink_fn!(sink_key_up, KeyUpEvent);
    ui_event_sink_fn!(sink_wheel, WheelEvent);
}

macro_rules! attr_fn {
    ($fn_name:ident, $attr_name:literal) => {
        fn $fn_name(&mut self, value: &str) {
            self.attr(
                arwa::dom::Name::from_statically_parsed(arwa::dom::StaticallyParsedName {
                    name: $attr_name,
                }),
                value,
            );
        }
    };
}

macro_rules! boolean_attr_fn {
    ($fn_name:ident, $attr_name:literal) => {
        fn $fn_name(&mut self) {
            self.boolean_attr(arwa::dom::Name::from_statically_parsed(
                arwa::dom::StaticallyParsedName { name: $attr_name },
            ));
        }
    };
}

macro_rules! attr_ext_seal {
    ($seal_mod:ident) => {
        pub(crate) mod $seal_mod {
            use arwa::dom::Name;

            pub trait Seal {
                #[doc(hidden)]
                fn attr(&mut self, name: Name, value: &str);

                #[doc(hidden)]
                fn boolean_attr(&mut self, name: Name);
            }
        }
    };
}

attr_ext_seal!(global_attr_ext_seal);

pub trait GlobalAttrExt: global_attr_ext_seal::Seal {
    attr_fn!(attr_accesskey, "accesskey");
    attr_fn!(attr_class, "class");
    attr_fn!(attr_contenteditable, "contenteditable");
    attr_fn!(attr_contextmenu, "contextmenu");
    attr_fn!(attr_dir, "dir");
    attr_fn!(attr_draggable, "draggable");
    boolean_attr_fn!(attr_hidden, "hidden");
    attr_fn!(attr_id, "id");
    attr_fn!(attr_itemprop, "itemprop");
    attr_fn!(attr_lang, "lang");
    attr_fn!(attr_role, "role");
    attr_fn!(attr_slot, "slot");
    attr_fn!(attr_spellcheck, "spellcheck");
    attr_fn!(attr_style, "style");
    attr_fn!(attr_tabindex, "tabindex");
    attr_fn!(attr_title, "title");
    attr_fn!(attr_translate, "translate");
}

impl<'a, 'b, E> global_attr_ext_seal::Seal for ElementBuilder<'a, 'b, E> {
    fn attr(&mut self, name: Name, value: &str) {
        ElementBuilder::attr(self, name, value);
    }

    fn boolean_attr(&mut self, name: Name) {
        ElementBuilder::boolean_attr(self, name)
    }
}

impl<'a, 'b, E> GlobalAttrExt for ElementBuilder<'a, 'b, E> {}

macro_rules! impl_attr_ext {
    ($seal_mod:ident, $ext:ident, $element_tpe:ident) => {
        impl<'a, 'b> $seal_mod::Seal for ElementBuilder<'a, 'b, $element_tpe> {
            fn attr(&mut self, name: Name, value: &str) {
                ElementBuilder::attr(self, name, value)
            }

            fn boolean_attr(&mut self, name: Name) {
                ElementBuilder::boolean_attr(self, name)
            }
        }

        impl<'a, 'b> $ext for ElementBuilder<'a, 'b, $element_tpe> {}
    };
}

attr_ext_seal!(form_attr_ext_seal);

pub trait FormAttrExt: form_attr_ext_seal::Seal {
    attr_fn!(attr_accept_charset, "accept-charset");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_rel, "rel");
    attr_fn!(attr_action, "action");
    attr_fn!(attr_enctype, "enctype");
    attr_fn!(attr_method, "method");
    boolean_attr_fn!(attr_novalidate, "novalidate");
    attr_fn!(attr_target, "target");
}

impl_attr_ext!(form_attr_ext_seal, FormAttrExt, HtmlFormElement);

attr_ext_seal!(input_attr_ext_seal);

pub trait InputAttrExt: input_attr_ext_seal::Seal {
    attr_fn!(attr_accept, "accept");
    attr_fn!(attr_alt, "alt");
    boolean_attr_fn!(attr_autofocus, "autofocus");
    attr_fn!(attr_capture, "capture");
    boolean_attr_fn!(attr_checked, "checked");
    attr_fn!(attr_dirname, "dirname");
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_formaction, "formaction");
    attr_fn!(attr_formenctype, "formenctype");
    attr_fn!(attr_formmethod, "formmethod");
    attr_fn!(attr_formnovalidate, "formnovalidate");
    attr_fn!(attr_formtarget, "formtarget");
    attr_fn!(attr_height, "height");
    attr_fn!(attr_list, "list");
    attr_fn!(attr_max, "max");
    attr_fn!(attr_maxlength, "maxlength");
    attr_fn!(attr_min, "min");
    attr_fn!(attr_minlength, "minlength");
    boolean_attr_fn!(attr_multiple, "multiple");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_pattern, "pattern");
    attr_fn!(attr_placeholder, "placeholder");
    boolean_attr_fn!(attr_readonly, "readonly");
    boolean_attr_fn!(attr_required, "required");
    attr_fn!(attr_size, "size");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_step, "step");
    attr_fn!(attr_type, "type");
    attr_fn!(attr_value, "value");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(input_attr_ext_seal, InputAttrExt, HtmlInputElement);

// TODO: missing in arwa
// attr_ext_seal!(col_attr_ext_seal);
//
// pub trait ColAttrExt: col_attr_ext_seal::Seal {
//     attr_fn!(attr_span, "span");
// }
//
// impl_attr_ext!(col_attr_ext_seal, ColAttrExt, HtmlColElement);

attr_ext_seal!(colgroup_attr_ext_seal);

pub trait ColgroupAttrExt: colgroup_attr_ext_seal::Seal {
    attr_fn!(attr_span, "span");
}

impl_attr_ext!(colgroup_attr_ext_seal, ColgroupAttrExt, HtmlColgroupElement);

attr_ext_seal!(iframe_attr_ext_seal);

pub trait IframeAttrExt: iframe_attr_ext_seal::Seal {
    attr_fn!(attr_allow, "allow");
    attr_fn!(attr_fetchpriority, "fetchpriority");
    attr_fn!(attr_height, "height");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_sandbox, "sandbox");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_srcdoc, "srcdoc");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(iframe_attr_ext_seal, IframeAttrExt, HtmlIframeElement);

attr_ext_seal!(img_attr_ext_seal);

pub trait ImgAttrExt: img_attr_ext_seal::Seal {
    attr_fn!(attr_alt, "alt");
    attr_fn!(attr_crossorigin, "crossorigin");
    attr_fn!(attr_decoding, "decoding");
    attr_fn!(attr_fetchpriority, "fetchpriority");
    attr_fn!(attr_height, "height");
    attr_fn!(attr_ismap, "ismap");
    attr_fn!(attr_loading, "loading");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_sizes, "sizes");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_srcset, "srcset");
    attr_fn!(attr_width, "width");
    attr_fn!(attr_usemap, "usemap");
}

impl_attr_ext!(img_attr_ext_seal, ImgAttrExt, HtmlImgElement);

attr_ext_seal!(td_attr_ext_seal);

pub trait TdAttrExt: td_attr_ext_seal::Seal {
    attr_fn!(attr_colspan, "colspan");
    attr_fn!(attr_headers, "headers");
    attr_fn!(attr_rowspan, "rowspan");
}

impl_attr_ext!(td_attr_ext_seal, TdAttrExt, HtmlTdElement);

attr_ext_seal!(th_attr_ext_seal);

pub trait ThAttrExt: th_attr_ext_seal::Seal {
    attr_fn!(attr_abbr, "abbr");
    attr_fn!(attr_colspan, "colspan");
    attr_fn!(attr_headers, "headers");
    attr_fn!(attr_rowspan, "rowspan");
    attr_fn!(attr_scope, "scope");
}

impl_attr_ext!(th_attr_ext_seal, ThAttrExt, HtmlThElement);

attr_ext_seal!(area_attr_ext_seal);

pub trait AreaAttrExt: area_attr_ext_seal::Seal {
    attr_fn!(attr_alt, "alt");
    attr_fn!(attr_coords, "coords");
    attr_fn!(attr_download, "download");
    attr_fn!(attr_href, "href");
    attr_fn!(attr_hreflang, "hreflang");
    attr_fn!(attr_ping, "ping");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_rel, "rel");
    attr_fn!(attr_shape, "shape");
    attr_fn!(attr_target, "target");
}

impl_attr_ext!(area_attr_ext_seal, AreaAttrExt, HtmlAreaElement);

attr_ext_seal!(script_attr_ext_seal);

pub trait ScriptAttrExt: script_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_async, "async");
    attr_fn!(attr_crossorigin, "crossorigin");
    boolean_attr_fn!(attr_defer, "defer");
    attr_fn!(attr_fetchpriority, "fetchpriority");
    attr_fn!(attr_integrity, "integrity");
    boolean_attr_fn!(attr_nomodule, "nomodule");
    attr_fn!(attr_nonce, "nonce");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_type, "type");
}

impl_attr_ext!(script_attr_ext_seal, ScriptAttrExt, HtmlScriptElement);

attr_ext_seal!(select_attr_ext_seal);

pub trait SelectAttrExt: select_attr_ext_seal::Seal {
    attr_fn!(attr_autocomplete, "autocomplete");
    boolean_attr_fn!(attr_autofocus, "autofocus");
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_form, "form");
    boolean_attr_fn!(attr_multiple, "multiple");
    attr_fn!(attr_name, "name");
    boolean_attr_fn!(attr_required, "required");
    attr_fn!(attr_size, "size");
}

impl_attr_ext!(select_attr_ext_seal, SelectAttrExt, HtmlSelectElement);

attr_ext_seal!(textarea_attr_ext_seal);

pub trait TextareaAttrExt: textarea_attr_ext_seal::Seal {
    attr_fn!(attr_autocomplete, "autocomplete");
    boolean_attr_fn!(attr_autofocus, "autofocus");
    attr_fn!(attr_cols, "cols");
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_maxlength, "maxlength");
    attr_fn!(attr_minlength, "minlength");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_placeholder, "placeholder");
    boolean_attr_fn!(attr_readonly, "readonly");
    boolean_attr_fn!(attr_required, "required");
    attr_fn!(attr_rows, "rows");
    attr_fn!(attr_spellcheck, "spellcheck");
    attr_fn!(attr_wrap, "wrap");
}

impl_attr_ext!(textarea_attr_ext_seal, TextareaAttrExt, HtmlTextareaElement);

attr_ext_seal!(audio_attr_ext_seal);

pub trait AudioAttrExt: audio_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_autoplay, "autoplay");
    boolean_attr_fn!(attr_controls, "controls");
    attr_fn!(attr_crossorigin, "crossorigin");
    boolean_attr_fn!(attr_disableremoteplayback, "disableremoteplayback");
    boolean_attr_fn!(attr_loop, "loop");
    boolean_attr_fn!(attr_muted, "muted");
    attr_fn!(attr_preload, "preload");
    attr_fn!(attr_src, "src");
}

impl_attr_ext!(audio_attr_ext_seal, AudioAttrExt, HtmlAudioElement);

attr_ext_seal!(video_attr_ext_seal);

pub trait VideoAttrExt: video_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_autoplay, "autoplay");
    boolean_attr_fn!(attr_autopictureinpicture, "autopictureinpicture");
    boolean_attr_fn!(attr_controls, "controls");
    attr_fn!(attr_controlslist, "controlslist");
    attr_fn!(attr_crossorigin, "crossorigin");
    boolean_attr_fn!(attr_disablepictureinpicture, "disablepictureinpicture");
    boolean_attr_fn!(attr_disableremoteplayback, "disableremoteplayback");
    attr_fn!(attr_height, "height");
    boolean_attr_fn!(attr_loop, "loop");
    boolean_attr_fn!(attr_muted, "muted");
    boolean_attr_fn!(attr_playsinline, "playsinline");
    attr_fn!(attr_poster, "poster");
    attr_fn!(attr_preload, "preload");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(video_attr_ext_seal, VideoAttrExt, HtmlVideoElement);

attr_ext_seal!(object_attr_ext_seal);

pub trait ObjectAttrExt: object_attr_ext_seal::Seal {
    attr_fn!(attr_data, "data");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_height, "height");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_type, "type");
    attr_fn!(attr_usemap, "usemap");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(object_attr_ext_seal, ObjectAttrExt, HtmlObjectElement);

attr_ext_seal!(meta_attr_ext_seal);

pub trait MetaAttrExt: meta_attr_ext_seal::Seal {
    attr_fn!(attr_charset, "charset");
    attr_fn!(attr_content, "content");
    attr_fn!(attr_http_equiv, "http-equiv");
    attr_fn!(attr_name, "name");
}

impl_attr_ext!(meta_attr_ext_seal, MetaAttrExt, HtmlMetaElement);

attr_ext_seal!(blockquote_attr_ext_seal);

pub trait BlockquoteAttrExt: blockquote_attr_ext_seal::Seal {
    attr_fn!(attr_cite, "cite");
}

impl_attr_ext!(
    blockquote_attr_ext_seal,
    BlockquoteAttrExt,
    HtmlBlockquoteElement
);

attr_ext_seal!(del_attr_ext_seal);

pub trait DelAttrExt: del_attr_ext_seal::Seal {
    attr_fn!(attr_cite, "cite");
    attr_fn!(attr_datetime, "datetime");
}

impl_attr_ext!(del_attr_ext_seal, DelAttrExt, HtmlDelElement);

attr_ext_seal!(ins_attr_ext_seal);

pub trait InsAttrExt: ins_attr_ext_seal::Seal {
    attr_fn!(attr_cite, "cite");
    attr_fn!(attr_datetime, "datetime");
}

impl_attr_ext!(ins_attr_ext_seal, InsAttrExt, HtmlInsElement);

attr_ext_seal!(q_attr_ext_seal);

pub trait QAttrExt: q_attr_ext_seal::Seal {
    attr_fn!(attr_cite, "cite");
}

impl_attr_ext!(q_attr_ext_seal, QAttrExt, HtmlQElement);

attr_ext_seal!(link_attr_ext_seal);

pub trait LinkAttrExt: link_attr_ext_seal::Seal {
    attr_fn!(attr_as, "as");
    attr_fn!(attr_crossorigin, "crossorigin");
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_fetchpriority, "fetchpriority");
    attr_fn!(attr_href, "href");
    attr_fn!(attr_hreflang, "hreflang");
    attr_fn!(attr_imagesizes, "imagesizes");
    attr_fn!(attr_imagesrcset, "imagesrcset");
    attr_fn!(attr_integrity, "integrity");
    attr_fn!(attr_media, "media");
    attr_fn!(attr_prefetch, "prefetch");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_rel, "rel");
    attr_fn!(attr_sizes, "sizes");
    attr_fn!(attr_title, "title");
    attr_fn!(attr_type, "type");
}

impl_attr_ext!(link_attr_ext_seal, LinkAttrExt, HtmlLinkElement);

attr_ext_seal!(time_attr_ext_seal);

pub trait TimeAttrExt: time_attr_ext_seal::Seal {
    attr_fn!(attr_datetime, "datetime");
}

impl_attr_ext!(time_attr_ext_seal, TimeAttrExt, HtmlTimeElement);

attr_ext_seal!(track_attr_ext_seal);

pub trait TrackAttrExt: track_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_default, "default");
    attr_fn!(attr_kind, "kind");
    attr_fn!(attr_label, "label");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_srclang, "srclang");
}

impl_attr_ext!(track_attr_ext_seal, TrackAttrExt, HtmlTrackElement);

attr_ext_seal!(fieldset_attr_ext_seal);

pub trait FieldsetAttrExt: fieldset_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_name, "name");
}

impl_attr_ext!(fieldset_attr_ext_seal, FieldsetAttrExt, HtmlFieldsetElement);

attr_ext_seal!(optgroup_attr_ext_seal);

pub trait OptgroupAttrExt: optgroup_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_label, "label");
}

impl_attr_ext!(optgroup_attr_ext_seal, OptgroupAttrExt, HtmlOptgroupElement);

attr_ext_seal!(option_attr_ext_seal);

pub trait OptionAttrExt: optgroup_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_label, "label");
    boolean_attr_fn!(attr_selected, "selected");
    attr_fn!(attr_value, "value");
}

impl_attr_ext!(optgroup_attr_ext_seal, OptgroupAttrExt, HtmlOptionElement);

attr_ext_seal!(a_attr_ext_seal);

pub trait AAttrExt: a_attr_ext_seal::Seal {
    attr_fn!(attr_download, "download");
    attr_fn!(attr_href, "href");
    attr_fn!(attr_hreflang, "hreflang");
    attr_fn!(attr_ping, "ping");
    attr_fn!(attr_referrerpolicy, "referrerpolicy");
    attr_fn!(attr_rel, "rel");
    attr_fn!(attr_target, "target");
    attr_fn!(attr_type, "type");
}

impl_attr_ext!(a_attr_ext_seal, AAttrExt, HtmlAElement);

attr_ext_seal!(label_attr_ext_seal);

pub trait LabelAttrExt: label_attr_ext_seal::Seal {
    attr_fn!(attr_for, "for");
}

impl_attr_ext!(label_attr_ext_seal, LabelAttrExt, HtmlLabelElement);

attr_ext_seal!(output_attr_ext_seal);

pub trait OutputAttrExt: output_attr_ext_seal::Seal {
    attr_fn!(attr_for, "for");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_name, "name");
}

impl_attr_ext!(output_attr_ext_seal, OutputAttrExt, HtmlOutputElement);

attr_ext_seal!(meter_attr_ext_seal);

pub trait MeterAttrExt: meter_attr_ext_seal::Seal {
    attr_fn!(attr_value, "value");
    attr_fn!(attr_min, "min");
    attr_fn!(attr_max, "max");
    attr_fn!(attr_low, "low");
    attr_fn!(attr_high, "high");
    attr_fn!(attr_optimum, "optimum");
}

impl_attr_ext!(meter_attr_ext_seal, MeterAttrExt, HtmlMeterElement);

attr_ext_seal!(progress_attr_ext_seal);

pub trait ProgressAttrExt: progress_attr_ext_seal::Seal {
    attr_fn!(attr_max, "max");
    attr_fn!(attr_value, "value");
}

impl_attr_ext!(progress_attr_ext_seal, ProgressAttrExt, HtmlProgressElement);

attr_ext_seal!(button_attr_ext_seal);

pub trait ButtonAttrExt: button_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_autofocus, "autofocus");
    boolean_attr_fn!(attr_disabled, "disabled");
    attr_fn!(attr_form, "form");
    attr_fn!(attr_formaction, "formaction");
    attr_fn!(attr_formenctype, "formenctype");
    attr_fn!(attr_formmethod, "formmethod");
    attr_fn!(attr_formnovalidate, "formnovalidate");
    attr_fn!(attr_formtarget, "formtarget");
    attr_fn!(attr_name, "name");
    attr_fn!(attr_type, "type");
    attr_fn!(attr_value, "value");
}

impl_attr_ext!(button_attr_ext_seal, ButtonAttrExt, HtmlButtonElement);

attr_ext_seal!(canvas_attr_ext_seal);

pub trait CanvasAttrExt: canvas_attr_ext_seal::Seal {
    attr_fn!(attr_height, "height");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(canvas_attr_ext_seal, CanvasAttrExt, HtmlCanvasElement);

attr_ext_seal!(embed_attr_ext_seal);

pub trait EmbedAttrExt: embed_attr_ext_seal::Seal {
    attr_fn!(attr_height, "height");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_type, "type");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(embed_attr_ext_seal, EmbedAttrExt, HtmlEmbedElement);

attr_ext_seal!(base_attr_ext_seal);

pub trait BaseAttrExt: base_attr_ext_seal::Seal {
    attr_fn!(attr_href, "href");
    attr_fn!(attr_target, "target");
}

impl_attr_ext!(base_attr_ext_seal, BaseAttrExt, HtmlBaseElement);

attr_ext_seal!(source_attr_ext_seal);

pub trait SourceAttrExt: source_attr_ext_seal::Seal {
    attr_fn!(attr_type, "type");
    attr_fn!(attr_src, "src");
    attr_fn!(attr_srcset, "srcset");
    attr_fn!(attr_sizes, "sizes");
    attr_fn!(attr_media, "media");
    attr_fn!(attr_height, "height");
    attr_fn!(attr_width, "width");
}

impl_attr_ext!(source_attr_ext_seal, SourceAttrExt, HtmlSourceElement);

attr_ext_seal!(style_attr_ext_seal);

pub trait StyleAttrExt: style_attr_ext_seal::Seal {
    attr_fn!(attr_media, "media");
    attr_fn!(attr_nonce, "nonce");
    attr_fn!(attr_title, "title");
}

impl_attr_ext!(style_attr_ext_seal, StyleAttrExt, HtmlStyleElement);

attr_ext_seal!(map_attr_ext_seal);

pub trait MapAttrExt: map_attr_ext_seal::Seal {
    attr_fn!(attr_name, "name");
}

impl_attr_ext!(map_attr_ext_seal, MapAttrExt, HtmlMapElement);

attr_ext_seal!(details_attr_ext_seal);

pub trait DetailsAttrExt: details_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_open, "open");
}

impl_attr_ext!(details_attr_ext_seal, DetailsAttrExt, HtmlDetailsElement);

attr_ext_seal!(dialog_attr_ext_seal);

pub trait DialogAttrExt: dialog_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_open, "open");
}

impl_attr_ext!(dialog_attr_ext_seal, DialogAttrExt, HtmlDialogElement);

attr_ext_seal!(ol_attr_ext_seal);

pub trait OlAttrExt: ol_attr_ext_seal::Seal {
    boolean_attr_fn!(attr_reversed, "reversed");
    attr_fn!(attr_start, "start");
    attr_fn!(attr_type, "type");
}

impl_attr_ext!(ol_attr_ext_seal, OlAttrExt, HtmlOlElement);

attr_ext_seal!(data_attr_ext_seal);

pub trait DataAttrExt: data_attr_ext_seal::Seal {
    attr_fn!(attr_value, "value");
}

impl_attr_ext!(data_attr_ext_seal, DataAttrExt, HtmlDataElement);
