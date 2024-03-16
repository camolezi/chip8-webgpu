use chip8_webgpu::chip8::basic_types::Byte;

use super::*;

struct OperatorsTestParameters {
    opcode_first_nibble: Byte,
    opcode_last_nibble: Byte,

    expected_result: Byte,
}

/*
Register pre-set data
let register_data_x = 0b10001001;
let register_data_y = 0b00010101;
*/
fn run_operator_test(test_parameters: OperatorsTestParameters) {
    let mut test = InstructionTest::default();

    let OperatorsTestParameters {
        opcode_first_nibble,
        opcode_last_nibble,
        expected_result,
    } = test_parameters;

    let register_x = 0x5;
    let register_y = 0xa;

    let register_data_x = 0b10001001;
    let register_data_y = 0b00010101;

    let raw_instruction = create_raw_instruction(
        (opcode_first_nibble, register_x),
        (register_y, opcode_last_nibble),
    );

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

#[test]
fn or_register_test() {
    run_operator_test(OperatorsTestParameters {
        opcode_first_nibble: 0x8,
        opcode_last_nibble: 0x1,

        /*
        //0b10001001 | 0b00010101 = 0b10011101
         */
        expected_result: 0b10011101,
    })
}

#[test]
fn and_register_test() {
    run_operator_test(OperatorsTestParameters {
        opcode_first_nibble: 0x8,
        opcode_last_nibble: 0x2,

        /*
        //0b10001001 | 0b00010101 = 0b10011101
         */
        expected_result: 0b00000001,
    })
}

#[test]
fn xor_register_test() {
    run_operator_test(OperatorsTestParameters {
        opcode_first_nibble: 0x8,
        opcode_last_nibble: 0x3,

        /*
        0b10001001 ^ 0b00010101 = 0b10011100
        */
        expected_result: 0b10011100,
    })
}
