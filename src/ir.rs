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
    
    // 处理函数体
    generate_block(&func_def.block, ir);
    
    // 函数结束
    ir.push_str("}\n");
}

fn generate_block(block: &Block, ir: &mut String) {
    // 处理语句
    generate_stmt(&block.stmt, ir);
}

fn generate_stmt(stmt: &Stmt, ir: &mut String) {
    // 处理 return 语句
    ir.push_str(&format!("    ret {}\n", stmt.num));
}