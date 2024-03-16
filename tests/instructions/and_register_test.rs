use super::*;

#[test]
fn and_register_test() {
    let register_x = 0x5;
    let register_y = 0xa;

    let register_data_x = 0b10001001;
    let register_data_y = 0b00010101;

    //0b10001001 | 0b00010101 = 0b10011101
    let expected_result = 0b10011101;

    let raw_instruction = create_raw_instruction((0x8, register_x), (register_y, 0x1));

    let mut test = InstructionTest::default();

    test.setup(|vm_state| {
        vm_state
            .registers
            .set_data_register(register_x, register_data_x);

        vm_state
            .registers
            .set_data_register(register_y, register_data_y);
    });

    test.execute(raw_instruction);

    test.assert(|vm_state| {
        assert_eq!(
            vm_state.registers.get_data_register(register_x),
            expected_result
        );
    })
}
