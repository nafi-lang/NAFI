use crate::node::Expr;

test!(Expr: "｢Expr｣+｢Expr｣*｢Expr｣+｢Expr｣", "｢Expr｣*｢Expr｣+｢Expr｣*｢Expr｣");
parse!(Expr!: |p| parse_expr(p, f32::NEG_INFINITY));

fn parse_single_expr(p: &mut Parser<'_>) {
    match p.peek() {
        Some(SyntaxKind::Expr) => p.bump(),
        kind => panic!("unexpected kind {:?} at begining of expr", kind),
    }
}

fn parse_expr(p: &mut Parser<'_>, current_binding_power: f32) {
    let checkpoint = p.checkpoint();

    parse_single_expr(p);

    while let Some(SyntaxKind::Syntax) = p.peek() {
        let (left_power, right_power) = op_binding_power(p.peek_src().unwrap());
        if left_power < current_binding_power {
            break;
        }
        p.bump();
        parse_expr(p, right_power);
        p.wrap_node_from(checkpoint, SyntaxKind::Expr);
    }
}

fn op_binding_power(op: &str) -> (f32, f32) {
    match op {
        "+" | "-" => (10.0, 11.0),
        "*" | "/" => (20.0, 21.0),
        _ => (f32::NEG_INFINITY, f32::INFINITY),
    }
}
