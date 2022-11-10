pub fn doc_file(c: char) -> Option<&'static str> {
    match c {
        '+' => Some("help/conjugate_add.md"),
        '-' => Some("help/negate_subtract.md"),
        '×' => Some("help/sign_multiply.md"),
        '÷' => Some("help/reciprocal_divide.md"),
        '⋆' => Some("help/exponential_power.md"),
        '√' => Some("help/squareroot_root.md"),
        '⌊' => Some("help/floor_minimum.md"),
        '⌈' => Some("help/ceiling_maximum.md"),
        '∧' => Some("help/sortup_and.md"),
        '∨' => Some("help/sortdown_or.md"),
        '¬' => Some("help/not_span.md"),
        '|' => Some("help/absolutevalue_modulus.md"),
        '≤' => Some("help/lessthanorequalto.md"),
        '<' => Some("help/enclose_lessthan.md"),
        '>' => Some("help/merge_greaterthan.md"),
        '≥' => Some("help/greaterthanorequalto.md"),
        '=' => Some("help/rank_equals.md"),
        '≠' => Some("help/length_notequals.md"),
        '≡' => Some("help/depth_match.md"),
        '≢' => Some("help/shape_notmatch.md"),
        '⊣' => Some("help/identity_left.md"),
        '⊢' => Some("help/identity_right.md"),
        '⥊' => Some("help/deshape_reshape.md"),
        '∾' => Some("help/join_jointo.md"),
        '≍' => Some("help/solo_couple.md"),
        '⋈' => Some("help/enlist_pair.md"),
        '↑' => Some("help/prefixes_take.md"),
        '↓' => Some("help/suffixes_drop.md"),
        '↕' => Some("help/range_windows.md"),
        '«' => Some("help/shiftbefore.md"),
        '»' => Some("help/shiftafter.md"),
        '⌽' => Some("help/reverse_rotate.md"),
        '⍉' => Some("help/transpose_reorderaxes.md"),
        '/' => Some("help/indices_replicate.md"),
        '⍋' => Some("help/gradeup_binsup.md"),
        '⍒' => Some("help/gradedown_binsdown.md"),
        '⊏' => Some("help/firstcell_select.md"),
        '⊑' => Some("help/first_pick.md"),
        '⊐' => Some("help/classify_indexof.md"),
        '⊒' => Some("help/occurrencecount_progressiveindexof.md"),
        '∊' => Some("help/markfirst_memberof.md"),
        '⍷' => Some("help/deduplicate_find.md"),
        '⊔' => Some("help/groupindices_group.md"),
        '!' => Some("help/assert_assertwithmessage.md"),
        '˙' => Some("help/constant.md"),
        '˜' => Some("help/self_swap.md"),
        '∘' => Some("help/atop.md"),
        '○' => Some("help/over.md"),
        '⊸' => Some("help/before_bind.md"),
        '⟜' => Some("help/after_bind.md"),
        '⌾' => Some("help/under.md"),
        '⊘' => Some("help/valences.md"),
        '◶' => Some("help/choose.md"),
        '⎊' => Some("help/catch.md"),
        '⎉' => Some("help/rank.md"),
        '˘' => Some("help/cells.md"),
        '⚇' => Some("help/depth.md"),
        '¨' => Some("help/each.md"),
        '⌜' => Some("help/table.md"),
        '⍟' => Some("help/repeat.md"),
        '⁼' => Some("help/undo.md"),
        '´' => Some("help/fold.md"),
        '˝' => Some("help/insert.md"),
        '`' => Some("help/scan.md"),
        '←' => Some("help/define.md"),
        '⇐' => Some("help/export.md"),
        '↩' => Some("help/change.md"),
        ',' => Some("help/separator.md"),
        '.' => Some("help/namespacefield.md"),
        '(' => Some("help/beginexpression.md"),
        ')' => Some("help/endexpression.md"),
        '{' => Some("help/beginblock.md"),
        '}' => Some("help/endblock.md"),
        ';' => Some("help/nextbody.md"),
        ':' => Some("help/header.md"),
        '?' => Some("help/predicate.md"),
        '⟨' => Some("help/beginlist.md"),
        '⟩' => Some("help/endlist.md"),
        '[' => Some("help/beginarray.md"),
        ']' => Some("help/endarray.md"),
        '‿' => Some("help/strand.md"),
        '·' => Some("help/nothing.md"),
        '•' => Some("help/system.md"),
        '𝕨' => Some("help/leftargument.md"),
        '𝕩' => Some("help/rightargument.md"),
        '𝔽' => Some("help/modifierleftoperand.md"),
        '𝔾' => Some("help/2-modifierrightoperand.md"),
        '𝕊' => Some("help/currentfunction.md"),
        '𝕣' => Some("help/currentmodifier.md"),
        '¯' => Some("help/minus.md"),
        'π' => Some("help/pi.md"),
        '∞' => Some("help/infinity.md"),
        '@' => Some("help/nullcharacter.md"),
        '#' => Some("help/comment.md"),
        '\'' => Some("help/character.md"),
        '"' => Some("help/string.md"),
        _ => None,
    }
}