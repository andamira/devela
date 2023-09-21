// devela::codegen::deprecate
//
//!
//

/// Set a deprecated feature in order to display a warning if it's enabled.
///
/// You can only call this macro once per deprecated feature.
///
/// # Examples
/// ```
/// # use devela::deprecate_feature;
/// deprecate_feature![old: "old-feature-1"];
/// deprecate_feature![old: "old-feature-2", since: "2.0.0"];
/// deprecate_feature![old: "old-feature-3", new: "new_feature_3"];
/// deprecate_feature![old: "old-feature-4", new: "new_feature_4", since: "4.0.0"];
/// ```
#[macro_export]
macro_rules! deprecate_feature {
    (     old:   $old_feature:literal
      $(, new:   $new_feature:literal )?
      $(, since:       $since:literal )?
    ) => {
        $crate::paste! {
            // old, !new, !since
            #[$crate::codegen::compile_attr(
                all( none($($new_feature)?), none($($since)?) ),
                deprecated(note = "\nWARNING. `" $old_feature
                "` feature deprecated." )
            )]

            // old, !new, since
            #[$crate::codegen::compile_attr(
                all( none($($new_feature)?), some($($since)?) ),
                deprecated( $(since = $since,)? note = "\nWARNING. `" $old_feature
                "` feature deprecated since version " $($since)? "." )
            )]

            // old, new, !since
            #[$crate::codegen::compile_attr(
                all( some($($new_feature)?), none($($since)?) ),
                deprecated(note = "\nWARNING. `" $old_feature
                "` feature deprecated, use `" $($new_feature)? "` instead." )
            )]

            // old, new, since
            #[$crate::codegen::compile_attr(
                all( some($($new_feature)?), some($($since)?) ),
                deprecated( $(since = $since,)? note = "\nWARNING. `" $old_feature
                "` feature deprecated since version " $($since)?
                ", use `" $($new_feature)? "` instead." )
            )]

            #[cfg(feature = $old_feature)]
            const fn [<deprecate_feature_ $old_feature:snake>]() {}

            #[cfg(feature = $old_feature)]
            #[allow(dead_code)]
            const [<DEPRECATED_ $old_feature:snake:upper>]: ()
                = [<deprecate_feature_ $old_feature:snake>]();
        }
    };
}
pub use deprecate_feature;
