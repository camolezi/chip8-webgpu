use super::*;

#[test]
fn set_register_test() {
    let register_number = 2;
    let data = 43;

    let raw_instruction = create_raw_instruction_w_data((0x7, register_number), data);

    let mut test = InstructionTest::default();

    test.execute(raw_instruction);

    test.assert(|vm_state| {
        assert_eq!(vm_state.registers.get_data_register(register_number), data);
    })
}
