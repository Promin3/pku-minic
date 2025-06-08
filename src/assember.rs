pub trait GenerateAsm {
    fn generate(&self, output: &mut String);
}

impl GenerateAsm for koopa::ir::Program {
    fn generate(&self, output: &mut String) {
    output.push_str("  .text\n");
    for &func in self.func_layout() {
        let func_data = self.func(func);
        func_data.generate(output);
    }
  }
}

use koopa::ir::{BinaryOp, ValueKind};
impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self, output: &mut String) {
        output.push_str(format!("  .globl {}\n",self.name().trim_start_matches('@')).as_str());
        output.push_str(format!("{}:\n",self.name().trim_start_matches('@')).as_str());
        for (&_bb, node) in self.layout().bbs() {

            let mut t_register = 0;

            for &inst in node.insts().keys(){
                let inst_value_data = self.dfg().value(inst);
                
                match inst_value_data.kind(){
                    ValueKind::Binary(bin) =>{
                        let op = bin.op();
                        let lhs_value_id = bin.lhs();
                        let lhs_value_data = self.dfg().value(lhs_value_id);
                        let rhs_value_id = bin.rhs();
                        let rhs_value_data = self.dfg().value(rhs_value_id);
                        match op {
                            BinaryOp::Eq =>{
                                match lhs_value_data.kind() {
                                    ValueKind::Integer(int) => {
                                        output.push_str(&format!("  li t{}, {}\n",t_register,int.value()));
                                    }
                                    ValueKind::Binary(_bin) => {
                                        output.push_str(&format!("  li t{}, t{}\n",t_register,t_register-1));
                                    }
                                    _ => unreachable!()
                                    
                                };
                                output.push_str(&format!("  xor t{}, t{}, x0\n",t_register,t_register));
                                output.push_str(&format!("  seqz t{}, t{}\n",t_register,t_register));
                                t_register += 1;
                            },
                            BinaryOp::Sub =>{
                                match rhs_value_data.kind() {
                                    ValueKind::Integer(int) => {
                                        output.push_str(&format!("  sub t{}, x0, {}\n",t_register,int.value()));
                                    }
                                    ValueKind::Binary(_bin) => {
                                        output.push_str(&format!("  sub t{}, x0, t{}\n",t_register,t_register-1));
                                    }
                                    _ => unreachable!()
                                }
                                t_register += 1;
                            },
                            _ => unreachable!()
                        }
                    },
                
                    ValueKind::Return(ret) =>{
                        let ret_value_id = ret.value().unwrap();
                        let ret_value_data = self.dfg().value(ret_value_id);
                        match ret_value_data.kind(){
                            ValueKind::Integer(int) =>{
                                output.push_str(&format!("  li a0, {}\n",int.value()));
                            }
                            ValueKind::Binary(_bin) =>{
                                output.push_str(&format!("  mv a0, t{}\n",t_register-1));
                            }
                            _=> unreachable!()
                        }

                        output.push_str("  ret");
                    },
                    _ => unreachable!() 
                }

            }
        }
    }
}

