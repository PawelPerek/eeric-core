import os

BRANCHES = """
Flw(_) => todo!(),
Fsw(_) => todo!(),
Fmadds(_) => todo!(),
Fmsubs(_) => todo!(),
Fnmsubs(_) => todo!(),
Fnmadds(_) => todo!(),
Fadds(_) => todo!(),
Fsubs(_) => todo!(),
Fmuls(_) => todo!(),
Fdivs(_) => todo!(),
Fsqrts(_) => todo!(),
Fsgnjs(_) => todo!(),
Fsgnjns(_) => todo!(),
Fsgnjxs(_) => todo!(),
Fmins(_) => todo!(),
Fmaxs(_) => todo!(),
Fcvtws(_) => todo!(),
Fcvtwus(_) => todo!(),
Fmvxw(_) => todo!(),
Feqs(_) => todo!(),
Flts(_) => todo!(),
Fles(_) => todo!(),
Fclasss(_) => todo!(),
Fcvtsw(_) => todo!(),
Fcvtswu(_) => todo!(),
Fmvwx(_) => todo!(),
Fcvtls(_) => todo!(),
Fcvtlus(_) => todo!(),
Fcvtsl(_) => todo!(),
Fcvtslu(_) => todo!(),
"""

EXTENSION = "m"

def extract_instructions_with_todo(rust_code):
    instructions = []
    for line in rust_code.split('\n'):
        if "todo!()" in line:
            # Extract instruction name
            instruction = line.strip().split('(')[0]
            instructions.append(instruction)
    return instructions

def create_instruction_files(instructions):
    root_dir = "src/rv_core/instruction/executor/"

    for instruction in instructions:
        if not instruction.endswith('s'):
            directory = os.path.join(root_dir, EXTENSION)
            if not os.path.exists(directory):
                os.makedirs(directory)

            file_path = os.path.join(directory, f"{instruction.lower()}.rs")
            with open(file_path, "w") as file:
                file.write("""
    use crate::rv_core::{{
        instruction::format::base::R, 
        registers::integer::IntegerRegisters
    }};

    pub fn {0}(R {{ rd, rs1, rs2 }}: R, x: &IntegerRegisters) {{
        todo!()
    }}
                """.format(instruction.lower()))

            # Add the created file reference to the mod.rs file
            mod_file_path = os.path.join(directory, "mod.rs")
            with open(mod_file_path, "a") as mod_file:
                mod_file.write(f"\nmod {instruction.lower()};\npub use {instruction.lower()}::{instruction.lower()};")
        else:
            instruction = instruction[:-1]
            directory = os.path.join(root_dir, EXTENSION)
            if not os.path.exists(directory):
                os.makedirs(directory)

            file_path = os.path.join(directory, f"{instruction.lower()}.rs")
            with open(file_path, "w") as file:
                file.write("""
    use crate::rv_core::{{
        instruction::format::base::R, 
        registers::integer::IntegerRegisters
    }};

    pub fn {0}(R {{ rd, rs1, rs2 }}: R, x: &IntegerRegisters) {{
        todo!()
    }}
                """.format("s"))

            # Add the created file reference to the mod.rs file
            mod_file_path = os.path.join(directory, "mod.rs")
            with open(mod_file_path, "a") as mod_file:
                mod_file.write(f"\npub use {instruction.lower()}")


instructions_with_todo = extract_instructions_with_todo(BRANCHES)
create_instruction_files(instructions_with_todo)

print(f"Files for {len(instructions_with_todo)} instructions have been created!")
