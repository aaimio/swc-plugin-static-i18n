use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use swc_core::ecma::transforms::testing::test_inline;
use swc_core::plugin::proxies::TransformPluginProgramMetadata;
use swc_ecma_ast::{CallExpr, Callee, Expr, ExprOrSpread, Lit, Program};
use swc_ecma_visit::{as_folder, FoldWith, VisitMut, VisitMutWith};
use swc_plugin_macro::plugin_transform;

#[derive(Deserialize)]
pub struct StaticI18n {
    function_name: String,
    strings: HashMap<String, String>,
}

impl VisitMut for StaticI18n {
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if let Callee::Expr(expr) = &call_expr.callee {
            if let Expr::Ident(ident) = &**expr {
                if ident.sym != self.function_name {
                    return;
                }

                if let Some(ExprOrSpread { expr, .. }) = call_expr.args.get_mut(0) {
                    if let Expr::Lit(Lit::Str(original)) = &mut **expr {
                        if let Some(replacement) = self.strings.get(original.value.as_ref()) {
                            original.value = replacement.clone().into();
                            original.raw = None;
                        }
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<StaticI18n>(
        &metadata
            .get_transform_plugin_config()
            .expect("Invalid config"),
    )
    .expect("Failed to deserialize config");

    program.fold_with(&mut as_folder(StaticI18n { ..config }))
}

test_inline!(
    Default::default(),
    |_| {
        let mut translations = HashMap::new();

        translations.insert(
            "Hello World. Goodbye Mars.".to_string(),
            "Hallo Wereld. Tot ziens Mars.".to_string(),
        );

        as_folder(StaticI18n {
            function_name: "translate".to_string(),
            strings: translations,
        })
    },
    complex_transform,
    r#"const a = translate("Hello World. Goodbye Mars.");"#,
    r#"const a = translate("Hallo Wereld. Tot ziens Mars.");"#
);
