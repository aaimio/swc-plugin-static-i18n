use serde::Deserialize;
use serde_json;
use std::collections::HashMap;

use swc_core::{
    ecma::{
        ast::{CallExpr, Callee, Expr, ExprOrSpread, Lit, Program},
        transforms::testing::{test, test_inline},
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

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
    let config = serde_json::from_str::<TransformVisitor>(
        &metadata
            .get_transform_plugin_config()
            .expect("Invalid config"),
    )
    .expect("Failed to deserialize config");

    program.fold_with(&mut as_folder(TransformVisitor { ..config }))
}

test_inline!(
    Default::default(),
    |_| {
        let mut translations = HashMap::new();

        translations.insert(
            "Hello World. Goodbye Mars.".to_string(),
            "Hallo Welt. Auf Wiedersehen Mars.".to_string(),
        );

        translations.insert(
            "You are {{ age }} years old.".to_string(),
            "Du bist {{ age }} Jahre alt.".to_string(),
        );

        as_folder(TransformVisitor {
            function_name: "translate".to_string(),
            strings: translations,
        })
    },
    complex_transform,
    // Input
    r#"
    const a = translate("Hello World. Goodbye Mars.");
    const b = translate("You are {{ age }} years old.", { age: 25 });
    "#,
    // Output
    r#"
    const a = translate("Hallo Welt. Auf Wiedersehen Mars.");
    const b = translate("Du bist {{ age }} Jahre alt.", { age: 25 });
    "#
);
