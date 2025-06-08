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
// cargo run -- -koopa ./debug/hello.c -o hello.riscv
lalrpop_mod!(sysy);

fn main() -> Result<()> {
  // 解析命令行参数
  let mut args = args();
  args.next();
  let mode = args.next().unwrap();
  let input = args.next().unwrap();
  args.next();
  let output = args.next().unwrap();

  let input = read_to_string(input)?;

  // 调用 lalrpop 生成的 parser 解析输入文件
  let ast = sysy::CompUnitParser::new().parse(&input).unwrap();

  // 将 AST 转换为 Koopa IR
  let ir_string = generate_ir(&ast);
  println!("{}", ir_string);

  // 将 文本形式的 Koopa IR 转换为内存形式
  let ir_string_copy = ir_string.clone();
  let driver = koopa::front::Driver::from(ir_string_copy);
  let program = driver.generate_program().unwrap();

  // 将 内存形式Koopa IR 转换为 RISC-V 汇编代码
  let mut asm_string = String::new();
  program.generate(&mut asm_string);
  println!("{}", asm_string);

  if mode == "-koopa" {
    // 如果是 -koopa 模式，输出 Koopa IR
    fs::write(output, ir_string)?;
  } else if mode == "-riscv" {
    // 如果是 -riscv 模式，输出 RISC-V 汇编代码
    fs::write(output, asm_string)?;
  }

  Ok(())
}
