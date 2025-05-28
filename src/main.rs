use assember::GenerateAsm;
use ir::generate_ir;
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::fs;
use std::io::Result;

mod ir;
mod ast;
mod assember;

// 引用 lalrpop 生成的解析器
// 因为我们刚刚创建了 sysy.lalrpop, 所以模块名是 sysy

lalrpop_mod!(sysy);

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let _mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  // 读取输入文件
  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

  // // 输出解析得到的 AST
  // println!("{:#?}", ast);
  // fs::write(output, generate_ir(&ast))?;
  let ir_string = generate_ir(&ast);
  let driver = koopa::front::Driver::from(ir_string);
  let program = driver.generate_program().unwrap();
  let mut asm_string = String::new();
  program.generate(&mut asm_string);
  fs::write(output, asm_string)?;
  Ok(())
  
}
