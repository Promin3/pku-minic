// 遍历生成的 ast，输出 Koopa IR 代码
use crate::ast::*;

pub fn generate_ir(ast: &CompUnit) -> String {
    let mut ir = String::new();
    
    // 处理编译单元（CompUnit）
    generate_func_def(&ast.func_def, &mut ir);
    
    ir
}

fn generate_func_def(func_def: &FuncDef, ir: &mut String) {
    // 函数头：根据函数名和返回类型生成
    let ret_type = match func_def.func_type {
        FuncType::Int => "i32",
        FuncType::Void => "", // 通常 void 在 Koopa IR 中表示为无返回值
    };
    
    ir.push_str(&format!("fun @{}(): {} {{\n", func_def.ident, ret_type));
    
    // 生成入口基本块
    ir.push_str("%entry:\n");
    
    let mut mark = 0;
    generate_block(&func_def.block, ir, &mut mark);
    
    ir.push_str("}\n");
}

fn generate_block(block: &Block, ir: &mut String, mark: &mut i32) {
    // 处理语句
    generate_stmt(&block.stmt, ir, mark);
}

fn generate_stmt(stmt: &Stmt, ir: &mut String, mark: &mut i32) {
    // 处理 return 语句
    generate_exp(&stmt.exp,ir, mark);
    ir.push_str(&format!("  ret %{}\n", *mark - 1));
}

fn generate_exp(exp: &Exp, ir: &mut String, mark: &mut i32) -> String {
    generate_unary_exp(&exp.unaryexp, ir, mark)
}

fn generate_unary_exp(unaryexp: &UnaryExp, ir: &mut String, mark: &mut i32) -> String {
    match unaryexp {
        UnaryExp::Primary(primary) => generate_primary_exp(primary, ir, mark),
        UnaryExp::Unary(op, exp) => {
            match op {
                UnaryOp::Neg => {
                    let exp_str = generate_unary_exp(exp, ir, mark);
                    ir.push_str(&format!("  %{} = sub 0, {}\n", mark, exp_str));
                    *mark += 1;
                    format!("%{}", *mark - 1)
                },
                UnaryOp::LogiNot => {
                    let exp_str = generate_unary_exp(exp, ir, mark);
                    ir.push_str(&format!("  %{} = eq {}, 0\n", mark, exp_str));
                    *mark += 1;
                    format!("%{}", *mark - 1)
                }
            }
        }
        
    }
}

fn generate_primary_exp(primary: &PrimaryExp, ir: &mut String, mark: &mut i32) -> String {
    match primary {
        PrimaryExp::Exp(exp) => generate_exp(exp, ir, mark),
        PrimaryExp::Number(num) => num.to_string()
    }
}