var srcIndex = JSON.parse('{\
"anstream":["",[["adapter",[],["mod.rs","strip.rs","wincon.rs"]]],["auto.rs","buffer.rs","fmt.rs","lib.rs","macros.rs","stream.rs","strip.rs","wincon.rs"]],\
"anstyle":["",[],["color.rs","effect.rs","lib.rs","macros.rs","reset.rs","style.rs"]],\
"anstyle_parse":["",[["state",[],["definitions.rs","mod.rs","table.rs"]]],["lib.rs","params.rs"]],\
"anstyle_query":["",[],["lib.rs","windows.rs"]],\
"anstyle_wincon":["",[],["ansi.rs","lib.rs","stream.rs","windows.rs"]],\
"anyhow":["",[],["backtrace.rs","chain.rs","context.rs","ensure.rs","error.rs","fmt.rs","kind.rs","lib.rs","macros.rs","ptr.rs","wrapper.rs"]],\
"bst_words":["",[],["bst.rs","lib.rs","type.rs","utils.rs"]],\
"cfg_if":["",[],["lib.rs"]],\
"clap":["",[],["lib.rs"]],\
"clap_builder":["",[["builder",[],["action.rs","app_settings.rs","arg.rs","arg_group.rs","arg_predicate.rs","arg_settings.rs","command.rs","debug_asserts.rs","ext.rs","mod.rs","os_str.rs","possible_value.rs","range.rs","resettable.rs","str.rs","styled_str.rs","styling.rs","value_hint.rs","value_parser.rs"]],["error",[],["context.rs","format.rs","kind.rs","mod.rs"]],["output",[["textwrap",[],["core.rs","mod.rs"]]],["fmt.rs","help.rs","help_template.rs","mod.rs","usage.rs"]],["parser",[["features",[],["mod.rs","suggestions.rs"]],["matches",[],["arg_matches.rs","matched_arg.rs","mod.rs","value_source.rs"]]],["arg_matcher.rs","error.rs","mod.rs","parser.rs","validator.rs"]],["util",[],["any_value.rs","color.rs","flat_map.rs","flat_set.rs","graph.rs","id.rs","mod.rs","str_to_bool.rs"]]],["derive.rs","lib.rs","macros.rs","mkeymap.rs"]],\
"clap_derive":["",[["derives",[],["args.rs","into_app.rs","mod.rs","parser.rs","subcommand.rs","value_enum.rs"]],["utils",[],["doc_comments.rs","error.rs","mod.rs","spanned.rs","ty.rs"]]],["attr.rs","dummies.rs","item.rs","lib.rs","macros.rs"]],\
"clap_lex":["",[],["ext.rs","lib.rs"]],\
"colorchoice":["",[],["lib.rs"]],\
"console":["",[["windows_term",[],["mod.rs"]]],["ansi.rs","common_term.rs","kb.rs","lib.rs","term.rs","utils.rs"]],\
"dialoguer":["",[["prompts",[],["confirm.rs","input.rs","mod.rs","multi_select.rs","password.rs","select.rs","sort.rs"]],["theme",[],["colorful.rs","mod.rs","render.rs","simple.rs"]]],["completion.rs","edit.rs","error.rs","history.rs","lib.rs","paging.rs","validate.rs"]],\
"encode_unicode":["",[],["decoding_iterators.rs","errors.rs","lib.rs","traits.rs","utf16_char.rs","utf16_iterators.rs","utf8_char.rs","utf8_iterators.rs"]],\
"fastrand":["",[],["global_rng.rs","lib.rs"]],\
"heck":["",[],["kebab.rs","lib.rs","lower_camel.rs","shouty_kebab.rs","shouty_snake.rs","snake.rs","title.rs","train.rs","upper_camel.rs"]],\
"itoa":["",[],["lib.rs","udiv128.rs"]],\
"lazy_static":["",[],["inline_lazy.rs","lib.rs"]],\
"libc":["",[["windows",[["msvc",[],["mod.rs"]]],["mod.rs"]]],["fixed_width_ints.rs","lib.rs","macros.rs"]],\
"proc_macro2":["",[],["detection.rs","extra.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]],\
"quote":["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]],\
"ryu":["",[["buffer",[],["mod.rs"]],["pretty",[],["exponent.rs","mantissa.rs","mod.rs"]]],["common.rs","d2s.rs","d2s_full_table.rs","d2s_intrinsics.rs","digit_table.rs","f2s.rs","f2s_intrinsics.rs","lib.rs"]],\
"serde":["",[["de",[],["format.rs","ignored_any.rs","impls.rs","mod.rs","seed.rs","size_hint.rs","value.rs"]],["private",[],["de.rs","doc.rs","mod.rs","ser.rs"]],["ser",[],["fmt.rs","impls.rs","impossible.rs","mod.rs"]]],["integer128.rs","lib.rs","macros.rs"]],\
"serde_derive":["",[["internals",[],["ast.rs","attr.rs","case.rs","check.rs","ctxt.rs","mod.rs","receiver.rs","respan.rs","symbol.rs"]]],["bound.rs","de.rs","dummy.rs","fragment.rs","lib.rs","pretend.rs","ser.rs","this.rs"]],\
"serde_json":["",[["features_check",[],["mod.rs"]],["io",[],["mod.rs"]],["value",[],["de.rs","from.rs","index.rs","mod.rs","partial_eq.rs","ser.rs"]]],["de.rs","error.rs","iter.rs","lib.rs","macros.rs","map.rs","number.rs","read.rs","ser.rs"]],\
"shell_words":["",[],["lib.rs"]],\
"strsim":["",[],["lib.rs"]],\
"syn":["",[["gen",[],["clone.rs"]]],["attr.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","drops.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","gen_helper.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","meta.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","restriction.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]],\
"tempfile":["",[["file",[["imp",[],["mod.rs","windows.rs"]]],["mod.rs"]]],["dir.rs","error.rs","lib.rs","spooled.rs","util.rs"]],\
"thiserror":["",[],["aserror.rs","display.rs","lib.rs","provide.rs"]],\
"thiserror_impl":["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","span.rs","valid.rs"]],\
"unicode_ident":["",[],["lib.rs","tables.rs"]],\
"unicode_width":["",[],["lib.rs","tables.rs"]],\
"utf8parse":["",[],["lib.rs","types.rs"]],\
"windows_targets":["",[],["lib.rs"]],\
"windows_x86_64_msvc":["",[],["lib.rs"]],\
"zeroize":["",[],["lib.rs","x86.rs"]]\
}');
createSrcSidebar();