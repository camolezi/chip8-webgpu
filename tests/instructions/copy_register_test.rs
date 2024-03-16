use super::*;

#[test]
fn copy_register_test() {
    let data = 68;
    let register_number = 0x2;
    let copied_register_number = 0x6;

    let raw_instruction =
        create_raw_instruction((0x8, register_number), (copied_register_number, 0x0));

    let mut test = InstructionTest::default();

    test.setup(|vm_state| {
        vm_state
            .registers
            .set_data_register(copied_register_number, data);
    });

    test.execute(raw_instruction);

    test.assert(|vm_state| {
        assert_eq!(vm_state.registers.get_data_register(register_number), data);
    })
}
