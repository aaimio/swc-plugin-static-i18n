use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use swc_core::ecma::{
    ast::{CallExpr, Callee, Expr, ExprOrSpread, Lit, Program},
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[derive(Deserialize)]
pub struct TransformVisitor {
    function_name: String,
    strings: HashMap<String, String>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if let Callee::Expr(expr) = &call_expr.callee {
            if let Expr::Ident(ident) = &**expr {
                if ident.sym != *self.function_name {
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
    let config: TransformVisitor = serde_json::from_str(
        &metadata
            .get_transform_plugin_config()
            .expect("Invalid config"),
    )
    .expect("Failed to deserialize config");

    program.fold_with(&mut as_folder(TransformVisitor {
        function_name: config.function_name,
        strings: config.strings,
    }))
}

test!(
    Default::default(),
    |_| {
        let mut translations = HashMap::new();

        translations.insert("Hello World".to_string(), "Hallo Wereld".to_string());

        as_folder(TransformVisitor {
            function_name: "translate".to_string(),
            strings: translations,
        })
    },
    complex_transform,
    // Where output? https://github.com/swc-project/website/commit/4f494565822140e8fad498416e205ceda7e6e86f#commitcomment-134090267
    "translate(\"Hello Mars\");",
    ok_if_code_eq
);
