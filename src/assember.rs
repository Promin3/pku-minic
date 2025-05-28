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

use koopa::ir::ValueKind;
impl GenerateAsm for koopa::ir::FunctionData {
    fn generate(&self, output: &mut String) {
        output.push_str(format!("  .globl {}\n",self.name().trim_start_matches('@')).as_str());
        output.push_str(format!("{}:\n",self.name().trim_start_matches('@')).as_str());
        for (&bb, node) in self.layout().bbs() {
            for &inst in node.insts().keys(){
                let inst_value_data = self.dfg().value(inst);
                match inst_value_data.kind(){
                    ValueKind:: Return(ret) =>{
                        let ret_value_id = ret.value().unwrap();
                        let ret_value_data = self.dfg().value(ret_value_id);
                        match ret_value_data.kind(){
                            ValueKind::Integer(int) =>{
                                output.push_str(&format!("  li a0, {}\n",int.value()));
                            }
                            _=> unreachable!()
                        }

                        output.push_str("  ret");
                    }
                    _ => unreachable!() 
                }

            }
        }
    }
}


